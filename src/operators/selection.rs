//! Selection methods

use crate::heuristic::{components::*, Individual, State};
use rand::seq::SliceRandom;

/// Selects `lambda` random solutions.
///
/// Solutions can be selected multiple times in a single iteration.
#[derive(serde::Serialize)]
pub struct Es {
    /// Offspring per iteration.
    pub lambda: u32,
}
impl Selection for Es {
    fn select<'p>(
        &mut self,
        _state: &mut State,
        population: &'p [Individual],
        selection: &mut Vec<&'p Individual>,
    ) {
        let rng = &mut rand::thread_rng();
        for _ in 0..self.lambda {
            selection.push(population.choose(rng).unwrap());
        }
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
#[derive(serde::Serialize)]
pub struct Iwo {
    /// Minimum number of seeds per plant per iteration
    pub min_number_of_seeds: u32,
    /// Maximum number of seeds per plant per iteration
    pub max_number_of_seeds: u32,
}
impl Selection for Iwo {
    fn select<'p>(
        &mut self,
        _state: &mut State,
        population: &'p [Individual],
        selection: &mut Vec<&'p Individual>,
    ) {
        #[rustfmt::skip]
        let best: f64 = population.iter().map(Individual::fitness).min().unwrap().into();
        #[rustfmt::skip]
        let worst: f64 = population.iter().map(Individual::fitness).max().unwrap().into();

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
            assert!(num_seeds <= self.max_number_of_seeds);

            for _ in 0..num_seeds {
                selection.push(plant);
            }
        }
    }
}
