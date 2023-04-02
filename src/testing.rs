//! Testing utilities.

use crate::{
    framework::{Individual, SingleObjective},
    problems::{Evaluator, HasKnownOptimum, Problem, SingleObjectiveProblem},
    state::{common::EvaluatorInstance, State},
};
use std::borrow::Borrow;

/// Helper problem for test purposes.
pub struct TestProblem;

impl Problem for TestProblem {
    type Encoding = ();
    type Objective = SingleObjective;

    fn name(&self) -> &str {
        "TestProblem"
    }

    fn default_evaluator<'a>(&self) -> EvaluatorInstance<'a, Self> {
        EvaluatorInstance::new(TestEvaluator)
    }
}

impl HasKnownOptimum for TestProblem {
    fn known_optimum(&self) -> SingleObjective {
        0.0.try_into().unwrap()
    }
}

pub struct TestEvaluator;

impl Evaluator for TestEvaluator {
    type Problem = TestProblem;

    fn evaluate(
        &mut self,
        _problem: &Self::Problem,
        _state: &mut State<Self::Problem>,
        individuals: &mut [Individual<Self::Problem>],
    ) {
        for individual in individuals {
            individual.evaluate(0.0.try_into().unwrap());
        }
    }
}

pub fn new_test_individual(objective: f64) -> Individual<TestProblem> {
    Individual::new_test_unit(objective.try_into().unwrap())
}

pub fn new_test_population(objective_values: &[f64]) -> Vec<Individual<TestProblem>> {
    objective_values
        .iter()
        .cloned()
        .map(new_test_individual)
        .collect()
}

pub fn collect_population_objective_values<P: SingleObjectiveProblem, I: Borrow<Individual<P>>>(
    population: &[I],
) -> Vec<f64> {
    population
        .iter()
        .map(I::borrow)
        .map(Individual::objective)
        .map(SingleObjective::value)
        .collect()
}
