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
    fn terminate(&self, state: &mut State) -> bool {
        state.progress = state.iterations as f64 / self.max_iterations as f64;
        state.iterations >= self.max_iterations
    }
}
#[cfg(test)]
mod fixed_iterations {
    use super::*;

    #[test]
    fn terminates() {
        let mut state = State::new();
        let comp = FixedIterations {
            max_iterations: 200,
        };
        state.iterations = 100;
        assert_eq!(comp.terminate(&mut state), false);
        state.iterations = 200;
        assert_eq!(comp.terminate(&mut state), true);
    }

    #[test]
    fn updates_progress() {
        let mut state = State::new();
        let comp = FixedIterations {
            max_iterations: 200,
        };
        state.iterations = 100;
        comp.terminate(&mut state);
        float_eq::assert_float_eq!(state.progress, 0.5, ulps <= 2);
        state.iterations = 200;
        comp.terminate(&mut state);
        float_eq::assert_float_eq!(state.progress, 1.0, ulps <= 2);
    }
}

/// Terminates after a fixed number of evaluations.
///
/// Supports measuring time to completion.
#[derive(Serialize, Deserialize)]
pub struct FixedEvaluations {
    /// Maximum number of evaluations.
    pub max_evaluations: u32,
}
impl Termination for FixedEvaluations {
    fn terminate(&self, state: &mut State) -> bool {
        state.progress = state.evaluations as f64 / self.max_evaluations as f64;
        state.evaluations >= self.max_evaluations
    }
}
#[cfg(test)]
mod fixed_evaluations {
    use super::*;

    #[test]
    fn terminates() {
        let mut state = State::new();
        let comp = FixedEvaluations {
            max_evaluations: 200,
        };
        state.evaluations = 100;
        assert_eq!(comp.terminate(&mut state), false);
        state.evaluations = 200;
        assert_eq!(comp.terminate(&mut state), true);
    }

    #[test]
    fn updates_progress() {
        let mut state = State::new();
        let comp = FixedEvaluations {
            max_evaluations: 200,
        };
        state.evaluations = 100;
        comp.terminate(&mut state);
        float_eq::assert_float_eq!(state.progress, 0.5, ulps <= 2);
        state.evaluations = 200;
        comp.terminate(&mut state);
        float_eq::assert_float_eq!(state.progress, 1.0, ulps <= 2);
    }
}
