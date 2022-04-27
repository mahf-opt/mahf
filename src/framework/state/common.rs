use super::{CustomState, State};
use crate::framework::{Fitness, Individual};
use derive_deref::{Deref, DerefMut};
use serde::Serialize;

pub fn default(state: &mut State) {
    state.insert(Population::new());
    state.insert(Evaluations(0));
    state.insert(Iterations(0));
    state.insert(Progress(0.0));
    state.insert(BestFitness(Fitness::default()));
}

#[derive(Deref, DerefMut)]
pub struct BestIndividual(pub Individual);
impl CustomState for BestIndividual {}

#[derive(Deref, DerefMut, Clone, Serialize)]
pub struct Evaluations(pub u32);
impl CustomState for Evaluations {}

#[derive(Deref, DerefMut, Clone, Serialize)]
pub struct Iterations(pub u32);
impl CustomState for Iterations {}

#[derive(Deref, DerefMut, Clone, Serialize)]
pub struct Progress(pub f64);
impl CustomState for Progress {}

#[derive(Deref, DerefMut, Clone, Serialize)]
pub struct BestFitness(pub Fitness);
impl CustomState for BestFitness {}

#[derive(Deref, DerefMut)]
pub struct Loop(pub bool);
impl CustomState for Loop {}

#[derive(Default)]
pub struct Population {
    stack: Vec<Vec<Individual>>,
}
impl CustomState for Population {}
impl Population {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn current(&self) -> &[Individual] {
        self.stack.last().unwrap()
    }

    pub fn current_mut(&mut self) -> &mut Vec<Individual> {
        self.stack.last_mut().unwrap()
    }

    pub fn push(&mut self, population: Vec<Individual>) {
        self.stack.push(population);
    }

    pub fn pop(&mut self) -> Vec<Individual> {
        self.stack.pop().unwrap()
    }
}
