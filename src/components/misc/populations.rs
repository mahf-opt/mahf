use eyre::ensure;
use itertools::{interleave, Itertools};
use serde::{Deserialize, Serialize};

use crate::{
    component::ExecResult, components::Component, problems::SingleObjectiveProblem, Problem, State,
};

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
    #[contracts::requires()]
    fn execute(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
        ensure!(
            state.populations().len() >= self.n,
            "not enough populations to rotate"
        );
        state.populations_mut().rotate(self.n);
        Ok(())
    }
}

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
