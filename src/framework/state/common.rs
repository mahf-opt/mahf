use super::{CustomState, State};
use crate::{
    framework::{Fitness, Individual},
    random::Random,
};
use derive_deref::{Deref, DerefMut};

pub fn common_state(state: &mut State) {
    state.insert(Evaluations(0));
    state.insert(Iterations(0));
    state.insert(Progress(0.0));
    state.insert(BestFitness(Fitness::default()));
}

#[derive(Deref, DerefMut)]
pub struct Rng(pub Random);
impl CustomState for Rng {}

#[derive(Deref, DerefMut)]
pub struct Population(pub Vec<Individual>);
impl CustomState for Population {}

#[derive(Deref, DerefMut)]
pub struct BestIndividual(pub Individual);
impl CustomState for BestIndividual {}

#[derive(Deref, DerefMut)]
pub struct Selection(pub Vec<Individual>);
impl CustomState for Selection {}

#[derive(Deref, DerefMut)]
pub struct RawGeneration<E>(pub Vec<E>);
impl<E: 'static> CustomState for RawGeneration<E> {}

#[derive(Deref, DerefMut)]
pub struct Offspring(pub Individual);
impl CustomState for Offspring {}

#[derive(Deref, DerefMut)]
pub struct Evaluations(pub u32);
impl CustomState for Evaluations {}

#[derive(Deref, DerefMut)]
pub struct Iterations(pub u32);
impl CustomState for Iterations {}

#[derive(Deref, DerefMut)]
pub struct Progress(pub f64);
impl CustomState for Progress {}

#[derive(Deref, DerefMut)]
pub struct BestFitness(pub Fitness);
impl CustomState for BestFitness {}

#[derive(Deref, DerefMut)]
pub struct Loop(pub bool);
impl CustomState for Loop {}
