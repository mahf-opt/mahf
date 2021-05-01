use crate::{fitness::Fitness, tracking::Log};
use std::convert::TryFrom;

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
}

impl State {
    pub(crate) fn new() -> Self {
        State {
            evaluations: 0,
            iterations: 0,
            progress: 0.0,
            best_so_far: Fitness::try_from(f64::INFINITY).unwrap(),
        }
    }

    /// Logs an evaluation and increments [State::evaluations].
    pub(crate) fn log_evaluation(&mut self, logger: &mut Log, fitness: Fitness) {
        self.evaluations += 1;
        if fitness < self.best_so_far {
            self.best_so_far = fitness;
        }
        logger.log_evaluation(self.evaluations, fitness.into(), self.best_so_far.into());
    }

    /// Logs an iteration and increments [State::iterations].
    pub(crate) fn log_iteration(&mut self, logger: &mut Log) {
        self.iterations += 1;
        logger.log_iteration(self.iterations, self.best_so_far.into(), 0.0);
    }
}
