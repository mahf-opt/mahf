//! Termination methods

use crate::framework::{components::*, State};
use crate::operators::custom_state::FitnessImprovementState;
use crate::problems::{HasKnownOptimum, Problem};
use serde::{Deserialize, Serialize};

/// Only a placeholder. Replace this with something else.
///
/// See [operators::termination][crate::operators::termination] for possible criteria.
#[derive(Serialize, Deserialize)]
pub struct Undefined;
impl Undefined {
    pub fn new<P>() -> Box<dyn Termination<P>>
    where
        P: Problem,
    {
        Box::new(Self)
    }
}
impl<P> Termination<P> for Undefined
where
    P: Problem,
{
    fn terminate(&self, _state: &mut State, _problem: &P) -> bool {
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
    pub fn new<P>(max_iterations: u32) -> Box<dyn Termination<P>>
    where
        P: Problem,
    {
        Box::new(Self { max_iterations })
    }
}
impl<P> Termination<P> for FixedIterations
where
    P: Problem,
{
    fn terminate(&self, state: &mut State, _problem: &P) -> bool {
        state.progress = state.iterations as f64 / self.max_iterations as f64;
        state.iterations >= self.max_iterations
    }
}
#[cfg(test)]
mod fixed_iterations {
    use super::*;
    use crate::problems::bmf::BenchmarkFunction;

    #[test]
    fn terminates() {
        let problem = BenchmarkFunction::sphere(3);
        let mut state = State::new();
        let comp = FixedIterations {
            max_iterations: 200,
        };
        state.iterations = 100;
        assert!(!comp.terminate(&mut state, &problem));
        state.iterations = 200;
        assert!(comp.terminate(&mut state, &problem));
    }

    #[test]
    fn updates_progress() {
        let problem = BenchmarkFunction::sphere(3);
        let mut state = State::new();
        let comp = FixedIterations {
            max_iterations: 200,
        };
        state.iterations = 100;
        comp.terminate(&mut state, &problem);
        float_eq::assert_float_eq!(state.progress, 0.5, ulps <= 2);
        state.iterations = 200;
        comp.terminate(&mut state, &problem);
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
impl<P> Termination<P> for FixedEvaluations
where
    P: Problem,
{
    fn terminate(&self, state: &mut State, _problem: &P) -> bool {
        state.progress = state.evaluations as f64 / self.max_evaluations as f64;
        state.evaluations >= self.max_evaluations
    }
}
#[cfg(test)]
mod fixed_evaluations {
    use super::*;
    use crate::problems::bmf::BenchmarkFunction;

    #[test]
    fn terminates() {
        let problem = BenchmarkFunction::sphere(3);
        let mut state = State::new();
        let comp = FixedEvaluations {
            max_evaluations: 200,
        };
        state.evaluations = 100;
        assert!(!comp.terminate(&mut state, &problem));
        state.evaluations = 200;
        assert!(comp.terminate(&mut state, &problem));
    }

    #[test]
    fn updates_progress() {
        let problem = BenchmarkFunction::sphere(3);
        let mut state = State::new();
        let comp = FixedEvaluations {
            max_evaluations: 200,
        };
        state.evaluations = 100;
        comp.terminate(&mut state, &problem);
        float_eq::assert_float_eq!(state.progress, 0.5, ulps <= 2);
        state.evaluations = 200;
        comp.terminate(&mut state, &problem);
        float_eq::assert_float_eq!(state.progress, 1.0, ulps <= 2);
    }
}

/// Terminates after distance to the known optimum is less than specified value.
///
/// Progress is unknown, as optimizer should not have information on optimum.
#[derive(Serialize, Deserialize)]
pub struct DistanceToOpt {
    /// Distance to known optimum.
    pub distance: f64,
    /// Known optimum.
    pub optimum: f64,
}
impl<P: HasKnownOptimum> Termination<P> for DistanceToOpt
where
    P: Problem,
{
    fn terminate(&self, state: &mut State, problem: &P) -> bool {
        state.best_so_far.into() < problem.known_optimum() + self.distance
    }
}
#[cfg(test)]
mod distance_to_opt {
    use super::*;
    use crate::framework::Fitness;
    use crate::problems::bmf::BenchmarkFunction;

    #[test]
    fn terminates() {
        let problem = BenchmarkFunction::sphere(3);
        let mut state = State::new();
        let comp = DistanceToOpt {
            distance: 0.1,
            optimum: 0.0,
        };
        state.best_so_far = Fitness::try_from(2.0).unwrap();
        assert!(!comp.terminate(&mut state, &problem));
        state.best_so_far = Fitness::try_from(0.05).unwrap();
        assert!(comp.terminate(&mut state, &problem));
    }
}

/// Terminates after a specified number of steps (iterations) did not yield any improvement.
///
/// Progress is unknown, as steps depend on current performance of optimizer.
#[derive(Serialize, Deserialize)]
pub struct StepsWithoutImprovement {
    /// Number of steps without improvement.
    pub steps: usize,
}
impl<P> Termination<P> for StepsWithoutImprovement
where
    P: Problem,
{
    fn terminate(&self, state: &mut State, _problem: &P) -> bool {
        if !state.custom.has::<FitnessImprovementState>() {
            state.custom.insert(FitnessImprovementState {
                current_steps: 0,
                current_fitness: state.best_so_far.into(),
            });
        }
        let termination_state = state.custom.get_mut::<FitnessImprovementState>();
        let error_margin = f64::EPSILON;
        if (state.best_so_far.into() - termination_state.current_fitness).abs() < error_margin {
            termination_state.current_steps += 1;
        } else {
            termination_state.current_fitness = state.best_so_far.into();
            termination_state.current_steps = 0;
        }
        termination_state.current_steps >= self.steps
    }
}
#[cfg(test)]
mod steps_without_improvement {
    use super::*;
    use crate::framework::Fitness;
    use crate::problems::bmf::BenchmarkFunction;

    #[test]
    fn terminates() {
        let problem = BenchmarkFunction::sphere(3);
        let mut state = State::new();
        let comp = StepsWithoutImprovement { steps: 20 };
        state.custom.insert(FitnessImprovementState {
            current_steps: 0,
            current_fitness: 0.5,
        });
        state.best_so_far = Fitness::try_from(0.5).unwrap();
        state.iterations = 10;
        assert!(!comp.terminate(&mut state, &problem));
        state.best_so_far = Fitness::try_from(0.5).unwrap();
        let test_state = state.custom.get_mut::<FitnessImprovementState>();
        test_state.current_steps = 20;
        assert!(comp.terminate(&mut state, &problem));
    }
}
