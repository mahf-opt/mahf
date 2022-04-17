//! Archiving methods

use crate::operators::custom_state::ElitismState;
use crate::problems::Problem;
use crate::{
    framework::{
        components::*,
        legacy::{components::*, State},
        Individual,
    },
    random::Random,
};
use serde::{Deserialize, Serialize};

/// Do not use archive
#[derive(Serialize, Deserialize)]
pub struct None;
impl None {
    pub fn new<P: Problem>() -> Box<dyn Component<P>> {
        Box::new(Archiver(Self))
    }
}
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
pub struct Elitists {
    pub n_elitists: usize,
}
impl Elitists {
    pub fn new<P: Problem>(n_elitists: usize) -> Box<dyn Component<P>> {
        Box::new(Archiver(Self { n_elitists }))
    }
}
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
        if !state.custom.has::<ElitismState>() {
            state.custom.insert(ElitismState { elitists: vec![] });
        }
        let mut elitism_state = state.custom.get_mut::<ElitismState>();

        for elitist in elitism_state.elitists.drain(..) {
            if population.iter().all(|ind| ind != &elitist) {
                population.push(elitist);
            }
        }

        let mut pop = population.iter().collect::<Vec<&Individual>>();
        pop.sort_unstable_by_key(|i| i.fitness());
        pop.truncate(self.n_elitists);
        let elitists = pop.into_iter().map(Individual::clone).collect();
        elitism_state.elitists = elitists;

        assert_eq!(state.best_so_far, elitism_state.elitists[0].fitness());
    }
}
