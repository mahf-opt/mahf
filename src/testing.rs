use crate::{
    framework::{Individual, SingleObjective},
    problems::{HasKnownOptimum, Problem, SingleObjectiveProblem},
};
use std::borrow::Borrow;

/// Helper problem for test purposes.
pub struct TestProblem;

impl Problem for TestProblem {
    type Encoding = ();
    type Objective = SingleObjective;

    fn evaluate_solution(&self, _solution: &Self::Encoding) -> Self::Objective {
        0.0.try_into().unwrap()
    }

    fn name(&self) -> &str {
        "TestProblem"
    }
}

impl HasKnownOptimum for TestProblem {
    fn known_optimum(&self) -> SingleObjective {
        0.0.try_into().unwrap()
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
