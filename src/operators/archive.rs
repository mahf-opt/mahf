//! Archiving methods

use crate::{
    framework::{components::*, legacy::components::*, Individual, State},
    operators::custom_state::ElitismState,
    problems::Problem,
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

/// Adds elitists of the population to [ElitismState].
#[derive(Serialize, Deserialize)]
pub struct Elitists {
    pub n_elitists: usize,
}
impl Elitists {
    pub fn new<P: Problem>(n_elitists: usize) -> Box<dyn Component<P>> {
        Box::new(Self { n_elitists })
    }
}
impl<P> Component<P> for Elitists
where
    P: Problem,
{
    fn initialize(&self, _problem: &P, state: &mut State) {
        state.get_or_default::<ElitismState>();
    }

    fn execute(&self, _problem: &P, state: &mut State) {
        let mut population = state.population_stack_mut().pop();
        let mut elitism_state = state.get_mut::<ElitismState>();

        for elitist in elitism_state.elitists.drain(..) {
            if !population.contains(&elitist) {
                population.push(elitist);
            }
        }

        let mut pop = population.iter().collect::<Vec<&Individual>>();
        pop.sort_unstable_by_key(|i| i.fitness());
        pop.truncate(self.n_elitists);
        let elitists = pop.into_iter().map(Individual::clone).collect();
        elitism_state.elitists = elitists;

        let fittest = elitism_state.elitists[0].fitness();

        let best_so_far = state.best_fitness();
        assert_eq!(best_so_far, fittest);

        state.population_stack_mut().push(population);
    }
}
