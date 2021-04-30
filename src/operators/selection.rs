use crate::modular::{components::*, Individual, State};
use rand::seq::SliceRandom;

pub struct Es {
    /// Offspring per iteration
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
