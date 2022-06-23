use std::ops::{Deref, DerefMut};

use crate::{
    framework::{Fitness, Individual},
    random,
    tracking::log::Logger,
};

mod many;
use many::MultiStateTuple;
pub use many::MutState;

mod map;
use map::AsAny;
pub(crate) use map::StateMap;

pub mod common;

/// Makes custom state trackable.
pub trait CustomState: AsAny {
    fn auto_logger(&self) -> Option<Logger> {
        None
    }
}

#[derive(Default)]
pub struct State {
    parent: Option<Box<State>>,
    map: StateMap,
}

impl State {
    pub fn new_root() -> Self {
        State {
            parent: None,
            map: StateMap::new(),
        }
    }

    pub fn with_substate(&mut self, fun: impl FnOnce(&mut State)) {
        let mut substate: State = Self {
            parent: Some(Box::new(std::mem::take(self))),
            map: StateMap::new(),
        };
        fun(&mut substate);
        *self = *substate.parent.unwrap()
    }

    pub fn parent(&self) -> Option<&Self> {
        self.parent.as_deref()
    }

    pub fn parent_mut(&mut self) -> Option<&mut Self> {
        self.parent.as_deref_mut()
    }

    pub fn insert<T: CustomState>(&mut self, state: T) {
        self.map.insert(state);
    }

    pub fn has<T: CustomState>(&self) -> bool {
        self.map.has::<T>() || self.parent().map(|p| p.has::<T>()).unwrap_or_default()
    }

    #[track_caller]
    pub fn require<T: CustomState>(&self) {
        assert!(
            self.has::<T>(),
            "operator requires {} state",
            std::any::type_name::<T>()
        );
    }

    pub fn get<T: CustomState>(&self) -> &T {
        if self.map.has::<T>() {
            self.map.get::<T>()
        } else {
            self.parent().unwrap().get::<T>()
        }
    }

    pub fn get_or_insert_default<T: CustomState + Default>(&mut self) -> &mut T {
        self.map.get_or_insert_default()
    }

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
    /// use mahf::{framework::{State, common_state::Population}, random::Random};
    /// let mut state = State::new_root();
    /// state.insert(Random::testing());
    /// state.insert(Population::new());
    ///
    /// let mut mut_state = state.get_states_mut();
    /// let rng = mut_state.random_mut();
    /// let population = mut_state.population_stack_mut();
    ///
    /// // Do something with rng and population, or borrow additional types.
    /// ```
    pub fn get_states_mut(&mut self) -> MutState<'_> {
        MutState::new(self)
    }

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
    /// use mahf::{framework::{State, common_state::Population}, random::Random};
    /// let mut state = State::new_root();
    /// state.insert(Random::testing());
    /// state.insert(Population::new());
    ///
    /// let (rng, population) = state.get_multiple_mut::<(Random, Population)>();
    ///
    /// // Do something with rng and population.
    /// ```
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

        /// Returns [BestFitness](common::BestFitness) state.
        pub fn best_fitness(self: $t) -> Fitness {
            self.get_value::<common::BestFitness>()
        }

        /// Returns [BestIndividual](common::BestIndividual) state.
        pub fn best_individual(self: $t) -> Option<&Individual> {
            self.get::<common::BestIndividual>().as_ref()
        }

        /// Returns [Population](common::Population) state.
        pub fn population_stack(self: $t) -> &$l common::Population {
            self.get::<common::Population>()
        }

        /// Returns mutable [Population](common::Population) state.
        pub fn population_stack_mut(&mut self) -> &$l mut common::Population {
            self.get_mut::<common::Population>()
        }

        /// Returns mutable [Random](random::Random) state.
        pub fn random_mut(&mut self) -> &$l mut random::Random {
            self.get_mut::<random::Random>()
        }
    };
}

impl_convenience_functions!(State);
impl_convenience_functions!(MutState, 'a, &mut Self);
