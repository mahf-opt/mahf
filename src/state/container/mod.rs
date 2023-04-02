#![doc = include_str!("../../../docs/state.md")]

use std::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use crate::{
    framework::{Individual, Random, SingleObjective},
    problems::{MultiObjectiveProblem, Problem, SingleObjectiveProblem},
    state::common,
    tracking::Log,
};

mod many;
use better_any::{Tid, TidAble};
pub use many::{MultiStateTuple, MutState};

mod map;
pub(crate) use map::StateMap;

/// A marker trait for custom state.
pub trait CustomState<'a>: Tid<'a> + Send {}

/// Container for storing and managing state.
pub struct State<'a, P> {
    parent: Option<Box<State<'a, P>>>,
    map: StateMap<'a>,
    _phantom: PhantomData<P>,
}

impl<P: Problem> Default for State<'_, P> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, P: Problem> State<'a, P> {
    /// Creates a new state container.
    ///
    /// Only needed for tests.
    pub fn new() -> Self {
        State {
            parent: None,
            map: StateMap::new(),
            _phantom: PhantomData,
        }
    }

    /// Runs a closure within a new scope.
    pub fn with_substate(&mut self, fun: impl FnOnce(&mut State<P>)) {
        let mut substate: State<P> = Self {
            parent: Some(Box::new(std::mem::take(self))),
            map: StateMap::new(),
            _phantom: PhantomData,
        };
        fun(&mut substate);
        *self = *substate.parent.unwrap();
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

    /// Tries to find an inner map containing T.
    fn find<T: CustomState<'a>>(&self) -> Option<&Self> {
        if self.map.has::<T>() {
            Some(self)
        } else {
            self.parent().and_then(Self::find::<T>)
        }
    }

    /// Tires to find an inner map containing T.
    fn find_mut<T: CustomState<'a>>(&mut self) -> Option<&mut Self> {
        if self.map.has::<T>() {
            Some(self)
        } else {
            self.parent_mut().and_then(Self::find_mut::<T>)
        }
    }

    /// Checks whether the state exists.
    pub fn has<T: CustomState<'a>>(&self) -> bool {
        self.find::<T>().is_some()
    }

    /// Panics if the state `T` does not exist, but is needed by the component `C`.
    ///
    /// This is the recommended way to ensure the state
    /// is available in [Component::initialize](crate::framework::components::Component::initialize).
    ///
    /// # Examples
    ///
    /// Basic usage:
    /// ```
    /// use mahf::prelude::*;
    /// use mahf::framework::components::Component;
    /// use mahf::state::CustomState;
    ///
    /// #[derive(better_any::Tid)]
    /// struct RequiredCustomState;
    /// impl CustomState<'_> for RequiredCustomState {}
    ///
    /// #[derive(Clone, serde::Serialize)]
    /// struct ExampleComponent;
    ///
    /// impl<P: problems::Problem> Component<P> for ExampleComponent {
    ///     fn initialize(&self, problem: &P, state: &mut State<P>) {
    ///         // Panics with an error message if `RequiredCustomState` is not present.
    ///         state.require::<Self, RequiredCustomState>();
    ///     }
    ///
    ///     fn execute(&self, problem: &P, state: &mut State<P>) {
    ///         unimplemented!()
    ///     }
    /// }
    /// ```
    #[track_caller]
    pub fn require<C, T: CustomState<'a>>(&self) {
        assert!(
            self.has::<T>(),
            "{} requires {} state",
            std::any::type_name::<C>(),
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
    pub fn holding<T>(&mut self, code: impl FnOnce(&mut T, &mut Self))
    where
        T: CustomState<'a> + TidAble<'a>,
    {
        let state_with_t = self.find_mut::<T>().unwrap();

        state_with_t.insert(Placeholder::<T>(PhantomData));
        let mut instance = state_with_t.take::<T>();
        code(&mut instance, self);

        let state_with_t = self.find_mut::<Placeholder<T>>().unwrap();
        state_with_t.insert(instance);
        state_with_t.take::<Placeholder<T>>();
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
    /// let mut state = State::new();
    /// state.insert(Random::testing());
    /// state.insert(Population::<BenchmarkFunction>::new());
    ///
    /// let mut mut_state = state.get_states_mut();
    /// let rng = mut_state.random_mut();
    /// let population = mut_state.population_stack_mut::<BenchmarkFunction>();
    ///
    /// // Do something with rng and population, or borrow additional types.
    /// ```
    pub fn get_states_mut<'b>(&'b mut self) -> MutState<'b, 'a, P> {
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
    /// let mut state = State::new();
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

macro_rules! impl_convenience_methods {
    ($lifetime:lifetime, $self_type:ty) => {
        /// Returns [Iterations](common::Iterations) state.
        pub fn iterations(self: $self_type) -> u32 {
            self.get_value::<common::Iterations>()
        }

        /// Returns [Evaluations](common::Evaluations) state.
        pub fn evaluations(self: $self_type) -> u32 {
            self.get_value::<common::Evaluations>()
        }

        /// Returns [Population](common::Populations) state.
        pub fn populations(self: $self_type) -> &$lifetime common::Populations<P> {
            self.get::<common::Populations<P>>()
        }

        /// Returns mutable [Population](common::Populations) state.
        pub fn populations_mut(&mut self) -> &$lifetime mut common::Populations<P> {
            self.get_mut::<common::Populations<P>>()
        }

        /// Returns mutable [Random](random::Random) state.
        pub fn random_mut(&mut self) -> &$lifetime mut Random {
            self.get_mut::<Random>()
        }

        /// Returns the [Log].
        pub fn log(self: $self_type) -> &$lifetime Log {
            self.get::<Log>()
        }
    };
}

impl<'a, P: Problem> State<'a, P> {
    // Uses '_ as 'self lifetime.
    // This has to match the lifetime bounds of [State::get].
    impl_convenience_methods!('_, &Self);
}

impl<'a, 's, P: Problem> MutState<'a, 's, P> {
    // Uses 'a as the internal [State]s lifetime.
    // This has to match the lifetime bounds of [MutState::get].
    impl_convenience_methods!('a, &mut Self);
}

macro_rules! impl_single_objective_convenience_methods {
    ($lifetime:lifetime, $self_type:ty) => {
        /// Returns [BestIndividual](common::BestIndividual) state.
        pub fn best_individual(self: $self_type) -> Option<&$lifetime Individual<P>> {
            self.get::<common::BestIndividual<P>>().as_ref()
        }

        /// Returns [BestIndividual](common::BestIndividual) state.
        pub fn best_individual_mut(&mut self) -> Option<&$lifetime mut Individual<P>> {
            self.get_mut::<common::BestIndividual<P>>().as_mut()
        }

        /// Returns the objective value of the [BestIndividual](common::BestIndividual).
        pub fn best_objective_value(self: $self_type) -> Option<&$lifetime SingleObjective> {
            self.best_individual().map(|i| i.objective())
        }
    };
}

impl<'a, P: SingleObjectiveProblem> State<'a, P> {
    // Uses '_ as 'self lifetime.
    // This has to match the lifetime bounds of [State::get].
    impl_single_objective_convenience_methods!('_, &Self);
}

impl<'a, 's, P: SingleObjectiveProblem> MutState<'a, 's, P> {
    // Uses 'a as the internal [State]s lifetime.
    // This has to match the lifetime bounds of [MutState::get].
    impl_single_objective_convenience_methods!('a, &mut Self);
}

macro_rules! impl_multi_objective_convenience_methods {
    ($lifetime:lifetime, $self_type:ty) => {
        /// Returns [ParetoFront](common::ParetoFront) state.
        pub fn pareto_front(self: $self_type) -> &$lifetime common::ParetoFront<P> {
            self.get::<common::ParetoFront<P>>()
        }

        /// Returns [ParetoFront](common::ParetoFront) state.
        pub fn pareto_front_mut(&mut self) -> &$lifetime mut common::ParetoFront<P> {
            self.get_mut::<common::ParetoFront<P>>()
        }
    };
}

impl<'a, P: MultiObjectiveProblem> State<'a, P> {
    // Uses '_ as 'self lifetime.
    // This has to match the lifetime bounds of [State::get].
    impl_multi_objective_convenience_methods!('_, &Self);
}

impl<'a, 's, P: MultiObjectiveProblem> MutState<'a, 's, P> {
    // Uses 'a as the internal [State]s lifetime.
    // This has to match the lifetime bounds of [MutState::get].
    impl_multi_objective_convenience_methods!('a, &mut Self);
}

#[derive(Tid)]
struct Placeholder<T>(PhantomData<T>);
impl<'a, T: CustomState<'a> + TidAble<'a>> CustomState<'a> for Placeholder<T> {}
