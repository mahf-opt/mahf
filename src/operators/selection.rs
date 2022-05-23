//! Selection methods

use rand::{
    distributions::{Distribution, WeightedIndex},
    seq::SliceRandom,
    Rng,
};
use serde::{Deserialize, Serialize};

use crate::{
    framework::{
        components::*,
        specializations::{Selection, Selector},
        Individual, State,
    },
    problems::Problem,
};

#[cfg(test)]
macro_rules! impl_selects_right_number_of_children {
    ($component: expr) => {
        #[test]
        fn selects_right_number_of_children() {
            let mut state = State::new_root();
            state.insert(Random::testing());
            let population = new_test_population(&[1.0, 2.0, 3.0]);
            let comp = $component;
            let selection = comp.select_offspring(&population, &mut state);
            assert_eq!(selection.len(), comp.offspring as usize);
        }
    };
}

/// Helper function to obtain minimum and maximum fitness ranges for normalization.
fn get_fitness_range(population: &[Individual]) -> (f64, f64) {
    let best = population
        .iter()
        .map(Individual::fitness)
        .min()
        .unwrap()
        .into();
    let worst = population
        .iter()
        .map(Individual::fitness)
        .max()
        .unwrap()
        .into();
    (worst, best)
}

/// Helper function to calculate normalized fitness weights from the population,
/// where individuals with lower fitness get more weight.
fn get_minimizing_weights(population: &[Individual]) -> Vec<f64> {
    let (worst, best) = get_fitness_range(population);
    assert!(
        worst.is_finite(),
        "weighting does not work with Inf fitness values"
    );
    // Normalize fitness values
    let normalized: Vec<f64> = population
        .iter()
        .map(|i| (i.fitness().into() - best) / (worst - best))
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
/// The elements are the indices of the individuals, ordered by fitness.
/// The first element is therefore the index of the best individual in the population.
fn get_ranking(population: &[Individual]) -> Vec<usize> {
    // Get positions in population and fitness of all individuals
    let mut weight_pos: Vec<(usize, f64)> = population
        .iter()
        .enumerate()
        .map(|(i, f)| (i, f.fitness().into()))
        .collect();

    // Sort those by their fitness values from worst to best
    weight_pos.sort_by(|a, b| (b.1).partial_cmp(&a.1).unwrap());

    // Discard the fitness
    weight_pos.into_iter().map(|(i, _f)| i).collect()
}

/// Selects all individuals once.
#[derive(Serialize, Deserialize)]
pub struct All;
impl All {
    pub fn new<P: Problem>() -> Box<dyn Component<P>> {
        Box::new(Selector(Self))
    }
}
impl Selection for All {
    fn select_offspring<'p>(
        &self,
        population: &'p [Individual],
        _state: &mut State,
    ) -> Vec<&'p Individual> {
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
impl Selection for None {
    fn select_offspring<'p>(
        &self,
        _population: &'p [Individual],
        _state: &mut State,
    ) -> Vec<&'p Individual> {
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
impl Selection for DuplicateSingle {
    fn select_offspring<'p>(
        &self,
        population: &'p [Individual],
        _state: &mut State,
    ) -> Vec<&'p Individual> {
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
    use crate::operators::testing::new_test_population;

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
impl Selection for FullyRandom {
    fn select_offspring<'p>(
        &self,
        population: &'p [Individual],
        state: &mut State,
    ) -> Vec<&'p Individual> {
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
    use crate::operators::testing::new_test_population;
    use crate::random::Random;

    use super::*;

    impl_selects_right_number_of_children!(FullyRandom { offspring: 4 });
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
    pub fn new<P: Problem>(min_offspring: u32, max_offspring: u32) -> Box<dyn Component<P>> {
        Box::new(Selector(Self {
            min_offspring,
            max_offspring,
        }))
    }
}

impl Selection for DeterministicFitnessProportional {
    fn select_offspring<'p>(
        &self,
        population: &'p [Individual],
        _state: &mut State,
    ) -> Vec<&'p Individual> {
        let (worst, best) = get_fitness_range(population);

        assert!(
            worst.is_finite(),
            "selection::DeterministicFitnessProportional does not work with Inf fitness values"
        );

        let mut selection = Vec::new();

        for ind in population.iter() {
            let bonus: f64 = (ind.fitness().into() - worst) / (best - worst);
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
    use crate::operators::testing::new_test_population;
    use crate::random::Random;

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
        let selection: Vec<f64> = selection.into_iter().map(|i| i.fitness().into()).collect();

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
    pub fn new<P: Problem>(offspring: u32) -> Box<dyn Component<P>> {
        Box::new(Selector(Self { offspring }))
    }
}
impl Selection for RouletteWheel {
    fn select_offspring<'p>(
        &self,
        population: &'p [Individual],
        state: &mut State,
    ) -> Vec<&'p Individual> {
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
    use crate::operators::testing::new_test_population;
    use crate::random::Random;

    use super::*;

    impl_selects_right_number_of_children!(RouletteWheel { offspring: 4 });
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
impl Selection for StochasticUniversalSampling {
    fn select_offspring<'p>(
        &self,
        population: &'p [Individual],
        state: &mut State,
    ) -> Vec<&'p Individual> {
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
    use crate::operators::testing::new_test_population;
    use crate::random::Random;

    use super::*;

    impl_selects_right_number_of_children!(FullyRandom { offspring: 4 });
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
impl Selection for Tournament {
    fn select_offspring<'p>(
        &self,
        population: &'p [Individual],
        state: &mut State,
    ) -> Vec<&'p Individual> {
        assert!(population.len() >= self.size as usize);
        let rng = state.random_mut();
        let mut selection = Vec::new();
        // For each individual
        for _ in 0..self.offspring {
            // choose size competitors in tournament
            let mut tournament: Vec<&Individual> = population
                .choose_multiple(rng, self.size as usize)
                .collect();
            // and evaluate them against each other, placing the winner first
            tournament.sort_by(|x, y| {
                (y.fitness().into())
                    .partial_cmp(&(x.fitness().into()))
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
    use crate::operators::testing::new_test_population;
    use crate::random::Random;

    use super::*;

    impl_selects_right_number_of_children!(Tournament {
        offspring: 4,
        size: 2,
    });
}

/// Selects `offspring` solutions using linear ranking.
///
/// Solutions can be selected multiple times in a single iteration.
#[derive(Serialize, Deserialize)]
pub struct LinearRank {
    /// Offspring per iteration.
    pub offspring: u32,
}
impl Selection for LinearRank {
    fn select_offspring<'p>(
        &self,
        population: &'p [Individual],
        state: &mut State,
    ) -> Vec<&'p Individual> {
        let rng = state.random_mut();
        // Get positions in population and fitness of all individuals
        let weight_pos = get_ranking(population);
        // Weights are new positions after sorting by fitness, worst has smallest weight
        let weights: Vec<usize> = weight_pos.iter().enumerate().map(|(i, _k)| 1 + i).collect();
        let wheel = WeightedIndex::new(&weights).unwrap();
        let mut selection = Vec::new();
        for _ in 0..self.offspring {
            // Sample individuals by their ranks but select them from the initial population by the
            // positions marked in weight_pos
            let position = weight_pos[wheel.sample(rng)];
            selection.push(&population[position]);
        }
        assert_eq!(selection.len(), self.offspring as usize);
        selection
    }
}
#[cfg(test)]
mod linear_rank {
    use crate::operators::testing::new_test_population;
    use crate::random::Random;

    use super::*;

    impl_selects_right_number_of_children!(LinearRank { offspring: 4 });
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
impl Selection for ExponentialRank {
    fn select_offspring<'p>(
        &self,
        population: &'p [Individual],
        state: &mut State,
    ) -> Vec<&'p Individual> {
        let rng = state.random_mut();
        let ranking = get_ranking(population);
        // Weight ranking by exponential equation, worst has smallest weight
        let weights: Vec<f64> = ranking
            .iter()
            .enumerate()
            .map(|(i, _k)| {
                (self.base - 1.0) / (self.base.powi(population.len() as i32) - 1.0)
                    * (self.base.powi((population.len() - i) as i32))
            })
            .collect();
        let wheel = WeightedIndex::new(&weights).unwrap();
        let mut selection = Vec::new();
        for _ in 0..self.offspring {
            let position = ranking[wheel.sample(rng)];
            selection.push(&population[position]);
        }
        assert_eq!(selection.len(), self.offspring as usize);
        selection
    }
}
#[cfg(test)]
mod exponential_rank {
    use crate::operators::testing::new_test_population;
    use crate::random::Random;

    use super::*;

    impl_selects_right_number_of_children!(ExponentialRank {
        offspring: 4,
        base: 0.5,
    });
}
