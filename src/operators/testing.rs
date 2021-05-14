use crate::heuristic::Individual;
use std::borrow::Borrow;

pub fn new_test_population(fitness_values: &[f64]) -> Vec<Individual> {
    fitness_values
        .iter()
        .cloned()
        .map(Individual::new_test_unit)
        .collect()
}

pub fn collect_population_fitness<I: Borrow<Individual>>(population: &[I]) -> Vec<f64> {
    population
        .iter()
        .map(I::borrow)
        .map(Individual::fitness)
        .map(f64::from)
        .collect()
}
