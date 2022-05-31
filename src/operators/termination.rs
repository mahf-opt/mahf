//! Termination methods

use crate::{
    framework::{
        common_state::{BestFitness, Evaluations, Iterations, Progress},
        components::Condition,
        legacy::components::*,
        State,
    },
    operators::custom_state::FitnessImprovementState,
    problems::{HasKnownOptimum, HasKnownTarget, Problem},
};
use serde::{Deserialize, Serialize};

/// Only a placeholder. Replace this with something else.
///
/// See [operators::termination][crate::operators::termination] for possible criteria.
#[derive(Serialize, Deserialize)]
pub struct Undefined;
impl Undefined {
    pub fn new<P>() -> Box<dyn Condition<P>>
    where
        P: Problem,
    {
        Box::new(Terminator(Self))
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

#[derive(Serialize)]
#[serde(bound = "")]
pub struct And<P: Problem>(Vec<Box<dyn Condition<P>>>);
impl<P: Problem + 'static> And<P> {
    pub fn new(terminators: Vec<Box<dyn Condition<P>>>) -> Box<dyn Condition<P>> {
        Box::new(Self(terminators))
    }
}
impl<P> Condition<P> for And<P>
where
    P: Problem + 'static,
{
    fn initialize(&self, problem: &P, state: &mut State) {
        for condition in &self.0 {
            condition.initialize(problem, state);
        }
    }
    fn evaluate(&self, problem: &P, state: &mut State) -> bool {
        self.0
            .iter()
            .all(|condition| condition.evaluate(problem, state))
    }
}

#[derive(Serialize, Deserialize)]
pub struct TargetHit;
impl TargetHit {
    pub fn new<P>() -> Box<dyn Condition<P>>
    where
        P: Problem + HasKnownTarget,
    {
        Box::new(Terminator(Self))
    }
}
impl<P> Termination<P> for TargetHit
where
    P: Problem + HasKnownTarget,
{
    fn terminate(&self, state: &mut State, problem: &P) -> bool {
        problem.target_hit(state.best_fitness())
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
    pub fn new<P: Problem>(max_iterations: u32) -> Box<dyn Condition<P>>
    where
        P: Problem,
    {
        Box::new(Self { max_iterations })
    }
}
impl<P> Condition<P> for FixedIterations
where
    P: Problem,
{
    fn initialize(&self, _problem: &P, state: &mut State) {
        state.insert(Iterations(0));
        state.insert(Progress(0.0));
    }

    fn evaluate(&self, _problem: &P, state: &mut State) -> bool {
        let iterations = state.get_value::<Iterations>();

        state.set_value::<Iterations>(iterations + 1);
        state.set_value::<Progress>(iterations as f64 / self.max_iterations as f64);

        iterations < self.max_iterations
    }
}
#[cfg(test)]
mod fixed_iterations {
    use super::*;
    use crate::{framework::common_state, problems::bmf::BenchmarkFunction};

    #[test]
    fn terminates() {
        let problem = BenchmarkFunction::sphere(3);
        let mut state = State::new_root();
        common_state::default(&mut state);
        let comp = FixedIterations {
            max_iterations: 200,
        };
        state.set_value::<Iterations>(100);
        assert!(comp.evaluate(&problem, &mut state));
        state.set_value::<Iterations>(200);
        assert!(!comp.evaluate(&problem, &mut state));
    }

    #[test]
    fn updates_progress() {
        let problem = BenchmarkFunction::sphere(3);
        let mut state = State::new_root();
        common_state::default(&mut state);
        let comp = FixedIterations {
            max_iterations: 200,
        };
        state.set_value::<Iterations>(100);
        comp.evaluate(&problem, &mut state);
        float_eq::assert_float_eq!(state.get_value::<Progress>(), 0.5, ulps <= 2);
        state.set_value::<Iterations>(200);
        comp.evaluate(&problem, &mut state);
        float_eq::assert_float_eq!(state.get_value::<Progress>(), 1.0, ulps <= 2);
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
        let evaluations = state.get_value::<Evaluations>();
        state.set_value::<Progress>(evaluations as f64 / self.max_evaluations as f64);
        evaluations >= self.max_evaluations
    }
}
#[cfg(test)]
mod fixed_evaluations {
    use super::*;
    use crate::{framework::common_state, problems::bmf::BenchmarkFunction};

    #[test]
    fn terminates() {
        let problem = BenchmarkFunction::sphere(3);
        let mut state = State::new_root();
        common_state::default(&mut state);
        let comp = FixedEvaluations {
            max_evaluations: 200,
        };
        state.set_value::<Evaluations>(100);
        assert!(!comp.terminate(&mut state, &problem));
        state.set_value::<Evaluations>(200);
        assert!(comp.terminate(&mut state, &problem));
    }

    #[test]
    fn updates_progress() {
        let problem = BenchmarkFunction::sphere(3);
        let mut state = State::new_root();
        common_state::default(&mut state);
        let comp = FixedEvaluations {
            max_evaluations: 200,
        };
        state.set_value::<Evaluations>(100);
        comp.terminate(&mut state, &problem);
        float_eq::assert_float_eq!(state.get_value::<Progress>(), 0.5, ulps <= 2);
        state.set_value::<Evaluations>(200);
        comp.terminate(&mut state, &problem);
        float_eq::assert_float_eq!(state.get_value::<Progress>(), 1.0, ulps <= 2);
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
        state.get_value::<BestFitness>().into() < problem.known_optimum() + self.distance
    }
}
#[cfg(test)]
mod distance_to_opt {
    use super::*;
    use crate::{
        framework::{common_state, Fitness},
        problems::bmf::BenchmarkFunction,
    };

    #[test]
    fn terminates() {
        let problem = BenchmarkFunction::sphere(3);
        let mut state = State::new_root();
        common_state::default(&mut state);
        let comp = DistanceToOpt {
            distance: 0.1,
            optimum: 0.0,
        };
        state.set_value::<BestFitness>(Fitness::try_from(2.0).unwrap());
        assert!(!comp.terminate(&mut state, &problem));
        state.set_value::<BestFitness>(Fitness::try_from(0.05).unwrap());
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
        let current_fitness = state.get_value::<BestFitness>().into();
        if !state.has::<FitnessImprovementState>() {
            state.insert(FitnessImprovementState {
                current_steps: 0,
                current_fitness,
            });
        }
        let termination_state = state.get_mut::<FitnessImprovementState>();
        let error_margin = f64::EPSILON;
        if (current_fitness - termination_state.current_fitness).abs() < error_margin {
            termination_state.current_steps += 1;
        } else {
            termination_state.current_fitness = current_fitness;
            termination_state.current_steps = 0;
        }
        termination_state.current_steps >= self.steps
    }
}
#[cfg(test)]
mod steps_without_improvement {
    use super::*;
    use crate::{
        framework::{common_state, Fitness},
        problems::bmf::BenchmarkFunction,
    };

    #[test]
    fn terminates() {
        let problem = BenchmarkFunction::sphere(3);
        let mut state = State::new_root();
        common_state::default(&mut state);
        let comp = StepsWithoutImprovement { steps: 20 };
        state.insert(FitnessImprovementState {
            current_steps: 0,
            current_fitness: 0.5,
        });
        state.set_value::<BestFitness>(Fitness::try_from(0.5).unwrap());
        state.set_value::<Iterations>(10);
        assert!(!comp.terminate(&mut state, &problem));
        state.set_value::<BestFitness>(Fitness::try_from(0.5).unwrap());
        let test_state = state.get_mut::<FitnessImprovementState>();
        test_state.current_steps = 20;
        assert!(comp.terminate(&mut state, &problem));
    }
}
