use crate::{
    framework::{Individual, SingleObjective},
    problems::{HasKnownOptimum, Problem, SingleObjectiveProblem},
};
use std::borrow::Borrow;

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

pub fn new_test_population(fitness_values: &[f64]) -> Vec<Individual<TestProblem>> {
    fitness_values
        .iter()
        .cloned()
        .map(|o| SingleObjective::try_from(o).unwrap())
        .map(Individual::new_test_unit)
        .collect()
}

pub fn collect_population_fitness<P: SingleObjectiveProblem, I: Borrow<Individual<P>>>(
    population: &[I],
) -> Vec<f64> {
    population
        .iter()
        .map(I::borrow)
        .map(Individual::objective)
        .map(|o| o.value())
        .collect()
}