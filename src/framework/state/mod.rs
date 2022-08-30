#![doc = include_str!("../../../docs/state.md")]

use std::ops::{Deref, DerefMut};

use crate::{
    framework::{random, Individual, SingleObjective},
    problems::{MultiObjectiveProblem, Problem, SingleObjectiveProblem},
    tracking::Log,
};

mod many;
pub use many::{MultiStateTuple, MutState};

mod map;
use map::AsAny;
pub(crate) use map::StateMap;

pub mod common;

/// A marker trait for custom state.
pub trait CustomState: AsAny {}

/// Container for storing and managing state.
#[derive(Default)]
pub struct State {
    parent: Option<Box<State>>,
    map: StateMap,
}

impl State {
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
    pub fn insert<T: CustomState>(&mut self, state: T) {
        self.map.insert(state);
    }

    /// Checks whether the state exists.
    pub fn has<T: CustomState>(&self) -> bool {
        self.map.has::<T>() || self.parent().map(|p| p.has::<T>()).unwrap_or_default()
    }

    /// Panics if the state does not exist.
    ///
    /// This is the recommended way to ensure the state
    /// is available in [Component::initialize](crate::framework::components::Component::initialize).
    #[track_caller]
    pub fn require<T: CustomState>(&self) {
        assert!(
            self.has::<T>(),
            "operator requires {} state",
            std::any::type_name::<T>()
        );
    }

    /// Returns the state.
    ///
    /// # Panics
    /// If the state does not exist.
    #[track_caller]
    pub fn get<T: CustomState>(&self) -> &T {
        if self.map.has::<T>() {
            self.map.get::<T>()
        } else {
            self.parent().unwrap().get::<T>()
        }
    }

    /// Returns the state or inserts its default.
    pub fn get_or_insert_default<T: CustomState + Default>(&mut self) -> &mut T {
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
        T: CustomState + Deref,
        T::Target: Sized + Copy,
    {
        if self.map.has::<T>() {
            *self.map.get::<T>().deref()
        } else {
            *self.parent().unwrap().get::<T>().deref()
        }
    }

    /// Returns the state mutably.
    ///
    /// # Panics
    /// If the state does not exist.
    #[track_caller]
    pub fn get_mut<T: CustomState>(&mut self) -> &mut T {
        if self.map.has::<T>() {
            self.map.get_mut::<T>()
        } else {
            self.parent_mut().unwrap().get_mut::<T>()
        }
    }

    /// Allows borrowing an arbitrary number of [CustomState]'s mutable at the same time.
    /// For additional information and limitations, see [MutState].
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use mahf::{framework::{state::{State, common::Population}, Random}, problems::bmf::BenchmarkFunction};
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
    pub fn get_states_mut(&mut self) -> MutState<'_> {
        MutState::new(self)
    }

    /// Updates the states inner value.
    ///
    /// Requires the state to implement [DerefMut].
    ///
    /// # Panics
    /// If the state does not exist.
    pub fn set_value<T>(&mut self, value: T::Target)
    where
        T: CustomState + DerefMut,
        T::Target: Sized,
    {
        if self.map.has::<T>() {
            *self.map.get_mut::<T>().deref_mut() = value;
        } else {
            *self.parent_mut().unwrap().get_mut::<T>().deref_mut() = value;
        }
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
        T: CustomState + DerefMut,
        T::Target: Sized,
    {
        if self.map.has::<T>() {
            self.map.get_mut::<T>().deref_mut()
        } else {
            self.parent_mut().unwrap().get_mut::<T>().deref_mut()
        }
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
    /// use mahf::{framework::{state::{State, common::Population}, Random}, problems::bmf::BenchmarkFunction};
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
    pub fn get_multiple_mut<'a, T: MultiStateTuple<'a>>(&'a mut self) -> T::References {
        T::fetch(self)
    }
}

/// Convenience functions for often required state.
///
/// If some state does not exist, the function will panic.
macro_rules! impl_convenience_functions {
    ($item:ident, $l:lifetime, $t:ty) => {
        impl<$l> $item<$l> {
            impl_convenience_functions!($l, $t);
        }
    };
    ($item:ident) => {
        impl $item {
            impl_convenience_functions!('_, &Self);
        }
    };
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
        pub fn best_individual<P: SingleObjectiveProblem>(self: $t) -> Option<&Individual<P>> {
            self.get::<common::BestIndividual<P>>().as_ref()
        }

        /// Returns the objective value of the [BestIndividual](common::BestIndividual).
        pub fn best_objective_value<P: SingleObjectiveProblem>(self: $t) -> Option<&SingleObjective> {
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
        pub fn random_mut(&mut self) -> &$l mut random::Random {
            self.get_mut::<random::Random>()
        }

        /// Returns the [Log].
        pub fn log(self: $t) -> &$l Log {
            self.get::<Log>()
        }
    };
}

impl_convenience_functions!(State);
impl_convenience_functions!(MutState, 'a, &mut Self);
