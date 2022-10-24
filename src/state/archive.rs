//! Archiving methods

use crate::{
    framework::{components::*, Individual},
    problems::SingleObjectiveProblem,
    state::{CustomState, State},
};
use serde::{Deserialize, Serialize};

/// State required for Elitism.
///
/// For preserving n elitist individuals.
pub struct ElitistArchive<P: SingleObjectiveProblem> {
    elitists: Vec<Individual<P>>,
}

impl<P: SingleObjectiveProblem> CustomState for ElitistArchive<P> {}

impl<P: SingleObjectiveProblem> ElitistArchive<P> {
    fn new() -> Self {
        Self {
            elitists: Vec::new(),
        }
    }

    fn state_update(&mut self, population: &[Individual<P>], n_elitists: usize) {
        let mut pop = population.iter().collect::<Vec<_>>();
        pop.sort_unstable_by_key(|i| i.objective());
        pop.truncate(n_elitists);
        self.elitists = pop.into_iter().cloned().collect();
    }
}

impl<P: SingleObjectiveProblem> ElitistArchive<P> {
    pub fn elitists(&self) -> &[Individual<P>] {
        &self.elitists
    }

    pub fn elitists_mut(&mut self) -> &mut [Individual<P>] {
        &mut self.elitists
    }
}

impl<P: SingleObjectiveProblem> ElitistArchive<P> {
    /// Updates the [ElitistArchiveState] with the current population.
    pub fn update(n_elitists: usize) -> Box<dyn Component<P>> {
        #[derive(Serialize, Deserialize, Clone)]
        pub struct ElitistArchiveUpdate {
            pub n_elitists: usize,
        }

        impl<P> Component<P> for ElitistArchiveUpdate
        where
            P: SingleObjectiveProblem,
        {
            fn initialize(&self, _problem: &P, state: &mut State) {
                state.insert::<ElitistArchive<P>>(ElitistArchive::new());
            }

            fn execute(&self, _problem: &P, state: &mut State) {
                let population = state.population_stack_mut().pop();
                state
                    .get_mut::<ElitistArchive<P>>()
                    .state_update(&population, self.n_elitists);
                state.population_stack_mut().push(population);
            }
        }

        Box::new(ElitistArchiveUpdate { n_elitists })
    }

    /// Adds elitists from [ElitistArchiveState] to the population.
    pub fn add_elitists() -> Box<dyn Component<P>> {
        #[derive(Serialize, Deserialize, Clone)]
        pub struct AddElitists;

        impl<P> Component<P> for AddElitists
        where
            P: SingleObjectiveProblem,
        {
            fn initialize(&self, _problem: &P, state: &mut State) {
                state.require::<ElitistArchive<P>>();
            }

            fn execute(&self, _problem: &P, state: &mut State) {
                let mut population = state.population_stack_mut().pop();
                let elitism_state = state.get::<ElitistArchive<P>>();

                for elitist in elitism_state.elitists() {
                    if !population.contains(elitist) {
                        population.push(elitist.clone());
                    }
                }

                state.population_stack_mut().push(population);
            }
        }

        Box::new(AddElitists)
    }
}
