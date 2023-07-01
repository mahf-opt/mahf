//! Common selection components.

use color_eyre::Section;
use eyre::{ensure, eyre, ContextCompat, WrapErr};
use rand::{seq::SliceRandom, Rng};
use serde::{Deserialize, Serialize};

use crate::{
    component::ExecResult,
    components::{
        selection::{functional as f, selection, Selection},
        Component,
    },
    problems::SingleObjectiveProblem,
    state::random::Random,
    Individual, Problem, State,
};

#[derive(Clone, Serialize, Deserialize)]
pub struct All;

impl All {
    pub fn from_params() -> Self {
        Self
    }

    pub fn new<P: Problem>() -> Box<dyn Component<P>> {
        Box::new(Self::from_params())
    }
}

impl<P: Problem> Selection<P> for All {
    fn select<'a>(
        &self,
        population: &'a [Individual<P>],
        _rng: &mut Random,
    ) -> ExecResult<Vec<&'a Individual<P>>> {
        Ok(population.iter().collect())
    }
}

impl<P: Problem> Component<P> for All {
    fn execute(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        selection(self, problem, state)
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct None;

impl None {
    pub fn from_params() -> Self {
        Self
    }

    pub fn new<P: Problem>() -> Box<dyn Component<P>> {
        Box::new(Self::from_params())
    }
}

impl<P: Problem> Selection<P> for None {
    fn select<'a>(
        &self,
        _population: &'a [Individual<P>],
        _rng: &mut Random,
    ) -> ExecResult<Vec<&'a Individual<P>>> {
        Ok(Vec::new())
    }
}

impl<P: Problem> Component<P> for None {
    fn execute(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        selection(self, problem, state)
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CloneSingle {
    pub num_selected: u32,
}

impl CloneSingle {
    pub fn from_params(num_selected: u32) -> Self {
        Self { num_selected }
    }

    pub fn new<P: Problem>(num_selected: u32) -> Box<dyn Component<P>> {
        Box::new(Self::from_params(num_selected))
    }
}

impl<P: Problem> Selection<P> for CloneSingle {
    fn select<'a>(
        &self,
        population: &'a [Individual<P>],
        _rng: &mut Random,
    ) -> ExecResult<Vec<&'a Individual<P>>> {
        if let [single] = population {
            Ok(std::iter::repeat(single)
                .take(self.num_selected as usize)
                .collect())
        } else {
            Err(eyre!(
                "population must contain a single individual, but {} were found",
                population.len()
            ))
        }
    }
}

impl<P: Problem> Component<P> for CloneSingle {
    fn execute(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        selection(self, problem, state)
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct FullyRandom {
    pub num_selected: u32,
}

impl FullyRandom {
    pub fn from_params(num_selected: u32) -> Self {
        Self { num_selected }
    }

    pub fn new<P: Problem>(num_selected: u32) -> Box<dyn Component<P>> {
        Box::new(Self::from_params(num_selected))
    }
}

impl<P: Problem> Selection<P> for FullyRandom {
    fn select<'a>(
        &self,
        population: &'a [Individual<P>],
        rng: &mut Random,
    ) -> ExecResult<Vec<&'a Individual<P>>> {
        let mut selection = Vec::new();
        for _ in 0..self.num_selected {
            selection.push(population.choose(rng).unwrap());
        }
        Ok(selection)
    }
}

impl<P: Problem> Component<P> for FullyRandom {
    fn execute(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        selection(self, problem, state)
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RandomWithoutRepetition {
    pub num_selected: u32,
}

impl RandomWithoutRepetition {
    pub fn from_params(num_selected: u32) -> Self {
        Self { num_selected }
    }

    pub fn new<P: Problem>(num_selected: u32) -> Box<dyn Component<P>> {
        Box::new(Self::from_params(num_selected))
    }
}

impl<P: Problem> Selection<P> for RandomWithoutRepetition {
    fn select<'a>(
        &self,
        population: &'a [Individual<P>],
        rng: &mut Random,
    ) -> ExecResult<Vec<&'a Individual<P>>> {
        let selection = population
            .choose_multiple(rng, self.num_selected as usize)
            .collect();
        Ok(selection)
    }
}

impl<P: Problem> Component<P> for RandomWithoutRepetition {
    fn execute(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        selection(self, problem, state)
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RouletteWheel {
    pub num_selected: u32,
    pub offset: f64,
}

impl RouletteWheel {
    pub fn from_params(num_selected: u32, offset: f64) -> Self {
        Self {
            num_selected,
            offset,
        }
    }

    pub fn new<P: SingleObjectiveProblem>(num_selected: u32, offset: f64) -> Box<dyn Component<P>> {
        Box::new(Self::from_params(num_selected, offset))
    }
}

impl<P: SingleObjectiveProblem> Selection<P> for RouletteWheel {
    fn select<'a>(
        &self,
        population: &'a [Individual<P>],
        rng: &mut Random,
    ) -> ExecResult<Vec<&'a Individual<P>>> {
        let weights = f::proportional_weights(population, self.offset, false)
            .wrap_err("population contains invalid objective values")
            .note("roulette wheel does not work with infinite objective values")?;
        let selection = f::sample_population_weighted(population, &weights, self.num_selected, rng)
            .wrap_err("sampling from population failed")?;
        Ok(selection)
    }
}

impl<P: SingleObjectiveProblem> Component<P> for RouletteWheel {
    fn execute(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        selection(self, problem, state)
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct StochasticUniversalSampling {
    pub num_selected: u32,
    pub offset: f64,
}

impl StochasticUniversalSampling {
    pub fn from_params(num_selected: u32, offset: f64) -> Self {
        Self {
            num_selected,
            offset,
        }
    }

    pub fn new<P: SingleObjectiveProblem>(num_selected: u32, offset: f64) -> Box<dyn Component<P>> {
        Box::new(Self::from_params(num_selected, offset))
    }
}

impl<P: SingleObjectiveProblem> Selection<P> for StochasticUniversalSampling {
    fn select<'a>(
        &self,
        population: &'a [Individual<P>],
        rng: &mut Random,
    ) -> ExecResult<Vec<&'a Individual<P>>> {
        let weights = f::proportional_weights(population, self.offset, false)
            .wrap_err("population contains invalid objective values")
            .note("stochastic universal sampling does not work with infinite objective values")?;

        // Calculate the distance between selection points and the random start point
        let weights_total = weights.iter().sum();
        let gaps = weights_total / self.num_selected as f64;
        let start = rng.gen::<f64>() * gaps;
        let mut distance = start;

        // Select the individuals for which the selection point falls within their fitness range
        let mut selection = Vec::new();
        let mut sum_weights = weights[0];
        let mut i: usize = 0;
        while distance < weights_total {
            while sum_weights < distance {
                i += 1;
                sum_weights += weights[i];
            }
            selection.push(&population[i]);
            distance += gaps;
        }
        Ok(selection)
    }
}

impl<P: SingleObjectiveProblem> Component<P> for StochasticUniversalSampling {
    fn execute(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        selection(self, problem, state)
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Tournament {
    pub num_selected: u32,
    pub size: u32,
}

impl Tournament {
    pub fn from_params(num_selected: u32, size: u32) -> Self {
        Self { num_selected, size }
    }

    pub fn new<P: SingleObjectiveProblem>(num_selected: u32, size: u32) -> Box<dyn Component<P>> {
        Box::new(Self::from_params(num_selected, size))
    }
}

impl<P: SingleObjectiveProblem> Selection<P> for Tournament {
    fn select<'a>(
        &self,
        population: &'a [Individual<P>],
        rng: &mut Random,
    ) -> ExecResult<Vec<&'a Individual<P>>> {
        ensure!(
            population.len() >= self.size as usize,
            "population size must be equal to or greater than the tournament size"
        );
        let mut selection = Vec::new();
        for _ in 0..self.num_selected {
            // Choose `size` competitors in tournament and select the winner
            let winner = population
                .choose_multiple(rng, self.size as usize)
                .min_by_key(|&i| i.objective())
                .unwrap();
            selection.push(winner);
        }
        Ok(selection)
    }
}

impl<P: SingleObjectiveProblem> Component<P> for Tournament {
    fn execute(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        selection(self, problem, state)
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct LinearRank {
    pub num_selected: u32,
}

impl LinearRank {
    pub fn from_params(num_selected: u32) -> Self {
        Self { num_selected }
    }

    pub fn new<P: SingleObjectiveProblem>(num_selected: u32) -> Box<dyn Component<P>> {
        Box::new(Self::from_params(num_selected))
    }
}

impl<P: SingleObjectiveProblem> Selection<P> for LinearRank {
    fn select<'a>(
        &self,
        population: &'a [Individual<P>],
        rng: &mut Random,
    ) -> ExecResult<Vec<&'a Individual<P>>> {
        let weights = f::reverse_rank(population);
        let selection = f::sample_population_weighted(population, &weights, self.num_selected, rng)
            .wrap_err("sampling from population failed")?;
        Ok(selection)
    }
}

impl<P: SingleObjectiveProblem> Component<P> for LinearRank {
    fn execute(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        selection(self, problem, state)
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ExponentialRank {
    pub num_selected: u32,
    pub base: f64,
}

impl ExponentialRank {
    pub fn from_params(num_selected: u32, base: f64) -> Self {
        Self { num_selected, base }
    }

    pub fn new<P: SingleObjectiveProblem>(num_selected: u32, base: f64) -> Box<dyn Component<P>> {
        Box::new(Self::from_params(num_selected, base))
    }
}

impl<P: SingleObjectiveProblem> Selection<P> for ExponentialRank {
    fn select<'a>(
        &self,
        population: &'a [Individual<P>],
        rng: &mut Random,
    ) -> ExecResult<Vec<&'a Individual<P>>> {
        let ranking = f::reverse_rank(population);
        let max_rank = ranking.iter().max().cloned().unwrap_or(0);
        let factor = (self.base - 1.0) / (self.base.powi(max_rank as i32) - 1.0);
        let weights: Vec<_> = ranking
            .iter()
            .map(|i| factor * (self.base.powi((max_rank - i) as i32)))
            .collect();
        let selection = f::sample_population_weighted(population, &weights, self.num_selected, rng)
            .wrap_err("sampling from population failed")?;
        Ok(selection)
    }
}

impl<P: SingleObjectiveProblem> Component<P> for ExponentialRank {
    fn execute(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        selection(self, problem, state)
    }
}
