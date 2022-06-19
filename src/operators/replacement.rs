//! Replacement methods

use crate::framework::common_state::Population;
use crate::{
    framework::{components::*, Individual, State},
    problems::Problem,
};
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};

/// Specialized component trait to replace the population with the child population,
/// typically generated by a [Selection] component.
/// Merges the two topmost populations on the stack.
///
/// # Implementing [Component]
///
/// Types implementing this trait can implement [Component] by wrapping the type in a [Replacer].
pub trait Replacement {
    fn replace_population(
        &self,
        parents: &mut Vec<Individual>,
        offspring: &mut Vec<Individual>,
        state: &mut State,
    );
}

#[derive(serde::Serialize)]
pub struct Replacer<T>(pub T);

impl<T, P> Component<P> for Replacer<T>
where
    P: Problem,
    T: AnyComponent + Replacement + Serialize,
{
    fn execute(&self, _problem: &P, state: &mut State) {
        let mut offspring = state.get_mut::<Population>().pop();
        let mut parents = state.get_mut::<Population>().pop();
        self.0
            .replace_population(&mut parents, &mut offspring, state);
        state.population_stack_mut().push(parents);
    }
}

/// Discards all individuals in the child population, keeping the parents unchanged.
#[derive(Serialize)]
pub struct Noop;
impl Noop {
    pub fn new<P: Problem>() -> Box<dyn Component<P>> {
        Box::new(Replacer(Self))
    }
}
impl Replacement for Noop {
    fn replace_population(
        &self,
        _parents: &mut Vec<Individual>,
        _offspring: &mut Vec<Individual>,
        _state: &mut State,
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
        Box::new(Replacer(Self {
            max_population_size,
        }))
    }
}
impl Replacement for MuPlusLambda {
    fn replace_population(
        &self,
        parents: &mut Vec<Individual>,
        offspring: &mut Vec<Individual>,
        _state: &mut State,
    ) {
        parents.append(offspring);
        parents.sort_unstable_by_key(Individual::fitness);
        parents.truncate(self.max_population_size as usize);
    }
}

#[cfg(test)]
mod mupluslambda {
    use crate::framework::State;
    use crate::operators::testing::{collect_population_fitness, new_test_population};

    use super::*;

    #[test]
    fn keeps_right_individuals() {
        let mut state = State::new_root();
        let comp = MuPlusLambda {
            max_population_size: 3,
        };
        let mut population = new_test_population(&[1.0, 3.0, 5.0]);
        let mut offspring = new_test_population(&[2.0, 6.0]);
        comp.replace_population(&mut population, &mut offspring, &mut state);
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
impl Generational {
    pub fn new<P: Problem>(max_population_size: u32) -> Box<dyn Component<P>> {
        Box::new(Replacer(Self {
            max_population_size,
        }))
    }
}
impl Replacement for Generational {
    fn replace_population(
        &self,
        parents: &mut Vec<Individual>,
        offspring: &mut Vec<Individual>,
        _state: &mut State,
    ) {
        parents.clear();
        parents.append(offspring);
        parents.truncate(self.max_population_size as usize);
    }
}

#[cfg(test)]
mod generational {
    use super::*;
    use crate::operators::testing::*;

    #[test]
    fn keeps_all_children() {
        let mut state = State::new_root();
        let comp = Generational {
            max_population_size: 5,
        };
        let mut population = new_test_population(&[1.0, 3.0, 5.0, 6.0, 7.0]);
        let mut offspring = new_test_population(&[2.0, 4.0, 8.0, 9.0, 10.0]);
        comp.replace_population(&mut population, &mut offspring, &mut state);
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
    fn replace_population(
        &self,
        parents: &mut Vec<Individual>,
        offspring: &mut Vec<Individual>,
        state: &mut State,
    ) {
        parents.append(offspring);
        parents.shuffle(state.random_mut());
        parents.truncate(self.max_population_size as usize);
    }
}

#[cfg(test)]
mod random_replacement {
    use super::*;
    use crate::operators::testing::*;
    use crate::random::Random;

    #[test]
    fn keeps_right_amount_of_children() {
        let mut state = State::new_root();
        state.insert(Random::testing());
        let comp = RandomReplacement {
            max_population_size: 5,
        };
        let mut population = new_test_population(&[1.0, 3.0, 5.0, 6.0, 7.0]);
        let mut offspring = new_test_population(&[2.0, 4.0, 8.0, 9.0, 10.0]);
        comp.replace_population(&mut population, &mut offspring, &mut state);
        let population = collect_population_fitness(&population);
        assert_eq!(population.len(), comp.max_population_size as usize);
    }
}
