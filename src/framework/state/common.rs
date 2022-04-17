use super::{CustomState, StateTree};
use crate::{
    framework::{Fitness, Individual},
    random::Random,
};

pub fn common_state(state: &mut StateTree) {
    state.insert(Evaluations(0));
    state.insert(Iterations(0));
    state.insert(Progress(0.0));
    state.insert(BestFitness(Fitness::default()));
}

pub struct Rng(pub Random);
impl CustomState for Rng {}

pub struct Population(pub Vec<Individual>);
impl CustomState for Population {}

pub struct BestIndividual(pub Individual);
impl CustomState for BestIndividual {}

pub struct Selection(pub Vec<Individual>);
impl CustomState for Selection {}

pub struct RawGeneration<E>(pub Vec<E>);
impl<E: 'static> CustomState for RawGeneration<E> {}

pub struct Offspring(pub Individual);
impl CustomState for Offspring {}

pub struct Evaluations(pub u32);
impl CustomState for Evaluations {}

pub struct Iterations(pub u32);
impl CustomState for Iterations {}

pub struct Progress(pub f64);
impl CustomState for Progress {}

pub struct BestFitness(pub Fitness);
impl CustomState for BestFitness {}

pub struct Loop(pub bool);
impl CustomState for Loop {}
