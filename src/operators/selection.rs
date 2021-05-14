//! Selection methods

use crate::{
    heuristic::{components::*, Individual, State},
    random::Random,
};
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};

/// Selects `lambda` random solutions.
///
/// Solutions can be selected multiple times in a single iteration.
#[derive(Serialize, Deserialize)]
pub struct Es {
    /// Offspring per iteration.
    pub lambda: u32,
}
impl Selection for Es {
    fn select<'p>(
        &self,
        _state: &mut State,
        rng: &mut Random,
        population: &'p [Individual],
        selection: &mut Vec<&'p Individual>,
    ) {
        for _ in 0..self.lambda {
            selection.push(population.choose(rng).unwrap());
        }
    }
}
#[cfg(test)]
mod es {
    use super::*;
    use crate::operators::testing::new_test_population;

    #[test]
    fn selects_right_number_of_children() {
        let mut state = State::new();
        let mut rng = Random::test_rng();
        let population = new_test_population(&[1.0, 2.0, 3.0]);
        let comp = Es { lambda: 4 };
        let mut selection = Vec::new();
        comp.select(&mut state, &mut rng, &population, &mut selection);
        assert_eq!(selection.len(), comp.lambda as usize);
    }
}

/// Deterministically selects individuals as proposed for the IWO.
///
/// Each individual gets selected between `min_number_of_seeds` and
/// `max_number_of_seeds` times.
///
/// - The worst solution gets selected exactly `min_number_of_seeds` times.
/// - The best solution gets selected exactly `max_number_of_seeds` times.
/// - All solutions between them get selected based on the linear interpolation
///   between `min_number_of_seeds` and `max_number_of_seeds`.
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
pub struct Iwo {
    /// Minimum number of seeds per plant per iteration
    pub min_number_of_seeds: u32,
    /// Maximum number of seeds per plant per iteration
    pub max_number_of_seeds: u32,
}
impl Selection for Iwo {
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
            "selection::Iwo does not work with Inf fitness values"
        );

        for plant in population.iter() {
            let bonus: f64 = (plant.fitness().into() - worst) / (best - worst);
            let bonus_seeds = (self.max_number_of_seeds - self.min_number_of_seeds) as f64;
            let num_seeds = self.min_number_of_seeds
                + if bonus.is_nan() {
                    // best ≈ worst, thus we picked a generic value
                    (0.5 * bonus_seeds).floor() as u32
                } else {
                    (bonus * bonus_seeds).floor() as u32
                };

            for _ in 0..num_seeds {
                selection.push(plant);
            }
        }
    }
}
#[cfg(test)]
mod iwo {
    use super::*;
    use crate::operators::testing::{collect_population_fitness, new_test_population};

    #[test]
    fn selects_right_children() {
        let comp = Iwo {
            min_number_of_seeds: 1,
            max_number_of_seeds: 3,
        };
        let population = new_test_population(&[1.0, 2.0, 3.0]);
        let mut rng = Random::test_rng();
        let mut selection = Vec::new();
        comp.select(&mut State::new(), &mut rng, &population, &mut selection);
        let selection = collect_population_fitness(&selection);

        assert!(selection.len() > (comp.min_number_of_seeds as usize * population.len()));
        assert!(selection.len() < (comp.max_number_of_seeds as usize * population.len()));

        // I(1.0) should have 3 seed
        // I(2.0) should have (1 + 3/2) seeds
        // I(3.0) should have 1 seeds
        assert_eq!(selection, vec![1.0, 1.0, 1.0, 2.0, 2.0, 3.0]);
    }
}
