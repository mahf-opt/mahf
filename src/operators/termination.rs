//! Termination methods

use crate::framework::{components::*, State};
use serde::{Deserialize, Serialize};

/// Only a placeholder. Replace this with something else.
///
/// See [operators::termination][crate::operators::termination] for possible criteria.
#[derive(Serialize, Deserialize)]
pub struct Undefined;
impl Undefined {
    pub fn new() -> Box<dyn Termination> {
        Box::new(Self)
    }
}
impl Termination for Undefined {
    fn terminate(&self, _state: &mut State) -> bool {
        unimplemented!(concat!(
            "Heuristic with no termination criteria was run. ",
            "Please specify a termination criteria."
        ));
    }
}

/// Terminates after a fixed number of iterations.
///
/// Supports measuring time to completion.
#[derive(Serialize, Deserialize)]
pub struct FixedIterations {
    /// Maximum number of iterations.
    pub max_iterations: u32,
}
impl FixedIterations {
    pub fn new(max_iterations: u32) -> Box<dyn Termination> {
        Box::new(Self { max_iterations })
    }
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
        assert!(!comp.terminate(&mut state));
        state.iterations = 200;
        assert!(comp.terminate(&mut state));
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
        assert!(!comp.terminate(&mut state));
        state.evaluations = 200;
        assert!(comp.terminate(&mut state));
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
