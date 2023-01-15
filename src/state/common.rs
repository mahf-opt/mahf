//! Common state used in most heuristics.

use super::CustomState;
use crate::problems::Problem;
use crate::{
    framework::Individual,
    problems::{MultiObjectiveProblem, SingleObjectiveProblem},
};
use better_any::Tid;
use derive_deref::{Deref, DerefMut};
use serde::Serialize;

/// Saves the [Individual] with best objective value.
///
/// To insert and update this state, use the [UpdateBestIndividual][crate::components::evaluation::UpdateBestIndividual] component.
#[derive(Deref, DerefMut, Tid)]
pub struct BestIndividual<P: SingleObjectiveProblem + 'static>(pub Option<Individual<P>>);
impl<P: SingleObjectiveProblem> BestIndividual<P> {
    pub fn replace_if_better(&mut self, candidate: &Individual<P>) -> bool {
        if let Some(individual) = &mut self.0 {
            if candidate.objective() < individual.objective() {
                *individual = candidate.clone();
                true
            } else {
                false
            }
        } else {
            self.0 = Some(candidate.clone());
            true
        }
    }
}
impl<P: SingleObjectiveProblem> CustomState<'_> for BestIndividual<P> {}

impl<P: SingleObjectiveProblem> Default for BestIndividual<P> {
    fn default() -> Self {
        Self(None::<Individual<P>>)
    }
}

#[derive(Deref, DerefMut, Clone, Serialize, Tid)]
pub struct Evaluations(pub u32);
impl CustomState<'_> for Evaluations {}

#[derive(Deref, DerefMut, Clone, Serialize, Tid)]
pub struct Iterations(pub u32);
impl CustomState<'_> for Iterations {}

#[derive(Deref, DerefMut, Clone, Serialize, Tid)]
pub struct Progress(pub f64);
impl CustomState<'_> for Progress {}

/// Saves non-pareto-dominated [Individual]'s.
///
/// To insert and update this state, use the [UpdateParetoFront][crate::components::evaluation::UpdateParetoFront] component.
#[derive(Deref, DerefMut, Tid)]
pub struct ParetoFront<P: MultiObjectiveProblem + 'static>(Vec<Individual<P>>);
impl<P: MultiObjectiveProblem> ParetoFront<P> {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn update(&mut self, individual: &Individual<P>) {
        if !individual.is_evaluated() {
            return;
        }

        let objective = individual.objective();
        let _comparisons: Vec<_> = self
            .front()
            .iter()
            .map(|other| objective.partial_cmp(other.objective()))
            .collect();

        todo!("Finish implementation.");
    }

    pub fn update_multiple(&mut self, population: &[Individual<P>]) {
        for individual in population {
            self.update(individual);
        }
    }

    pub fn front(&self) -> &[Individual<P>] {
        &self.0
    }
}
impl<P: MultiObjectiveProblem> CustomState<'_> for ParetoFront<P> {}

impl<P: MultiObjectiveProblem> Default for ParetoFront<P> {
    fn default() -> Self {
        Self(Vec::new())
    }
}

#[derive(Deref, DerefMut, Tid)]
pub struct Loop(pub bool);
impl CustomState<'_> for Loop {}

#[derive(Default, Tid)]
pub struct Population<P: Problem + 'static> {
    stack: Vec<Vec<Individual<P>>>,
}
impl<P: Problem> CustomState<'_> for Population<P> {}
impl<P: Problem> Population<P> {
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    pub fn current(&self) -> &[Individual<P>] {
        self.stack.last().unwrap()
    }

    pub fn current_mut(&mut self) -> &mut Vec<Individual<P>> {
        self.stack.last_mut().unwrap()
    }

    pub fn push(&mut self, population: Vec<Individual<P>>) {
        self.stack.push(population);
    }

    pub fn pop(&mut self) -> Vec<Individual<P>> {
        self.stack.pop().unwrap()
    }

    pub fn peek(&self, index: usize) -> &[Individual<P>] {
        let n = self.stack.len();
        &self.stack[n - 1 - index]
    }

    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
}
