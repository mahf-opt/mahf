use crate::{
    framework::{Fitness, Individual},
    random,
    tracking::log::Logger,
};
use std::ops::{Deref, DerefMut};

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

    pub fn get_or_default<T: CustomState + Default>(&mut self) -> &mut T {
        self.map.get_or_default()
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
}

/// Convenience functions for often required state.
///
/// If some state does not exist, the function will panic.
///
/// The following functions are basically guaranteed to work:
/// - [best_fitness](State::best_fitness)
impl State {
    /// Returns [Iterations](common::Iterations) state.
    pub fn iterations(&self) -> u32 {
        self.get_value::<common::Iterations>()
    }

    /// Returns [Evaluations](common::Evaluations) state.
    pub fn evaluations(&self) -> u32 {
        self.get_value::<common::Evaluations>()
    }

    /// Returns [BestFitness](common::BestFitness) state.
    pub fn best_fitness(&self) -> Fitness {
        self.get_value::<common::BestFitness>()
    }

    /// Returns [BestIndividual](common::BestIndividual) state.
    pub fn best_individual(&self) -> &Individual {
        self.get::<common::BestIndividual>()
    }

    /// Returns [Population](common::Population) state.
    pub fn population_stack(&self) -> &common::Population {
        self.get::<common::Population>()
    }

    /// Returns mutable [Population](common::Population) state.
    pub fn population_stack_mut(&mut self) -> &mut common::Population {
        self.get_mut::<common::Population>()
    }

    /// Returns mutable [Random](random::Random) state.
    pub fn random_mut(&mut self) -> &mut random::Random {
        self.get_mut::<random::Random>()
    }
}
