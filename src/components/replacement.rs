//! Replacement methods

use crate::{
    framework::{components::*, Individual},
    problems::{Problem, SingleObjectiveProblem},
    state::State,
};
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};

/// Specialized component trait to replace the population with the child population,
/// typically generated by a [Selection](crate::components::selection::Selection) component.
/// Merges the two topmost populations on the stack.
///
/// # Implementing [Component]
///
/// Types implementing this trait can implement [Component] by wrapping the type in a [Replacer].
pub trait Replacement<P: Problem> {
    fn replace_population(
        &self,
        parents: &mut Vec<Individual<P>>,
        offspring: &mut Vec<Individual<P>>,
        state: &mut State<P>,
    );
}

#[derive(serde::Serialize, Clone)]
pub struct Replacer<T: Clone>(pub T);

impl<T, P> Component<P> for Replacer<T>
where
    P: Problem,
    T: AnyComponent + Replacement<P> + Serialize + Clone,
{
    fn execute(&self, _problem: &P, state: &mut State<P>) {
        let mut offspring = state.populations_mut().pop();
        let mut parents = state.populations_mut().pop();
        self.0
            .replace_population(&mut parents, &mut offspring, state);
        state.populations_mut().push(parents);
    }
}

/// Discards all individuals in the child population, keeping the parents unchanged.
#[derive(Serialize, Deserialize, Clone)]
pub struct Noop;
impl Noop {
    pub fn new<P: Problem>() -> Box<dyn Component<P>> {
        Box::new(Replacer(Self))
    }
}
impl<P: Problem> Replacement<P> for Noop {
    fn replace_population(
        &self,
        _parents: &mut Vec<Individual<P>>,
        _offspring: &mut Vec<Individual<P>>,
        _state: &mut State<P>,
    ) {
    }
}

/// Always keeps the fittest individuals.
#[derive(Serialize, Deserialize, Clone)]
pub struct MuPlusLambda {
    /// Limits the population growth.
    pub max_population_size: u32,
}
impl MuPlusLambda {
    pub fn new<P: SingleObjectiveProblem>(max_population_size: u32) -> Box<dyn Component<P>> {
        Box::new(Replacer(Self {
            max_population_size,
        }))
    }
}
impl<P: SingleObjectiveProblem> Replacement<P> for MuPlusLambda {
    fn replace_population(
        &self,
        parents: &mut Vec<Individual<P>>,
        offspring: &mut Vec<Individual<P>>,
        _state: &mut State<P>,
    ) {
        parents.append(offspring);
        parents.sort_unstable_by_key(|i| *i.objective());
        parents.truncate(self.max_population_size as usize);
    }
}

#[cfg(test)]
mod mupluslambda {
    use crate::state::State;
    use crate::testing::*;

    use super::*;

    #[test]
    fn keeps_right_individuals() {
        let mut state = State::new();
        let comp = MuPlusLambda {
            max_population_size: 3,
        };
        let mut population = new_test_population(&[1.0, 3.0, 5.0]);
        let mut offspring = new_test_population(&[2.0, 6.0]);
        comp.replace_population(&mut population, &mut offspring, &mut state);
        let population = collect_population_objective_values(&population);
        assert_eq!(population.len(), comp.max_population_size as usize);
        assert_eq!(population, vec![1.0, 2.0, 3.0]);
    }
}

/// Always keeps the children.
#[derive(Serialize, Deserialize, Clone)]
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
impl<P: Problem> Replacement<P> for Generational {
    fn replace_population(
        &self,
        parents: &mut Vec<Individual<P>>,
        offspring: &mut Vec<Individual<P>>,
        _state: &mut State<P>,
    ) {
        parents.clear();
        parents.append(offspring);
        parents.truncate(self.max_population_size as usize);
    }
}

#[cfg(test)]
mod generational {
    use super::*;
    use crate::testing::*;

    #[test]
    fn keeps_all_children() {
        let mut state = State::new();
        let comp = Generational {
            max_population_size: 5,
        };
        let mut population = new_test_population(&[1.0, 3.0, 5.0, 6.0, 7.0]);
        let mut offspring = new_test_population(&[2.0, 4.0, 8.0, 9.0, 10.0]);
        comp.replace_population(&mut population, &mut offspring, &mut state);
        let population = collect_population_objective_values(&population);
        assert_eq!(population.len(), comp.max_population_size as usize);
        assert_eq!(population, vec![2.0, 4.0, 8.0, 9.0, 10.0]);
    }
}

/// Keeps random individuals from parents and children.
#[derive(Serialize, Deserialize, Clone)]
pub struct RandomReplacement {
    /// Limits the population growth.
    pub max_population_size: u32,
}
impl RandomReplacement {
    pub fn new<P: Problem>(max_population_size: u32) -> Box<dyn Component<P>> {
        Box::new(Replacer(Self {
            max_population_size,
        }))
    }
}
impl<P: Problem> Replacement<P> for RandomReplacement {
    fn replace_population(
        &self,
        parents: &mut Vec<Individual<P>>,
        offspring: &mut Vec<Individual<P>>,
        state: &mut State<P>,
    ) {
        parents.append(offspring);
        parents.shuffle(state.random_mut());
        parents.truncate(self.max_population_size as usize);
    }
}

#[cfg(test)]
mod random_replacement {
    use super::*;
    use crate::framework::Random;
    use crate::testing::*;

    #[test]
    fn keeps_right_amount_of_children() {
        let mut state = State::new();
        state.insert(Random::testing());
        let comp = RandomReplacement {
            max_population_size: 5,
        };
        let mut population = new_test_population(&[1.0, 3.0, 5.0, 6.0, 7.0]);
        let mut offspring = new_test_population(&[2.0, 4.0, 8.0, 9.0, 10.0]);
        comp.replace_population(&mut population, &mut offspring, &mut state);
        let population = collect_population_objective_values(&population);
        assert_eq!(population.len(), comp.max_population_size as usize);
    }
}

/// Keeps the better individual from parent and offspring at the same index.
#[derive(Serialize, Deserialize, Clone)]
pub struct IndividualPlus;
impl IndividualPlus {
    pub fn new<P: SingleObjectiveProblem>() -> Box<dyn Component<P>> {
        Box::new(Replacer(Self))
    }
}
impl<P: SingleObjectiveProblem> Replacement<P> for IndividualPlus {
    fn replace_population(
        &self,
        parents: &mut Vec<Individual<P>>,
        offspring: &mut Vec<Individual<P>>,
        _state: &mut State<P>,
    ) {
        assert_eq!(parents.len(), offspring.len());

        for (parent, offspring) in parents.iter_mut().zip(offspring.drain(..)) {
            if parent.objective() > offspring.objective() {
                *parent = offspring;
            }
        }
    }
}
#[cfg(test)]
mod greedy_index {
    use super::*;
    use crate::framework::Random;
    use crate::testing::*;

    #[test]
    fn keeps_right_amount_of_children() {
        let mut state = State::new();
        state.insert(Random::testing());
        let comp = IndividualPlus;
        let mut population = new_test_population(&[1.0, 3.0, 5.0, 6.0, 7.0]);
        let mut offspring = new_test_population(&[2.0, 4.0, 8.0, 9.0, 10.0]);
        let offspring_len = offspring.len();

        comp.replace_population(&mut population, &mut offspring, &mut state);
        assert_eq!(population.len(), offspring_len);
    }
}
