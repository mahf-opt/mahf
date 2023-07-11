//! Common selection components.

use color_eyre::Section;
use eyre::{ensure, ContextCompat, WrapErr};
use rand::{seq::SliceRandom, Rng};
use serde::{Deserialize, Serialize};

use crate::{
    component::ExecResult,
    components::{
        selection::{functional as f, selection, Selection},
        Component,
    },
    population::IntoSingleRef,
    problems::SingleObjectiveProblem,
    state::random::Random,
    Individual, Problem, State,
};

/// Selects all individuals once.
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

/// Selects no individual.
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

/// Select the single individual in the population `num_selected` times.
///
/// This is useful for cloning the current individual in a single-solution-based metaheuristic
/// and then performing some operation on each cloned individual e.g. for generating multiple neighbors.
///
/// # Errors
///
/// The component returns an `Err` if the population does not contain exactly one individual.
#[derive(Clone, Serialize, Deserialize)]
pub struct CloneSingle {
    /// Number of times the single individual is cloned.
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
        let single = population.into_single_ref()?;
        Ok(std::iter::repeat(single)
            .take(self.num_selected as usize)
            .collect())
    }
}

impl<P: Problem> Component<P> for CloneSingle {
    fn execute(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        selection(self, problem, state)
    }
}

/// Selects `num_selected` random individuals with replacement.
#[derive(Clone, Serialize, Deserialize)]
pub struct FullyRandom {
    /// Number of selected individuals.
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

/// Selects `num_selected` random individuals without replacement.
///
/// # Errors
///
/// Returns an `Err` if there is not a sufficient amount of solutions to choose from.
#[derive(Clone, Serialize, Deserialize)]
pub struct RandomWithoutRepetition {
    /// Number of selected individuals.
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
        let num_selected = self.num_selected as usize;
        ensure!(
            population.len() > num_selected,
            "the population does not contain enough individuals to sample without replacement"
        );
        let selection = population.choose_multiple(rng, num_selected).collect();
        Ok(selection)
    }
}

impl<P: Problem> Component<P> for RandomWithoutRepetition {
    fn execute(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        selection(self, problem, state)
    }
}

/// Selects `num_selected` individuals using the roulette-wheel method with replacement.
///
/// The weights for individuals are calculated using
/// [`proportional_weights`] with the `offset` as argument.
///
/// [`proportional_weights`]: f::proportional_weights
///
/// # Errors
///
/// Returns an `Err` if the population contains individuals with infinite objective value.
#[derive(Clone, Serialize, Deserialize)]
pub struct RouletteWheel {
    /// Number of selected individuals.
    pub num_selected: u32,
    /// The `offset` weight to prevent the worst individual having a weight of 0.
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

/// Selects `num_selected` individuals using stochastic universal sampling with replacement.
///
/// Note that the population is not sorted by fitness, but individuals are weighted "in place".
///
/// The weights for individuals are calculated using
/// [`proportional_weights`] with the `offset` as argument.
///
/// [`proportional_weights`]: f::proportional_weights
///
/// # Errors
///
/// Returns an `Err` if the population contains individuals with infinite objective value.
#[derive(Clone, Serialize, Deserialize)]
pub struct StochasticUniversalSampling {
    /// Number of selected individuals.
    pub num_selected: u32,
    /// The `offset` weight to prevent the worst individual having a weight of 0.
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

/// Selects `num_selected` individuals using deterministic Tournament selection of `size` with replacement.
#[derive(Clone, Serialize, Deserialize)]
pub struct Tournament {
    /// Number of selected individuals.
    pub num_selected: u32,
    /// Tournament size.
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

/// Selects `num_selected` solutions with replacement using linear ranking.
#[derive(Clone, Serialize, Deserialize)]
pub struct LinearRank {
    /// Number of selected individuals.
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

/// Selects `num_selected` solutions with replacement using exponential ranking.
#[derive(Serialize, Deserialize, Clone)]
pub struct ExponentialRank {
    /// Number of selected individuals.
    pub num_selected: u32,
    /// Base of the exponent within (0,1).
    pub base: f64,
}

impl ExponentialRank {
    pub fn from_params(num_selected: u32, base: f64) -> ExecResult<Self> {
        ensure!(
            (f64::EPSILON..1.0).contains(&base),
            "the base of the exponent must be within (0, 1)"
        );
        Ok(Self { num_selected, base })
    }

    pub fn new<P: SingleObjectiveProblem>(
        num_selected: u32,
        base: f64,
    ) -> ExecResult<Box<dyn Component<P>>> {
        Ok(Box::new(Self::from_params(num_selected, base)?))
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
