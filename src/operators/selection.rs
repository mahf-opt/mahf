//! Selection methods

use rand::{
    distributions::{Distribution, WeightedIndex},
    seq::SliceRandom,
    Rng,
};
use serde::{Deserialize, Serialize};

use crate::{
    framework::{components::*, state::State, Individual, SingleObjective},
    problems::{Problem, SingleObjectiveProblem},
};

/// Specialized component trait to select a subset of the current population and push it on the stack.
///
/// # Implementing [Component]
///
/// Types implementing this trait can implement [Component] by wrapping the type in a [Selector].
pub trait Selection<P: Problem> {
    fn select_offspring<'p>(
        &self,
        population: &'p [Individual<P>],
        state: &mut State,
    ) -> Vec<&'p Individual<P>>;
}

#[derive(serde::Serialize)]
pub struct Selector<T>(pub T);

impl<T, P> Component<P> for Selector<T>
where
    P: Problem,
    T: AnyComponent + Selection<P> + Serialize,
{
    fn execute(&self, _problem: &P, state: &mut State) {
        let population = state.population_stack_mut().pop();
        let selection: Vec<_> = self
            .0
            .select_offspring(&population, state)
            .into_iter()
            .cloned()
            .collect();
        state.population_stack_mut().push(population);
        state.population_stack_mut().push(selection);
    }
}

/// Helper function to obtain minimum and maximum objective ranges for normalization.
fn get_objective_range<P: SingleObjectiveProblem>(population: &[Individual<P>]) -> (f64, f64) {
    let best = population
        .iter()
        .map(Individual::objective)
        .min()
        .unwrap()
        .value();
    let worst = population
        .iter()
        .map(Individual::objective)
        .max()
        .unwrap()
        .value();
    (worst, best)
}

/// Helper function to calculate normalized fitness weights from the population,
/// where individuals with lower fitness get more weight.
fn get_minimizing_weights<P: SingleObjectiveProblem>(population: &[Individual<P>]) -> Vec<f64> {
    let (worst, best) = get_objective_range(population);
    assert!(
        worst.is_finite(),
        "weighting does not work with Inf fitness values"
    );
    // Normalize fitness values
    let normalized: Vec<f64> = population
        .iter()
        .map(|i| (i.objective().value() - best) / (worst - best))
        .collect();
    // Calculate population fitness as sum of individuals' fitness
    let total: f64 = normalized.iter().sum();
    // Calculate weights for individuals (fitness / total fitness)
    let weights: Vec<f64> = normalized.iter().map(|f| f / total).collect();
    // Due to minimisation, lower fitness is better, so adapt weights
    let weights_min: Vec<f64> = weights.iter().map(|&w| 1.0 - w).collect();

    weights_min
}

/// Helper function to obtain a ranking from the population.
fn get_ranking<P: SingleObjectiveProblem>(population: &[Individual<P>]) -> Vec<usize> {
    // First descending argsort with fitness
    let mut positions: Vec<_> = (1..=population.len()).collect();
    positions.sort_unstable_by_key(|&i| {
        SingleObjective::try_from(-population[i - 1].objective().value()).unwrap()
    });

    // Second argsort with positions obtains ranking
    let mut ranking: Vec<_> = (1..=population.len()).collect();
    ranking.sort_unstable_by_key(|&i| positions[i - 1]);

    ranking
}

/// Selects all individuals once.
#[derive(Serialize, Deserialize)]
pub struct All;
impl All {
    pub fn new<P: Problem>() -> Box<dyn Component<P>> {
        Box::new(Selector(Self))
    }
}
impl<P: Problem> Selection<P> for All {
    fn select_offspring<'p>(
        &self,
        population: &'p [Individual<P>],
        _state: &mut State,
    ) -> Vec<&'p Individual<P>> {
        population.iter().collect()
    }
}

/// Selects no individual.
#[derive(Serialize, Deserialize)]
pub struct None;
impl None {
    pub fn new<P: Problem>() -> Box<dyn Component<P>> {
        Box::new(Selector(Self))
    }
}
impl<P: Problem> Selection<P> for None {
    fn select_offspring<'p>(
        &self,
        _population: &'p [Individual<P>],
        _state: &mut State,
    ) -> Vec<&'p Individual<P>> {
        Vec::new()
    }
}

/// Select the single solution `offspring` times.
#[derive(Serialize, Deserialize)]
pub struct DuplicateSingle {
    /// Offspring per iteration.
    pub offspring: u32,
}
impl DuplicateSingle {
    pub fn new<P: Problem>(offspring: u32) -> Box<dyn Component<P>> {
        Box::new(Selector(Self { offspring }))
    }
}
impl<P: Problem> Selection<P> for DuplicateSingle {
    fn select_offspring<'p>(
        &self,
        population: &'p [Individual<P>],
        _state: &mut State,
    ) -> Vec<&'p Individual<P>> {
        assert_eq!(population.len(), 1);
        let single_solution = population.first().unwrap();
        (0..self.offspring)
            .into_iter()
            .map(|_| single_solution)
            .collect()
    }
}

#[cfg(test)]
mod duplicate_single {
    use crate::testing::*;

    use super::*;

    #[test]
    fn selects_right_number_of_children() {
        let mut state = State::new_root();
        // Note that the population contains exactly one element
        let population = new_test_population(&[1.0]);
        let comp = DuplicateSingle { offspring: 4 };
        let selection = comp.select_offspring(&population, &mut state);
        assert_eq!(selection.len(), comp.offspring as usize);
    }
}

/// Selects `offspring` random solutions.
///
/// Solutions can be selected multiple times in a single iteration.
#[derive(Serialize, Deserialize)]
pub struct FullyRandom {
    /// Offspring per iteration.
    pub offspring: u32,
}
impl FullyRandom {
    pub fn new<P: Problem>(offspring: u32) -> Box<dyn Component<P>> {
        Box::new(Selector(Self { offspring }))
    }
}
impl<P: Problem> Selection<P> for FullyRandom {
    fn select_offspring<'p>(
        &self,
        population: &'p [Individual<P>],
        state: &mut State,
    ) -> Vec<&'p Individual<P>> {
        let rng = state.random_mut();
        let mut selection = Vec::new();
        for _ in 0..self.offspring {
            selection.push(population.choose(rng).unwrap());
        }
        selection
    }
}
#[cfg(test)]
mod fully_random {
    use crate::framework::Random;
    use crate::testing::*;

    use super::*;

    #[test]
    fn selects_right_number_of_children() {
        let mut state = State::new_root();
        state.insert(Random::testing());
        let population = new_test_population(&[1.0, 2.0, 3.0]);
        let comp = FullyRandom { offspring: 4 };
        let selection = comp.select_offspring(&population, &mut state);
        assert_eq!(selection.len(), comp.offspring as usize);
    }
}

/// Deterministically selects individuals proporional to their fitness.
///
/// Originally proposed for, and use as selection in the Invasive Weed Optimization.
///
/// Each individual gets selected between `min_offspring` and `max_offspring` times.
///
/// - The worst solution gets selected exactly `min_offspring` times.
/// - The best solution gets selected exactly `max_offspring` times.
/// - All solutions between them get selected based on the linear interpolation
///   between `min_offspring` and `max_offspring`.
///
/// # Problems
/// - *Individuals with `Inf` fitness*. They will mess up this operator,
///   as it does not allow interpolating between the best and worst fitness.
/// - *Homogeneity*. If all individuals have the same fitness value,
///   they will all be considered average and receive a 50% bonus.
///   This case has not been accounted for in the reference paper.
///
/// # References
/// See [crate::heuristics::iwo]
#[derive(Serialize, Deserialize)]
pub struct DeterministicFitnessProportional {
    /// Minimum offspring per individual per iteration
    pub min_offspring: u32,
    /// Maximum offspring per individual per iteration
    pub max_offspring: u32,
}
impl DeterministicFitnessProportional {
    pub fn new<P: SingleObjectiveProblem>(
        min_offspring: u32,
        max_offspring: u32,
    ) -> Box<dyn Component<P>> {
        assert!(min_offspring <= max_offspring);
        Box::new(Selector(Self {
            min_offspring,
            max_offspring,
        }))
    }
}

impl<P: SingleObjectiveProblem> Selection<P> for DeterministicFitnessProportional {
    fn select_offspring<'p>(
        &self,
        population: &'p [Individual<P>],
        _state: &mut State,
    ) -> Vec<&'p Individual<P>> {
        let (worst, best) = get_objective_range(population);

        assert!(
            worst.is_finite(),
            "selection::DeterministicFitnessProportional does not work with Inf fitness values"
        );

        let mut selection = Vec::new();

        for ind in population.iter() {
            let bonus: f64 = (ind.objective().value() - worst) / (best - worst);
            let bonus_offspring = (self.max_offspring - self.min_offspring) as f64;
            let num_offspring = self.min_offspring
                + if bonus.is_nan() {
                    // best ≈ worst, thus we picked a generic value
                    (0.5 * bonus_offspring).floor() as u32
                } else {
                    (bonus * bonus_offspring).floor() as u32
                };

            for _ in 0..num_offspring {
                selection.push(ind);
            }
        }
        selection
    }
}

#[cfg(test)]
mod deterministic_fitness_proportional {
    use crate::framework::Random;
    use crate::testing::*;

    use super::*;

    #[test]
    fn selects_right_children() {
        let mut state = State::new_root();
        state.insert(Random::testing());
        let population = new_test_population(&[1.0, 2.0, 3.0]);
        let comp = DeterministicFitnessProportional {
            min_offspring: 1,
            max_offspring: 3,
        };
        let selection = comp.select_offspring(&population, &mut state);
        let selection: Vec<f64> = selection
            .into_iter()
            .map(|i| i.objective().value())
            .collect();

        assert!(selection.len() > (comp.min_offspring as usize * population.len()));
        assert!(selection.len() < (comp.max_offspring as usize * population.len()));

        // I(1.0) should have 3 seed
        // I(2.0) should have (1 + 3/2) seeds
        // I(3.0) should have 1 seeds
        assert_eq!(selection, vec![1.0, 1.0, 1.0, 2.0, 2.0, 3.0]);
    }
}

/// Selects `offspring` solutions using roulette-wheel method.
///
/// Solutions can be selected multiple times in a single iteration.
#[derive(Serialize, Deserialize)]
pub struct RouletteWheel {
    /// Offspring per iteration.
    pub offspring: u32,
}
impl RouletteWheel {
    pub fn new<P: SingleObjectiveProblem>(offspring: u32) -> Box<dyn Component<P>> {
        Box::new(Selector(Self { offspring }))
    }
}
impl<P: SingleObjectiveProblem> Selection<P> for RouletteWheel {
    fn select_offspring<'p>(
        &self,
        population: &'p [Individual<P>],
        state: &mut State,
    ) -> Vec<&'p Individual<P>> {
        let rng = state.random_mut();
        let weights_min = get_minimizing_weights(population);
        let wheel = WeightedIndex::new(weights_min).unwrap();
        wheel
            .sample_iter(rng)
            .take(self.offspring as usize)
            .map(|i| &population[i])
            .collect()
    }
}
#[cfg(test)]
mod roulette_wheel {
    use crate::framework::Random;
    use crate::testing::*;

    use super::*;

    #[test]
    fn selects_right_number_of_children() {
        let mut state = State::new_root();
        state.insert(Random::testing());
        let population = new_test_population(&[1.0, 2.0, 3.0]);
        let comp = RouletteWheel { offspring: 4 };
        let selection = comp.select_offspring(&population, &mut state);
        assert_eq!(selection.len(), comp.offspring as usize);
    }
}

/// Selects `offspring` solutions using stochastic universal sampling.
///
/// Solutions can be selected multiple times in a single iteration. Population is not sorted by fitness,
/// but individuals are weighted "in place".
#[derive(Serialize, Deserialize)]
pub struct StochasticUniversalSampling {
    /// Offspring per iteration.
    pub offspring: u32,
}
impl StochasticUniversalSampling {
    pub fn new<P: SingleObjectiveProblem>(offspring: u32) -> Box<dyn Component<P>> {
        Box::new(Selector(Self { offspring }))
    }
}
impl<P: SingleObjectiveProblem> Selection<P> for StochasticUniversalSampling {
    fn select_offspring<'p>(
        &self,
        population: &'p [Individual<P>],
        state: &mut State,
    ) -> Vec<&'p Individual<P>> {
        let rng = state.random_mut();
        let weights_min = get_minimizing_weights(population);

        // Calculate the distance between selection points and the random start point
        let weights_total = weights_min.iter().sum();
        let gaps = weights_total / self.offspring as f64;
        let start = rng.gen::<f64>() * gaps;
        let mut distance = start;

        // Select the individuals for which the selection point falls within their fitness range
        let mut selection = Vec::new();
        let mut sum_weights = weights_min[0];
        let mut i: usize = 0;
        while distance < weights_total {
            while sum_weights < distance {
                i += 1;
                sum_weights += weights_min[i];
            }
            selection.push(&population[i]);
            distance += gaps;
        }
        assert_eq!(selection.len(), self.offspring as usize);
        selection
    }
}
#[cfg(test)]
mod stochastic_universal_sampling {
    use crate::framework::Random;
    use crate::testing::*;

    use super::*;

    #[test]
    fn selects_right_number_of_children() {
        let mut state = State::new_root();
        state.insert(Random::testing());
        let population = new_test_population(&[1.0, 2.0, 3.0]);
        let comp = StochasticUniversalSampling { offspring: 4 };
        let selection = comp.select_offspring(&population, &mut state);
        assert_eq!(selection.len(), comp.offspring as usize);
    }
}

/// Selects `offspring` using deterministic Tournament selection.
///
/// Solutions can be selected multiple times in a single iteration.
#[derive(Serialize, Deserialize)]
pub struct Tournament {
    /// Offspring per iteration.
    pub offspring: u32,
    /// Tournament size.
    pub size: u32,
}
impl Tournament {
    pub fn new<P: SingleObjectiveProblem>(offspring: u32, size: u32) -> Box<dyn Component<P>> {
        Box::new(Selector(Self { offspring, size }))
    }
}
impl<P: SingleObjectiveProblem> Selection<P> for Tournament {
    fn select_offspring<'p>(
        &self,
        population: &'p [Individual<P>],
        state: &mut State,
    ) -> Vec<&'p Individual<P>> {
        assert!(population.len() >= self.size as usize);
        let rng = state.random_mut();
        let mut selection = Vec::new();
        // For each individual
        for _ in 0..self.offspring {
            // choose size competitors in tournament
            let mut tournament: Vec<&Individual<P>> = population
                .choose_multiple(rng, self.size as usize)
                .collect();
            // and evaluate them against each other, placing the winner first
            tournament.sort_unstable_by(|x, y| {
                (x.objective().value())
                    .partial_cmp(&(y.objective().value()))
                    .unwrap()
            });
            // Add winner (first) to selection
            selection.push(tournament[0]);
        }
        assert_eq!(selection.len(), self.offspring as usize);
        selection
    }
}
#[cfg(test)]
mod tournament {
    use crate::framework::Random;
    use crate::testing::*;

    use super::*;

    #[test]
    fn selects_right_number_of_children() {
        let mut state = State::new_root();
        state.insert(Random::testing());
        let population = new_test_population(&[1.0, 2.0, 3.0]);
        let comp = Tournament {
            offspring: 4,
            size: 2,
        };
        let selection = comp.select_offspring(&population, &mut state);
        assert_eq!(selection.len(), comp.offspring as usize);
    }
}

/// Selects `offspring` solutions using linear ranking.
///
/// Solutions can be selected multiple times in a single iteration.
#[derive(Serialize, Deserialize)]
pub struct LinearRank {
    /// Offspring per iteration.
    pub offspring: u32,
}
impl LinearRank {
    pub fn new<P: SingleObjectiveProblem>(offspring: u32) -> Box<dyn Component<P>> {
        Box::new(Selector(Self { offspring }))
    }
}
impl<P: SingleObjectiveProblem> Selection<P> for LinearRank {
    fn select_offspring<'p>(
        &self,
        population: &'p [Individual<P>],
        state: &mut State,
    ) -> Vec<&'p Individual<P>> {
        let rng = state.random_mut();
        let weights = get_ranking(population);
        let wheel = WeightedIndex::new(&weights).unwrap();
        let mut selection = Vec::new();
        for _ in 0..self.offspring {
            selection.push(&population[wheel.sample(rng)]);
        }
        assert_eq!(selection.len(), self.offspring as usize);
        selection
    }
}
#[cfg(test)]
mod linear_rank {
    use crate::framework::Random;
    use crate::testing::*;

    use super::*;

    #[test]
    fn selects_right_number_of_children() {
        let mut state = State::new_root();
        state.insert(Random::testing());
        let population = new_test_population(&[1.0, 2.0, 3.0]);
        let comp = LinearRank { offspring: 4 };
        let selection = comp.select_offspring(&population, &mut state);
        assert_eq!(selection.len(), comp.offspring as usize);
    }
}

/// Selects `offspring` solutions using exponential ranking.
///
/// Solutions can be selected multiple times in a single iteration.
///
/// See [https://tik-old.ee.ethz.ch/file/6c0e384dceb283cd4301339a895b72b8/TIK-Report11.pdf](https://tik-old.ee.ethz.ch/file/6c0e384dceb283cd4301339a895b72b8/TIK-Report11.pdf) for details.
#[derive(Serialize, Deserialize)]
pub struct ExponentialRank {
    /// Offspring per iteration.
    pub offspring: u32,
    /// Base of exponent within (0,1).
    pub base: f64,
}
impl ExponentialRank {
    pub fn new<P: SingleObjectiveProblem>(offspring: u32, base: f64) -> Box<dyn Component<P>> {
        Box::new(Selector(Self { offspring, base }))
    }
}
impl<P: SingleObjectiveProblem> Selection<P> for ExponentialRank {
    fn select_offspring<'p>(
        &self,
        population: &'p [Individual<P>],
        state: &mut State,
    ) -> Vec<&'p Individual<P>> {
        let rng = state.random_mut();
        let ranking = get_ranking(population);
        // Weight ranking by exponential equation, worst has smallest weight
        let weights: Vec<f64> = ranking
            .iter()
            .map(|i| {
                (self.base - 1.0) / (self.base.powi(population.len() as i32) - 1.0)
                    * (self.base.powi((population.len() - i) as i32))
            })
            .collect();
        let wheel = WeightedIndex::new(&weights).unwrap();
        let mut selection = Vec::new();
        for _ in 0..self.offspring {
            selection.push(&population[wheel.sample(rng)]);
        }
        assert_eq!(selection.len(), self.offspring as usize);
        selection
    }
}
#[cfg(test)]
mod exponential_rank {
    use crate::framework::Random;
    use crate::testing::*;

    use super::*;

    #[test]
    fn selects_right_number_of_children() {
        let mut state = State::new_root();
        state.insert(Random::testing());
        let population = new_test_population(&[1.0, 2.0, 3.0]);
        let comp = ExponentialRank {
            offspring: 4,
            base: 0.5,
        };
        let selection = comp.select_offspring(&population, &mut state);
        assert_eq!(selection.len(), comp.offspring as usize);
    }
}
