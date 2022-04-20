//! Replacement methods

use crate::{
    framework::{
        common_state::Population, components::*, legacy::components::*, Individual, State,
    },
    problems::Problem,
    random::Random,
};
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Noop;
impl Noop {
    pub fn new<P: Problem>() -> Box<dyn Component<P>> {
        Box::new(Replacer(Self))
    }
}
impl Replacement for Noop {
    fn replace(
        &self,
        _state: &mut State,
        _rng: &mut Random,
        _population: &mut Vec<Individual>,
        _offspring: &mut Vec<Individual>,
    ) {
    }
}

/// Always keeps the fittest individuals.
#[derive(Serialize, Deserialize)]
pub struct MuPlusLambda {
    /// Limits the population growth.
    pub max_population_size: u32,
}
impl MuPlusLambda {
    pub fn new<P: Problem>(max_population_size: u32) -> Box<dyn Component<P>> {
        Box::new(Self {
            max_population_size,
        })
    }
}
impl<P: Problem> Component<P> for MuPlusLambda {
    fn initialize(&self, _problem: &P, state: &mut State) {
        state.require::<Population>();
    }

    fn execute(&self, _problem: &P, state: &mut State) {
        let population_stack = state.get_mut::<Population>();
        let offspring = &mut population_stack.pop();
        let population = population_stack.current_mut();

        population.append(offspring);
        population.sort_unstable_by_key(Individual::fitness);
        population.truncate(self.max_population_size as usize);
    }
}

#[cfg(test)]
mod mupluslambda {
    #[test]
    fn keeps_right_individuals() {
        // TODO: fix tests
        // let mut state = State::new_root();
        // let mut rng = Random::testing();
        // let comp = MuPlusLambda {
        //     max_population_size: 3,
        // };
        // let mut population = new_test_population(&[1.0, 3.0, 5.0]);
        // let mut offspring = new_test_population(&[2.0, 6.0]);
        // comp.replace(&mut state, &mut rng, &mut population, &mut offspring);
        // let population = collect_population_fitness(&population);
        // assert_eq!(population.len(), comp.max_population_size as usize);
        // assert_eq!(population, vec![1.0, 2.0, 3.0]);
    }
}

/// Always keeps the children.
#[derive(Serialize, Deserialize)]
pub struct Generational {
    /// Limits the population growth.
    pub max_population_size: u32,
}
impl Generational {
    pub fn new<P: Problem>(max_population_size: u32) -> Box<dyn Component<P>> {
        Box::new(Replacer(Self {
            max_population_size,
        }))
    }
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
        let mut state = State::new_root();
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
        let mut state = State::new_root();
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
