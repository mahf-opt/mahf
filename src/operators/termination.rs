//! Termination methods

use crate::operators::state::custom_state::FitnessImprovementState;
use crate::{
    framework::{
        conditions::Condition,
        state::{
            common::{Evaluations, Iterations, Progress},
            State,
        },
    },
    problems::{HasKnownOptimum, HasKnownTarget, Problem, SingleObjectiveProblem},
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
        Box::new(Self)
    }
}
impl<P> Condition<P> for Undefined
where
    P: Problem,
{
    fn evaluate(&self, _problem: &P, _state: &mut State) -> bool {
        unimplemented!(concat!(
            "Heuristic with no termination criteria was run. ",
            "Please specify a termination criteria."
        ));
    }
}

#[derive(Serialize, Deserialize)]
pub struct TargetHit;
impl TargetHit {
    pub fn new<P>() -> Box<dyn Condition<P>>
    where
        P: SingleObjectiveProblem + HasKnownTarget,
    {
        Box::new(Self)
    }
}
impl<P> Condition<P> for TargetHit
where
    P: SingleObjectiveProblem + HasKnownTarget,
{
    fn evaluate(&self, problem: &P, state: &mut State) -> bool {
        if let Some(fitness) = state.best_objective_value::<P>() {
            !problem.target_hit(*fitness)
        } else {
            false
        }
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
        state.require::<Iterations>();
        state.insert(Progress(0.));
    }

    fn evaluate(&self, _problem: &P, state: &mut State) -> bool {
        let iterations = state.iterations();
        state.set_value::<Progress>(iterations as f64 / self.max_iterations as f64);

        iterations < self.max_iterations
    }
}
#[cfg(test)]
mod fixed_iterations {
    use super::*;
    use crate::testing::TestProblem;

    #[test]
    fn terminates() {
        let problem = TestProblem;
        let mut state = State::new_root();
        state.insert(Iterations(0));
        let comp = FixedIterations {
            max_iterations: 200,
        };
        comp.initialize(&problem, &mut state);
        state.set_value::<Iterations>(100);
        assert!(comp.evaluate(&problem, &mut state));
        state.set_value::<Iterations>(200);
        assert!(!comp.evaluate(&problem, &mut state));
    }

    #[test]
    fn updates_progress() {
        let problem = TestProblem;
        let mut state = State::new_root();
        state.insert(Iterations(0));
        let comp = FixedIterations {
            max_iterations: 200,
        };
        comp.initialize(&problem, &mut state);
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
impl FixedEvaluations {
    pub fn new<P: Problem>(max_evaluations: u32) -> Box<dyn Condition<P>>
    where
        P: Problem,
    {
        Box::new(Self { max_evaluations })
    }
}
impl<P> Condition<P> for FixedEvaluations
where
    P: Problem,
{
    fn initialize(&self, _problem: &P, state: &mut State) {
        state.require::<Evaluations>();
        state.insert(Progress(0.));
    }

    fn evaluate(&self, _problem: &P, state: &mut State) -> bool {
        let evaluations = state.evaluations();
        state.set_value::<Progress>(evaluations as f64 / self.max_evaluations as f64);
        evaluations < self.max_evaluations
    }
}
#[cfg(test)]
mod fixed_evaluations {
    use super::*;
    use crate::testing::*;

    #[test]
    fn terminates() {
        let problem = TestProblem;
        let mut state = State::new_root();
        state.insert(Evaluations(0));
        let comp = FixedEvaluations {
            max_evaluations: 200,
        };
        comp.initialize(&problem, &mut state);
        state.set_value::<Evaluations>(100);
        assert!(comp.evaluate(&problem, &mut state));
        state.set_value::<Evaluations>(200);
        assert!(!comp.evaluate(&problem, &mut state));
    }

    #[test]
    fn updates_progress() {
        let problem = TestProblem;
        let mut state = State::new_root();
        state.insert(Evaluations(0));
        let comp = FixedEvaluations {
            max_evaluations: 200,
        };
        comp.initialize(&problem, &mut state);
        state.set_value::<Evaluations>(100);
        comp.evaluate(&problem, &mut state);
        float_eq::assert_float_eq!(state.get_value::<Progress>(), 0.5, ulps <= 2);
        state.set_value::<Evaluations>(200);
        comp.evaluate(&problem, &mut state);
        float_eq::assert_float_eq!(state.get_value::<Progress>(), 1.0, ulps <= 2);
    }
}

/// Terminates after distance to the known optimum is less than specified value.
/// Is restricted to problems where the optimum is known, i.e., implement [HasKnownOptimum].
///
/// Progress is unknown, as optimizer should not have information on optimum.
#[derive(Serialize, Deserialize)]
pub struct DistanceToOpt {
    /// Distance to known optimum.
    pub distance: f64,
}
impl DistanceToOpt {
    pub fn new<P: HasKnownOptimum>(distance: f64) -> Box<dyn Condition<P>>
    where
        P: SingleObjectiveProblem,
    {
        Box::new(Self { distance })
    }
}
impl<P: HasKnownOptimum + SingleObjectiveProblem> Condition<P> for DistanceToOpt
where
    P: Problem,
{
    fn evaluate(&self, problem: &P, state: &mut State) -> bool {
        state.best_objective_value::<P>().unwrap().value()
            >= problem.known_optimum().value() + self.distance
    }
}
#[cfg(test)]
mod distance_to_opt {
    use super::*;
    use crate::framework::state::common;
    use crate::testing::*;

    #[test]
    fn terminates() {
        let problem = TestProblem;
        let mut state = State::new_root();
        state.insert(common::BestIndividual::<TestProblem>::default());
        let comp = DistanceToOpt { distance: 0.1 };

        state.set_value::<common::BestIndividual<TestProblem>>(Some(new_test_individual(0.2)));
        assert!(comp.evaluate(&problem, &mut state));
        state.set_value::<common::BestIndividual<TestProblem>>(Some(new_test_individual(0.05)));
        assert!(!comp.evaluate(&problem, &mut state));
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
impl StepsWithoutImprovement {
    pub fn new<P: Problem>(steps: usize) -> Box<dyn Condition<P>>
    where
        P: SingleObjectiveProblem,
    {
        Box::new(Self { steps })
    }
}
impl<P> Condition<P> for StepsWithoutImprovement
where
    P: SingleObjectiveProblem,
{
    fn initialize(&self, _problem: &P, state: &mut State) {
        state.insert(FitnessImprovementState {
            current_steps: 0,
            current_objective: Default::default(),
        })
    }

    fn evaluate(&self, _problem: &P, state: &mut State) -> bool {
        let best_fitness = *state.best_objective_value::<P>().unwrap();
        let termination_state = state.get_mut::<FitnessImprovementState>();
        termination_state.update(&best_fitness);

        termination_state.current_steps < self.steps
    }
}
#[cfg(test)]
mod steps_without_improvement {
    use super::*;
    use crate::framework::state::common;
    use crate::testing::*;

    #[test]
    fn terminates() {
        let problem = TestProblem;
        let mut state = State::new_root();
        let comp = StepsWithoutImprovement { steps: 20 };
        state.insert(FitnessImprovementState {
            current_steps: 0,
            current_objective: 0.5.try_into().unwrap(),
        });
        state.insert(common::BestIndividual::<TestProblem>::default());
        state.insert(Iterations(0));
        state.set_value::<common::BestIndividual<TestProblem>>(Some(new_test_individual(0.5)));
        state.set_value::<Iterations>(10);
        assert!(comp.evaluate(&problem, &mut state));
        state.set_value::<common::BestIndividual<TestProblem>>(Some(new_test_individual(0.5)));
        let test_state = state.get_mut::<FitnessImprovementState>();
        test_state.current_steps = 20;
        assert!(!comp.evaluate(&problem, &mut state));
    }
}
