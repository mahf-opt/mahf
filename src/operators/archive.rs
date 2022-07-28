//! Archiving methods

use crate::{
    framework::{components::*, state::State},
    operators::custom_state::ElitistArchiveState,
    problems::SingleObjectiveProblem,
};
use serde::{Deserialize, Serialize};

/// Updates the [ElitistArchiveState] with the current population.
#[derive(Serialize, Deserialize)]
pub struct ElitistArchive {
    pub n_elitists: usize,
}
impl ElitistArchive {
    pub fn new<P: SingleObjectiveProblem>(n_elitists: usize) -> Box<dyn Component<P>> {
        Box::new(Self { n_elitists })
    }
}
impl<P> Component<P> for ElitistArchive
where
    P: SingleObjectiveProblem,
{
    fn initialize(&self, _problem: &P, state: &mut State) {
        state.insert::<ElitistArchiveState<P>>(ElitistArchiveState::new(self.n_elitists));
    }

    fn execute(&self, _problem: &P, state: &mut State) {
        let population = state.population_stack_mut().pop();
        state
            .get_mut::<ElitistArchiveState<P>>()
            .update(&population);
        state.population_stack_mut().push(population);
    }
}

/// Adds elitists from [ElitistArchiveState] to the population.
#[derive(Serialize, Deserialize)]
pub struct AddElitists;
impl AddElitists {
    pub fn new<P: SingleObjectiveProblem>() -> Box<dyn Component<P>> {
        Box::new(Self)
    }
}
impl<P> Component<P> for AddElitists
where
    P: SingleObjectiveProblem,
{
    fn initialize(&self, _problem: &P, state: &mut State) {
        state.require::<ElitistArchiveState<P>>();
    }

    fn execute(&self, _problem: &P, state: &mut State) {
        let mut population = state.population_stack_mut().pop();
        let elitism_state = state.get::<ElitistArchiveState<P>>();

        for elitist in elitism_state.elitists() {
            if !population.contains(elitist) {
                population.push(elitist.clone());
            }
        }

        state.population_stack_mut().push(population);
    }
}
