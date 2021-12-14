//! Selection methods

use rand::{
    distributions::{Distribution, WeightedIndex},
    seq::SliceRandom,
    Rng,
};
use serde::{Deserialize, Serialize};

use crate::{
    framework::{components::*, Individual, State},
    random::Random,
};

/// Selects all individuals once.
#[derive(Serialize, Deserialize)]
pub struct All;
impl Selection for All {
    fn select<'p>(
        &self,
        _state: &mut State,
        _rng: &mut Random,
        population: &'p [Individual],
        selection: &mut Vec<&'p Individual>,
    ) {
        selection.extend(population);
    }
}

/// Selects no individual.
#[derive(Serialize, Deserialize)]
pub struct None;
impl Selection for None {
    fn select<'p>(
        &self,
        _state: &mut State,
        _rng: &mut Random,
        _population: &'p [Individual],
        _selection: &mut Vec<&'p Individual>,
    ) {
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
impl Selection for FullyRandom {
    fn select<'p>(
        &self,
        _state: &mut State,
        rng: &mut Random,
        population: &'p [Individual],
        selection: &mut Vec<&'p Individual>,
    ) {
        for _ in 0..self.offspring {
            selection.push(population.choose(rng).unwrap());
        }
    }
}
#[cfg(test)]
mod fully_random {
    use crate::operators::testing::new_test_population;

    use super::*;

    #[test]
    fn selects_right_number_of_children() {
        let mut state = State::new();
        let mut rng = Random::testing();
        let population = new_test_population(&[1.0, 2.0, 3.0]);
        let comp = FullyRandom { offspring: 4 };
        let mut selection = Vec::new();
        comp.select(&mut state, &mut rng, &population, &mut selection);
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
impl Selection for DeterministicFitnessProportional {
    fn select<'p>(
        &self,
        _state: &mut State,
        _rng: &mut Random,
        population: &'p [Individual],
        selection: &mut Vec<&'p Individual>,
    ) {
        #[rustfmt::skip]
        let best: f64 = population.iter().map(Individual::fitness).min().unwrap().into();
        #[rustfmt::skip]
        let worst: f64 = population.iter().map(Individual::fitness).max().unwrap().into();

        assert!(
            worst.is_finite(),
            "selection::FitnessProportional does not work with Inf fitness values"
        );

        for plant in population.iter() {
            let bonus: f64 = (plant.fitness().into() - worst) / (best - worst);
            let bonus_seeds = (self.max_offspring - self.min_offspring) as f64;
            let num_offspring = self.min_offspring
                + if bonus.is_nan() {
                    // best ≈ worst, thus we picked a generic value
                    (0.5 * bonus_seeds).floor() as u32
                } else {
                    (bonus * bonus_seeds).floor() as u32
                };

            for _ in 0..num_offspring {
                selection.push(plant);
            }
        }
    }
}
#[cfg(test)]
mod deterministic_fitness_proportional {
    use crate::operators::testing::{collect_population_fitness, new_test_population};

    use super::*;

    #[test]
    fn selects_right_children() {
        let comp = DeterministicFitnessProportional {
            min_offspring: 1,
            max_offspring: 3,
        };
        let population = new_test_population(&[1.0, 2.0, 3.0]);
        let mut rng = Random::testing();
        let mut selection = Vec::new();
        comp.select(&mut State::new(), &mut rng, &population, &mut selection);
        let selection = collect_population_fitness(&selection);

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
impl Selection for RouletteWheel {
    fn select<'p>(
        &self,
        _state: &mut State,
        rng: &mut Random,
        population: &'p [Individual],
        selection: &mut Vec<&'p Individual>,
    ) {
        #[rustfmt::skip]
        let total: f64 = population.iter().map(|i| i.fitness().into()).sum();
        let weights: Vec<f64> = population
            .iter()
            .map(|f| f.fitness().into() / total)
            .collect();
        let weights_min_total: f64 = weights.iter().map(|w| 1.0 / w).sum();
        let weights_min: Vec<f64> = weights
            .iter()
            .map(|w| (1.0 / w) / weights_min_total)
            .collect();
        let wheel = WeightedIndex::new(weights_min).unwrap();
        for _ in 0..self.offspring {
            selection.push(&population[wheel.sample(rng)]);
        }
    }
}
#[cfg(test)]
mod roulette_wheel {
    use crate::operators::testing::new_test_population;

    use super::*;

    #[test]
    fn selects_right_number_of_children() {
        let mut state = State::new();
        let mut rng = Random::testing();
        let population = new_test_population(&[1.0, 2.0, 3.0]);
        let comp = RouletteWheel { offspring: 4 };
        let mut selection = Vec::new();
        comp.select(&mut state, &mut rng, &population, &mut selection);
        assert_eq!(selection.len(), comp.offspring as usize);
    }
}

/// Selects `offspring` solutions using stochastic universal sampling.
///
/// Solutions can be selected multiple times in a single iteration.
#[derive(Serialize, Deserialize)]
pub struct StochasticUniversalSampling {
    /// Offspring per iteration.
    pub offspring: u32,
}
impl Selection for StochasticUniversalSampling {
    fn select<'p>(
        &self,
        _state: &mut State,
        rng: &mut Random,
        population: &'p [Individual],
        selection: &mut Vec<&'p Individual>,
    ) {
        #[rustfmt::skip]
        let total: f64 = population.iter().map(|i| i.fitness().into()).sum();
        let weights: Vec<f64> = population
            .iter()
            .map(|f| f.fitness().into() / total)
            .collect();
        let weights_min_total: f64 = weights.iter().map(|w| 1.0 / w).sum();
        let weights_min: Vec<f64> = weights
            .iter()
            .map(|w| (1.0 / w) / weights_min_total)
            .collect();

        let gap = 1.0 / self.offspring as f64;
        let start = rng.gen_range(0.0..gap);
        let mut distance = start * gap;

        let mut sum_weights = weights_min[0];
        let mut i: usize = 0;
        while distance < 1.0 {
            while sum_weights < distance {
                i += 1;
                sum_weights += weights_min[i];
            }
            selection.push(&population[i]);
            distance += gap;
        }
    }
}
#[cfg(test)]
mod stochastic_universal_sampling {
    use crate::operators::testing::new_test_population;

    use super::*;

    #[test]
    fn selects_right_number_of_children() {
        let mut state = State::new();
        let mut rng = Random::testing();
        let population = new_test_population(&[1.0, 2.0, 3.0]);
        let comp = StochasticUniversalSampling { offspring: 4 };
        let mut selection = Vec::new();
        comp.select(&mut state, &mut rng, &population, &mut selection);
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
impl Selection for Tournament {
    fn select<'p>(
        &self,
        _state: &mut State,
        rng: &mut Random,
        population: &'p [Individual],
        selection: &mut Vec<&'p Individual>,
    ) {
        assert!(population.len() >= self.size as usize);
        for _ in 0..self.offspring {
            let mut tournament: Vec<&Individual> = population
                .choose_multiple(rng, self.size as usize)
                .collect();
            tournament.sort_by(|x, y| {
                (y.fitness().into())
                    .partial_cmp(&(x.fitness().into()))
                    .unwrap()
            });
            selection.push(tournament[0]);
        }
    }
}
#[cfg(test)]
mod tournament {
    use crate::operators::testing::new_test_population;

    use super::*;

    #[test]
    fn selects_right_number_of_children() {
        let mut state = State::new();
        let mut rng = Random::testing();
        let population = new_test_population(&[1.0, 2.0, 3.0]);
        let comp = Tournament {
            offspring: 4,
            size: 2,
        };
        let mut selection = Vec::new();
        comp.select(&mut state, &mut rng, &population, &mut selection);
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
impl Selection for LinearRank {
    fn select<'p>(
        &self,
        _state: &mut State,
        rng: &mut Random,
        population: &'p [Individual],
        selection: &mut Vec<&'p Individual>,
    ) {
        let mut weight_pos: Vec<(usize, f64)> = population
            .iter()
            .enumerate()
            .map(|(i, f)| (i, f.fitness().into()))
            .collect();

        weight_pos.sort_by(|a, b| (b.1).partial_cmp(&a.1).unwrap());
        let weights: Vec<usize> = weight_pos.iter().enumerate().map(|(i, _k)| 1 + i).collect();

        let wheel = WeightedIndex::new(&weights).unwrap();
        for _ in 0..self.offspring {
            let position = (weight_pos[wheel.sample(rng)]).0;
            selection.push(&population[position]);
        }
    }
}
#[cfg(test)]
mod linear_rank {
    use crate::operators::testing::new_test_population;

    use super::*;

    #[test]
    fn selects_right_number_of_children() {
        let mut state = State::new();
        let mut rng = Random::testing();
        let population = new_test_population(&[1.0, 2.0, 3.0]);
        let comp = LinearRank { offspring: 4 };
        let mut selection = Vec::new();
        comp.select(&mut state, &mut rng, &population, &mut selection);
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
impl Selection for ExponentialRank {
    fn select<'p>(
        &self,
        _state: &mut State,
        rng: &mut Random,
        population: &'p [Individual],
        selection: &mut Vec<&'p Individual>,
    ) {
        let mut ranking: Vec<(usize, f64)> = population
            .iter()
            .enumerate()
            .map(|(i, f)| (i, f.fitness().into()))
            .collect();

        ranking.sort_by(|a, b| (b.1).partial_cmp(&a.1).unwrap());

        let weights: Vec<f64> = ranking
            .iter()
            .enumerate()
            .map(|(i, _k)| {
                (self.base - 1.0) / (self.base.powi(population.len() as i32) - 1.0)
                    * (self.base.powi((population.len() - i) as i32))
            })
            .collect();

        let wheel = WeightedIndex::new(&weights).unwrap();
        for _ in 0..self.offspring {
            let position = (ranking[wheel.sample(rng)]).0;
            selection.push(&population[position]);
        }
    }
}
#[cfg(test)]
mod exponential_rank {
    use crate::operators::testing::new_test_population;

    use super::*;

    #[test]
    fn selects_right_number_of_children() {
        let mut state = State::new();
        let mut rng = Random::testing();
        let population = new_test_population(&[1.0, 2.0, 3.0]);
        let comp = ExponentialRank {
            offspring: 4,
            base: 0.5,
        };
        let mut selection = Vec::new();
        comp.select(&mut state, &mut rng, &population, &mut selection);
        assert_eq!(selection.len(), comp.offspring as usize);
    }
}
