#![doc = include_str!("../../../docs/state.md")]

use std::ops::{Deref, DerefMut};

use crate::{
    framework::{Individual, Random, SingleObjective},
    problems::{MultiObjectiveProblem, Problem, SingleObjectiveProblem},
    state::common,
    tracking::Log,
};

mod many;
use better_any::Tid;
pub use many::{MultiStateTuple, MutState};

mod map;
pub(crate) use map::StateMap;

/// A marker trait for custom state.
pub trait CustomState<'a>: Tid<'a> + Send {}

/// Container for storing and managing state.
#[derive(Default)]
pub struct State<'a> {
    parent: Option<Box<State<'a>>>,
    map: StateMap<'a>,
}

impl<'a> State<'a> {
    /// Creates a new state container.
    ///
    /// Only needed for tests.
    pub fn new_root() -> Self {
        State {
            parent: None,
            map: StateMap::new(),
        }
    }

    /// Runs a closure within a new scope.
    pub fn with_substate(&mut self, fun: impl FnOnce(&mut State)) {
        let mut substate: State = Self {
            parent: Some(Box::new(std::mem::take(self))),
            map: StateMap::new(),
        };
        fun(&mut substate);
        *self = *substate.parent.unwrap()
    }

    /// Returns the parent state.
    pub fn parent(&self) -> Option<&Self> {
        self.parent.as_deref()
    }

    /// Returns the mutable parent state.
    pub fn parent_mut(&mut self) -> Option<&mut Self> {
        self.parent.as_deref_mut()
    }

    /// Inserts new state, overriding existing state.
    pub fn insert<T: CustomState<'a>>(&mut self, state: T) {
        self.map.insert(state);
    }

    /// Tires to find an inner map containing T.
    fn find<T: CustomState<'a>>(&self) -> Option<&Self> {
        if self.map.has::<T>() {
            Some(self)
        } else {
            self.parent()
        }
    }

    /// Tires to find an inner map containing T.
    fn find_mut<T: CustomState<'a>>(&mut self) -> Option<&mut Self> {
        if self.map.has::<T>() {
            Some(self)
        } else {
            self.parent_mut()
        }
    }

    /// Checks whether the state exists.
    pub fn has<T: CustomState<'a>>(&self) -> bool {
        self.find::<T>().is_some()
    }

    /// Panics if the state does not exist.
    ///
    /// This is the recommended way to ensure the state
    /// is available in [Component::initialize](crate::framework::components::Component::initialize).
    #[track_caller]
    pub fn require<T: CustomState<'a>>(&self) {
        assert!(
            self.has::<T>(),
            "operator requires {} state",
            std::any::type_name::<T>()
        );
    }

    /// Removes `T` from state and returns it.
    ///
    /// If `T` should only be removed temporarily, consider using [State::holding] instead.
    #[track_caller]
    pub fn take<T: CustomState<'a>>(&mut self) -> T {
        self.find_mut::<T>().unwrap().map.take::<T>()
    }

    /// Access `T` mutably without borrowing the state.
    ///
    /// To make this possible, `T` will be removed temporarily.
    /// This should only be used, if the state has to be passed to another function,
    /// whilst borrowing from it.
    #[track_caller]
    pub fn holding<T: CustomState<'a>>(&mut self, code: impl FnOnce(&mut T, &mut Self)) {
        let state_with_t = self.find_mut::<T>().unwrap() as *mut Self;

        let mut instance = self.take::<T>();
        code(&mut instance, self);

        // Insert the state where it was before.
        // This is save, because inner states can not be removed.
        unsafe { state_with_t.as_mut().unwrap().insert(instance) };
    }

    /// Returns the state.
    ///
    /// # Panics
    /// If the state does not exist.
    #[track_caller]
    pub fn get<T: CustomState<'a>>(&self) -> &T {
        self.find::<T>().unwrap().map.get::<T>()
    }

    /// Returns the state or inserts its default.
    pub fn get_or_insert_default<T: CustomState<'a> + Default>(&mut self) -> &mut T {
        self.map.get_or_insert_default()
    }

    /// Returns the states inner value.
    ///
    /// Requires the state to implement [Deref].
    ///
    /// # Panics
    /// If the state does not exist.
    #[track_caller]
    pub fn get_value<T>(&self) -> T::Target
    where
        T: CustomState<'a> + Deref,
        T::Target: Sized + Copy,
    {
        *self.find::<T>().unwrap().map.get::<T>().deref()
    }

    /// Returns the state mutably.
    ///
    /// # Panics
    /// If the state does not exist.
    #[track_caller]
    pub fn get_mut<T: CustomState<'a>>(&mut self) -> &mut T {
        self.find_mut::<T>().unwrap().map.get_mut::<T>()
    }

    /// Allows borrowing an arbitrary number of [CustomState]'s mutable at the same time.
    /// For additional information and limitations, see [MutState].
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use mahf::{state::{State, common::Population}, framework::Random, problems::bmf::BenchmarkFunction};
    /// let problem = BenchmarkFunction::sphere(3);
    /// let mut state = State::new_root();
    /// state.insert(Random::testing());
    /// state.insert(Population::<BenchmarkFunction>::new());
    ///
    /// let mut mut_state = state.get_states_mut();
    /// let rng = mut_state.random_mut();
    /// let population = mut_state.population_stack_mut::<BenchmarkFunction>();
    ///
    /// // Do something with rng and population, or borrow additional types.
    /// ```
    pub fn get_states_mut<'b>(&'b mut self) -> MutState<'b, 'a> {
        MutState::new(self)
    }

    /// Updates the states inner value.
    ///
    /// Requires the state to implement [DerefMut].
    ///
    /// # Panics
    /// If the state does not exist.
    #[track_caller]
    pub fn set_value<T>(&mut self, value: T::Target)
    where
        T: CustomState<'a> + DerefMut,
        T::Target: Sized,
    {
        *self.get_value_mut::<T>() = value;
    }

    /// Returns the states inner value mutably.
    ///
    /// Requires the state to implement [Deref].
    ///
    /// # Panics
    /// If the state does not exist.
    #[track_caller]
    pub fn get_value_mut<T>(&mut self) -> &mut T::Target
    where
        T: CustomState<'a> + DerefMut,
        T::Target: Sized,
    {
        self.find_mut::<T>().unwrap().map.get_mut::<T>().deref_mut()
    }

    /// Allows borrowing up to eight [CustomState]'s mutable at the same time, given they are different.
    /// The types are supplied as tuples.
    ///
    /// # Panics
    ///
    /// Panics on type duplicates in the tuple.
    ///
    /// # Examples
    ///
    ///  Basic usage:
    ///
    /// ```
    /// use mahf::{state::{State, common::Population}, framework::Random, problems::bmf::BenchmarkFunction};
    /// let problem = BenchmarkFunction::sphere(3);
    /// let mut state = State::new_root();
    /// state.insert(Random::testing());
    /// state.insert(Population::<BenchmarkFunction>::new());
    ///
    /// let (rng, population) = state.get_multiple_mut::<(Random, Population<BenchmarkFunction>)>();
    ///
    /// // Do something with rng and the population.
    /// ```
    #[track_caller]
    pub fn get_multiple_mut<'b, T: MultiStateTuple<'b, 'a>>(&'b mut self) -> T::References {
        T::fetch(self)
    }
}

/// Convenience functions for often required state.
///
/// If some state does not exist, the function will panic.
macro_rules! impl_convenience_functions {
    ($l:lifetime, $t:ty) => {
        /// Returns [Iterations](common::Iterations) state.
        pub fn iterations(self: $t) -> u32 {
            self.get_value::<common::Iterations>()
        }

        /// Returns [Evaluations](common::Evaluations) state.
        pub fn evaluations(self: $t) -> u32 {
            self.get_value::<common::Evaluations>()
        }

        /// Returns [BestIndividual](common::BestIndividual) state.
        pub fn best_individual<P: SingleObjectiveProblem>(self: $t) -> Option<&$l Individual<P>> {
            self.get::<common::BestIndividual<P>>().as_ref()
        }

        /// Returns the objective value of the [BestIndividual](common::BestIndividual).
        pub fn best_objective_value<P: SingleObjectiveProblem>(
            self: $t,
        ) -> Option<&SingleObjective> {
            self.best_individual::<P>().map(|i| i.objective())
        }

        /// Returns [ParetoFront](common::ParetoFront) state.
        pub fn pareto_front<P: MultiObjectiveProblem>(self: $t) -> &$l common::ParetoFront<P> {
            self.get::<common::ParetoFront<P>>()
        }

        /// Returns [Population](common::Population) state.
        pub fn population_stack<P: Problem>(self: $t) -> &$l common::Population<P> {
            self.get::<common::Population<P>>()
        }

        /// Returns mutable [Population](common::Population) state.
        pub fn population_stack_mut<P: Problem>(&mut self) -> &$l mut common::Population<P> {
            self.get_mut::<common::Population<P>>()
        }

        /// Returns mutable [Random](random::Random) state.
        pub fn random_mut(&mut self) -> &$l mut Random {
            self.get_mut::<Random>()
        }

        /// Returns the [Log].
        pub fn log(self: $t) -> &$l Log {
            self.get::<Log>()
        }
    };
}

impl<'a> State<'a> {
    // Uses '_ as 'self lifetime.
    // This has to match the lifetime bounds of [State::get].
    impl_convenience_functions!('_, &Self);
}

impl<'a, 's> MutState<'a, 's> {
    // Uses 'a as the internal [State]s lifetime.
    // This has to match the lifetime bounds of [MutState::get].
    impl_convenience_functions!('a, &mut Self);
}
