//! Replacement methods

use crate::heuristic::{components::*, Individual, State};

/// Always keeps the fittest individuals.
pub struct Fittest {
    /// Limits the population growth.
    pub max_population_size: u32,
}
impl Replacement for Fittest {
    fn replace(
        &mut self,
        _state: &mut State,
        population: &mut Vec<Individual>,
        offspring: &mut Vec<Individual>,
    ) {
        population.extend(offspring.drain(..));
        population.sort_unstable_by_key(Individual::fitness);
        population.truncate(self.max_population_size as usize);
    }
}
