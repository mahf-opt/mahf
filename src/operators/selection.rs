//! Selection methods

use crate::{
    heuristic::{components::*, Individual, State},
    random::Random,
};
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};

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
    use super::*;
    use crate::operators::testing::new_test_population;

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
pub struct FitnessProportional {
    /// Minimum offspring per individual per iteration
    pub min_offspring: u32,
    /// Maximum offspring per individual per iteration
    pub max_offspring: u32,
}
impl Selection for FitnessProportional {
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
mod fitness_proportional {
    use super::*;
    use crate::operators::testing::{collect_population_fitness, new_test_population};

    #[test]
    fn selects_right_children() {
        let comp = FitnessProportional {
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
