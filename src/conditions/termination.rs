//! Termination criteria

use crate::conditions::Condition;
use crate::{
    framework::SingleObjective,
    problems::{HasKnownOptimum, HasKnownTarget, Problem, SingleObjectiveProblem},
    state::{
        common::{Evaluations, Iterations, Progress},
        CustomState, State,
    },
};
use better_any::Tid;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

#[derive(Serialize, Deserialize, Clone)]
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
    fn evaluate(&self, problem: &P, state: &mut State<P>) -> bool {
        if let Some(fitness) = state.best_objective_value() {
            !problem.target_hit(*fitness)
        } else {
            false
        }
    }
}

#[derive(Serialize, Deserialize, derivative::Derivative)]
#[derivative(Clone(bound = ""))]
pub struct LessThanN<T> {
    pub n: u32,
    _phantom: PhantomData<fn() -> T>,
}

impl<T> LessThanN<T> {
    pub fn new<P: Problem>(n: u32) -> Box<dyn Condition<P>>
    where
        P: Problem,
        T: for<'a> CustomState<'a> + std::ops::Deref<Target = u32> + Default,
    {
        Box::new(Self {
            n,
            _phantom: PhantomData,
        })
    }
}

impl<P, T> Condition<P> for LessThanN<T>
where
    P: Problem,
    T: for<'a> CustomState<'a> + std::ops::Deref<Target = u32> + Default,
{
    fn initialize(&self, _problem: &P, state: &mut State<P>) {
        state.insert(Progress::<T>::default());
    }

    fn require(&self, _problem: &P, state: &State<P>) {
        state.require::<Self, T>();
    }

    fn evaluate(&self, _problem: &P, state: &mut State<P>) -> bool {
        let value = state.get_value::<T>();
        state.set_value::<Progress<T>>(value as f64 / self.n as f64);

        value < self.n
    }
}

/// Terminates after a fixed number of iterations.
///
/// Supports measuring time to completion.
#[derive(Serialize, Deserialize, Clone)]
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
    fn initialize(&self, _problem: &P, state: &mut State<P>) {
        state.require::<Self, Iterations>();
        state.insert(Progress::<Iterations>::default());
    }

    fn evaluate(&self, _problem: &P, state: &mut State<P>) -> bool {
        let iterations = state.iterations();
        state.set_value::<Progress<Iterations>>(iterations as f64 / self.max_iterations as f64);

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
        let mut state = State::new();
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
        let mut state = State::new();
        state.insert(Iterations(0));
        let comp = FixedIterations {
            max_iterations: 200,
        };
        comp.initialize(&problem, &mut state);
        state.set_value::<Iterations>(100);
        comp.evaluate(&problem, &mut state);
        float_eq::assert_float_eq!(state.get_value::<Progress<Iterations>>(), 0.5, ulps <= 2);
        state.set_value::<Iterations>(200);
        comp.evaluate(&problem, &mut state);
        float_eq::assert_float_eq!(state.get_value::<Progress<Iterations>>(), 1.0, ulps <= 2);
    }
}

/// Terminates after a fixed number of evaluations.
///
/// Supports measuring time to completion.
#[derive(Serialize, Deserialize, Clone)]
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
    fn initialize(&self, _problem: &P, state: &mut State<P>) {
        state.require::<Self, Evaluations>();
        state.insert(Progress::<Evaluations>::default());
    }

    fn evaluate(&self, _problem: &P, state: &mut State<P>) -> bool {
        let evaluations = state.evaluations();
        state.set_value::<Progress<Evaluations>>(evaluations as f64 / self.max_evaluations as f64);
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
        let mut state = State::new();
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
        let mut state = State::new();
        state.insert(Evaluations(0));
        let comp = FixedEvaluations {
            max_evaluations: 200,
        };
        comp.initialize(&problem, &mut state);
        state.set_value::<Evaluations>(100);
        comp.evaluate(&problem, &mut state);
        float_eq::assert_float_eq!(state.get_value::<Progress<Evaluations>>(), 0.5, ulps <= 2);
        state.set_value::<Evaluations>(200);
        comp.evaluate(&problem, &mut state);
        float_eq::assert_float_eq!(state.get_value::<Progress<Evaluations>>(), 1.0, ulps <= 2);
    }
}

/// Terminates after distance to the known optimum is less than specified value.
/// Is restricted to problems where the optimum is known, i.e., implement [HasKnownOptimum].
///
/// Progress is unknown, as optimizer should not have information on optimum.
#[derive(Serialize, Deserialize, Clone)]
pub struct DistanceToOptGreaterThan {
    /// Distance to known optimum.
    pub distance: f64,
}

impl DistanceToOptGreaterThan {
    pub fn new<P: HasKnownOptimum>(distance: f64) -> Box<dyn Condition<P>>
    where
        P: SingleObjectiveProblem,
    {
        Box::new(Self { distance })
    }
}

impl<P: HasKnownOptimum + SingleObjectiveProblem> Condition<P> for DistanceToOptGreaterThan
where
    P: Problem,
{
    fn evaluate(&self, problem: &P, state: &mut State<P>) -> bool {
        state.best_objective_value().unwrap().value()
            >= problem.known_optimum().value() + self.distance
    }
}

#[cfg(test)]
mod distance_to_opt {
    use super::*;
    use crate::state::common;
    use crate::testing::*;

    #[test]
    fn terminates() {
        let problem = TestProblem;
        let mut state = State::new();
        state.insert(common::BestIndividual::<TestProblem>::default());
        let comp = DistanceToOptGreaterThan { distance: 0.1 };

        state.set_value::<common::BestIndividual<TestProblem>>(Some(new_test_individual(0.2)));
        assert!(comp.evaluate(&problem, &mut state));
        state.set_value::<common::BestIndividual<TestProblem>>(Some(new_test_individual(0.05)));
        assert!(!comp.evaluate(&problem, &mut state));
    }
}

/// State required for Termination by Steps without Improvement.
///
/// For preserving current number of steps without improvement and corresponding fitness value.
#[derive(Tid)]
struct FitnessImprovementState {
    pub current_steps: usize,
    pub current_objective: SingleObjective,
}

impl FitnessImprovementState {
    pub fn update(&mut self, objective: &SingleObjective) -> bool {
        if objective >= &self.current_objective {
            self.current_steps += 1;
            false
        } else {
            self.current_objective = *objective;
            self.current_steps = 0;
            true
        }
    }
}

impl CustomState<'_> for FitnessImprovementState {}

/// Terminates after a specified number of steps (iterations) did not yield any improvement.
///
/// Progress is unknown, as steps depend on current performance of optimizer.
#[derive(Serialize, Deserialize, Clone)]
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
    fn initialize(&self, _problem: &P, state: &mut State<P>) {
        state.insert(FitnessImprovementState {
            current_steps: 0,
            current_objective: Default::default(),
        })
    }

    fn evaluate(&self, _problem: &P, state: &mut State<P>) -> bool {
        let best_fitness = *state.best_objective_value().unwrap();
        let termination_state = state.get_mut::<FitnessImprovementState>();
        termination_state.update(&best_fitness);

        termination_state.current_steps < self.steps
    }
}

#[cfg(test)]
mod steps_without_improvement {
    use super::*;
    use crate::state::common;
    use crate::testing::*;

    #[test]
    fn terminates() {
        let problem = TestProblem;
        let mut state = State::new();
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
