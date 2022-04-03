use crate::{framework::Fitness, tracking::Log};

mod custom;
pub use custom::{CustomState, CustomStateMap};

pub mod common;

#[derive(Default)]
pub struct StateTree {
    parent: Option<Box<StateTree>>,
    map: CustomStateMap,
}

impl StateTree {
    pub fn new_root() -> Self {
        let mut tree = StateTree {
            parent: None,
            map: CustomStateMap::new(),
        };
        common::insert_common_state(&mut tree);
        tree
    }

    pub fn with_substate(&mut self, fun: impl FnOnce(&mut StateTree)) {
        let mut substate: StateTree = Self {
            parent: Some(Box::new(std::mem::take(self))),
            map: CustomStateMap::new(),
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

    pub fn get<T: CustomState>(&self) -> &T {
        if self.map.has::<T>() {
            self.map.get::<T>()
        } else {
            self.parent().unwrap().get::<T>()
        }
    }

    pub fn get_mut<T: CustomState>(&mut self) -> &mut T {
        if self.map.has::<T>() {
            self.map.get_mut::<T>()
        } else {
            self.parent_mut().unwrap().get_mut::<T>()
        }
    }
}

/// Tracks various aspects of the current execution.
pub struct State {
    /// How many solutions have been evaluated.
    pub evaluations: u32,
    /// How many iterations have passed.
    pub iterations: u32,
    /// Percental progress towards termination.
    ///
    /// Value must be between `0.0` and `1.0`.
    pub progress: f64,
    /// Best fitness reached so far.
    pub best_so_far: Fitness,
    /// Custom state
    pub custom: CustomStateMap,
}

impl State {
    pub(crate) fn new() -> Self {
        State {
            evaluations: 0,
            iterations: 0,
            progress: 0.0,
            best_so_far: Fitness::try_from(f64::INFINITY).unwrap(),
            custom: CustomStateMap::new(),
        }
    }

    /// Logs an evaluation and increments [State::evaluations].
    pub(crate) fn log_evaluation(&mut self, logger: &mut Log, fitness: Fitness) {
        self.evaluations += 1;
        if fitness < self.best_so_far {
            self.best_so_far = fitness;
        }
        logger.log_evaluation(self, fitness.into());
    }

    /// Logs an iteration and increments [State::iterations].
    pub(crate) fn log_iteration(&mut self, logger: &mut Log) {
        self.iterations += 1;
        logger.log_iteration(self);
    }
}
