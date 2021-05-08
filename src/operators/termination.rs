//! Termination methods

use crate::heuristic::{components::*, State};
use serde::{Deserialize, Serialize};

/// Terminates after a fixed number of iterations.
///
/// Supports measuring time to completion.
#[derive(Serialize, Deserialize)]
pub struct FixedIterations {
    /// Maximum number of iterations.
    pub max_iterations: u32,
}
impl Termination for FixedIterations {
    fn terminate(&mut self, state: &mut State) -> bool {
        state.progress = state.iterations as f64 / self.max_iterations as f64;
        state.iterations >= self.max_iterations
    }
}

/// Terminates after a fixed number of evaluations.
///
/// Supports measuring time to completion.
#[derive(Serialize, Deserialize)]
pub struct FixedEvaluationsTermination {
    /// Maximum number of evaluations.
    pub max_evaluations: u32,
}
impl Termination for FixedEvaluationsTermination {
    fn terminate(&mut self, state: &mut State) -> bool {
        state.progress = state.evaluations as f64 / self.max_evaluations as f64;
        state.evaluations >= self.max_evaluations
    }
}
