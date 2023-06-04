#![allow(dead_code)]

use std::{any::type_name, marker::PhantomData};

use float_eq::assert_float_eq;

use crate::{
    individual::Individual,
    objective::{MultiObjective, Objective, SingleObjective},
    Problem,
};

pub struct TestProblem<O: Objective>(PhantomData<O>);

impl<O> TestProblem<O>
where
    O: Objective,
{
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<O> Problem for TestProblem<O>
where
    O: Objective + 'static,
{
    type Encoding = ();
    type Objective = O;

    fn name(&self) -> &str {
        type_name::<Self>()
    }
}

pub type SingleObjectiveTestProblem = TestProblem<SingleObjective>;
pub type MultiObjectiveTestProblem = TestProblem<MultiObjective>;

pub fn single_test_individual(objective: f64) -> Individual<SingleObjectiveTestProblem> {
    Individual::new_test_unit(objective.try_into().unwrap())
}

pub fn single_test_population(
    objective_values: &[f64],
) -> Vec<Individual<SingleObjectiveTestProblem>> {
    objective_values
        .iter()
        .cloned()
        .map(single_test_individual)
        .collect()
}

pub fn multi_test_individual(objective: &[f64]) -> Individual<MultiObjectiveTestProblem> {
    Individual::new_test_unit(objective.try_into().unwrap())
}

pub fn multi_test_population(
    objective_values: &[&[f64]],
) -> Vec<Individual<MultiObjectiveTestProblem>> {
    objective_values
        .iter()
        .cloned()
        .map(multi_test_individual)
        .collect()
}

pub fn assert_floats_eq(expected: &[f64], actual: &[f64]) {
    assert_eq!(expected.len(), actual.len());
    for (expected, actual) in expected.iter().zip(actual) {
        assert_float_eq!(expected, actual, ulps <= 6);
    }
}
