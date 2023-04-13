//! Archiving methods

use better_any::Tid;
use serde::{Deserialize, Serialize};

use crate::{
    components::Component,
    framework::Individual,
    problems::SingleObjectiveProblem,
    state::{CustomState, State},
};

/// State required for Elitism.
///
/// For preserving n elitist individuals.
#[derive(Tid)]
pub struct ElitistArchive<P: SingleObjectiveProblem + 'static> {
    elitists: Vec<Individual<P>>,
}

impl<P: SingleObjectiveProblem> CustomState<'_> for ElitistArchive<P> {}

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
            fn initialize(&self, _problem: &P, state: &mut State<P>) {
                state.insert(ElitistArchive::<P>::new());
            }

            fn execute(&self, _problem: &P, state: &mut State<P>) {
                let population = state.populations_mut().pop();
                state
                    .get_mut::<ElitistArchive<P>>()
                    .state_update(&population, self.n_elitists);
                state.populations_mut().push(population);
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
            fn initialize(&self, _problem: &P, state: &mut State<P>) {
                state.require::<Self, ElitistArchive<P>>();
            }

            fn execute(&self, _problem: &P, state: &mut State<P>) {
                let mut population = state.populations_mut().pop();
                let elitism_state = state.get::<ElitistArchive<P>>();

                for elitist in elitism_state.elitists() {
                    if !population.contains(elitist) {
                        population.push(elitist.clone());
                    }
                }

                state.populations_mut().push(population);
            }
        }

        Box::new(AddElitists)
    }
}
