//! Utility components for manipulating populations.

use eyre::ensure;
use itertools::{interleave, Itertools};
use serde::{Deserialize, Serialize};

use crate::{
    component::ExecResult, components::Component, problems::SingleObjectiveProblem, Problem, State,
};

/// Removes all individuals from the population.
#[derive(Clone, Serialize, Deserialize)]
pub struct ClearPopulation;

impl ClearPopulation {
    pub fn from_params() -> Self {
        Self
    }

    pub fn new<P: Problem>() -> Box<dyn Component<P>> {
        Box::new(Self::from_params())
    }
}

impl<P: Problem> Component<P> for ClearPopulation {
    fn execute(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
        state.populations_mut().current_mut().clear();
        Ok(())
    }
}

/// Shifts the first `n` populations circularly by one.
///
/// See [`Populations::rotate`] for more information.
///
/// [`Populations::rotate`]: crate::state::common::Populations::rotate
#[derive(Clone, Serialize, Deserialize)]
pub struct RotatePopulations {
    n: usize,
}

impl RotatePopulations {
    pub fn from_params(n: usize) -> Self {
        Self { n }
    }

    pub fn new<P: Problem>(n: usize) -> Box<dyn Component<P>> {
        Box::new(Self::from_params(n))
    }
}

impl<P: Problem> Component<P> for RotatePopulations {
    fn execute(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
        ensure!(
            state.populations().len() >= self.n,
            "not enough populations to rotate"
        );
        state.populations_mut().rotate(self.n);
        Ok(())
    }
}

/// Splits the population into two subpopulations, where the upper populations contains
/// the half of the previous population with better objective values, and the lower population
/// the half with worse objective values.
#[derive(Clone, Serialize, Deserialize)]
pub struct SplitPopulationByObjectiveValue;

impl SplitPopulationByObjectiveValue {
    pub fn from_params() -> Self {
        Self
    }

    pub fn new<P: SingleObjectiveProblem>() -> Box<dyn Component<P>> {
        Box::new(Self::from_params())
    }
}

impl<P: SingleObjectiveProblem> Component<P> for SplitPopulationByObjectiveValue {
    fn execute(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
        let mut populations = state.populations_mut();
        let mut population = populations.pop();

        population.sort_unstable_by_key(|i| *i.objective());
        let n = population.len();

        // For the amount of chunks, we want to use ceiling division, such that the
        // second chunk may be one element shorter when `n` is odd,
        // but there are still exactly two chunks in total.
        let (lower, upper) = population
            .into_iter()
            .chunks((n + 1) / 2)
            .into_iter()
            .map(|c| c.collect_vec())
            .collect_tuple::<(_, _)>()
            .unwrap();
        populations.push(upper);
        populations.push(lower);
        Ok(())
    }
}

/// Interleaves the two top-most populations into one population.
#[derive(Clone, Serialize, Deserialize)]
pub struct InterleavePopulations;

impl InterleavePopulations {
    pub fn from_params() -> Self {
        Self
    }

    pub fn new<P: SingleObjectiveProblem>() -> Box<dyn Component<P>> {
        Box::new(Self::from_params())
    }
}

impl<P: SingleObjectiveProblem> Component<P> for InterleavePopulations {
    fn execute(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
        let mut populations = state.populations_mut();
        let p1 = populations.pop();
        let p2 = populations.pop();
        populations.push(interleave(p1, p2).collect());
        Ok(())
    }
}

/// Duplicates all individuals in the population interleaved, i.e. each individual is immediately
/// followed by its duplicate.
#[derive(Clone, Serialize, Deserialize)]
pub struct DuplicatePopulation;

impl DuplicatePopulation {
    pub fn from_params() -> Self {
        Self
    }

    pub fn new<P: Problem>() -> Box<dyn Component<P>> {
        Box::new(Self::from_params())
    }
}

impl<P: Problem> Component<P> for DuplicatePopulation {
    fn execute(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
        let mut populations = state.populations_mut();
        let population = populations.pop();
        let duplicates = population.clone();
        populations.push(interleave(population, duplicates).collect());
        Ok(())
    }
}
