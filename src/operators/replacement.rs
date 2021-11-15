//! Replacement methods

use crate::{
    framework::{components::*, Individual, State},
    random::Random,
};
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};

/// Always keeps the fittest individuals.
#[derive(Serialize, Deserialize)]
pub struct Fittest {
    /// Limits the population growth.
    pub max_population_size: u32,
}
impl Replacement for Fittest {
    fn replace(
        &self,
        _state: &mut State,
        _rng: &mut Random,
        population: &mut Vec<Individual>,
        offspring: &mut Vec<Individual>,
    ) {
        population.append(offspring);
        population.sort_unstable_by_key(Individual::fitness);
        population.truncate(self.max_population_size as usize);
    }
}

#[cfg(test)]
mod fittest {
    use super::*;
    use crate::operators::testing::*;

    #[test]
    fn keeps_right_individuals() {
        let mut state = State::new();
        let mut rng = Random::testing();
        let comp = Fittest {
            max_population_size: 3,
        };
        let mut population = new_test_population(&[1.0, 3.0, 5.0]);
        let mut offspring = new_test_population(&[2.0, 6.0]);
        comp.replace(&mut state, &mut rng, &mut population, &mut offspring);
        let population = collect_population_fitness(&population);
        assert_eq!(population.len(), comp.max_population_size as usize);
        assert_eq!(population, vec![1.0, 2.0, 3.0]);
    }
}

/// Always keeps the children.
#[derive(Serialize, Deserialize)]
pub struct Generational {
    /// Limits the population growth.
    pub max_population_size: u32,
}
impl Replacement for Generational {
    fn replace(
        &self,
        _state: &mut State,
        _rng: &mut Random,
        population: &mut Vec<Individual>,
        offspring: &mut Vec<Individual>,
    ) {
        population.clear();
        population.append(offspring);
        population.truncate(self.max_population_size as usize);
    }
}

#[cfg(test)]
mod generational {
    use super::*;
    use crate::operators::testing::*;

    #[test]
    fn keeps_all_children() {
        let mut state = State::new();
        let mut rng = Random::testing();
        let comp = Generational {
            max_population_size: 5,
        };
        let mut population = new_test_population(&[1.0, 3.0, 5.0, 6.0, 7.0]);
        let mut offspring = new_test_population(&[2.0, 4.0, 8.0, 9.0, 10.0]);
        comp.replace(&mut state, &mut rng, &mut population, &mut offspring);
        let population = collect_population_fitness(&population);
        assert_eq!(population.len(), comp.max_population_size as usize);
        assert_eq!(population, vec![2.0, 4.0, 8.0, 9.0, 10.0]);
    }
}

/// Keeps random individuals from parents and children.
#[derive(Serialize, Deserialize)]
pub struct RandomReplacement {
    /// Limits the population growth.
    pub max_population_size: u32,
}
impl Replacement for RandomReplacement {
    fn replace(
        &self,
        _state: &mut State,
        rng: &mut Random,
        population: &mut Vec<Individual>,
        offspring: &mut Vec<Individual>,
    ) {
        population.append(offspring);
        population.shuffle(rng);
        population.truncate(self.max_population_size as usize);
    }
}

#[cfg(test)]
mod random_replacement {
    use super::*;
    use crate::operators::testing::*;

    #[test]
    fn keeps_right_amount_of_children() {
        let mut state = State::new();
        let mut rng = Random::testing();
        let comp = RandomReplacement {
            max_population_size: 5,
        };
        let mut population = new_test_population(&[1.0, 3.0, 5.0, 6.0, 7.0]);
        let mut offspring = new_test_population(&[2.0, 4.0, 8.0, 9.0, 10.0]);
        comp.replace(&mut state, &mut rng, &mut population, &mut offspring);
        let population = collect_population_fitness(&population);
        assert_eq!(population.len(), comp.max_population_size as usize);
    }
}
