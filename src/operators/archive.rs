//! Archiving methods

use crate::operators::custom_state::ElitismState;
use crate::problems::Problem;
use crate::{
    framework::{components::*, Individual, State},
    random::Random,
};
use serde::{Deserialize, Serialize};

/// Do not use archive
#[derive(Serialize, Deserialize)]
pub struct None;
impl<P> Archiving<P> for None
where
    P: Problem,
{
    fn archive(
        &self,
        _state: &mut State,
        _rng: &mut Random,
        _problem: &P,
        _population: &mut Vec<Individual>,
        _offspring: &mut Vec<Individual>,
    ) {
    }
}

/// Adds elitists after replacement
#[derive(Serialize, Deserialize)]
pub struct Elitists {}

impl<P> Archiving<P> for Elitists
where
    P: Problem,
{
    fn archive(
        &self,
        state: &mut State,
        _rng: &mut Random,
        _problem: &P,
        population: &mut Vec<Individual>,
        _offspring: &mut Vec<Individual>,
    ) {
        if state.custom.has::<ElitismState>() {
            let elitism_state = state.custom.get_mut::<ElitismState>();
            for elitist in elitism_state.elitists.iter() {
                if population.iter().all(|ind| ind != elitist) {
                    population.push(elitist.clone());
                }
            }
        }
    }
}
