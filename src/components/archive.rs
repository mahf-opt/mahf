//! Elitist archive.

use better_any::{Tid, TidAble};
use serde::{Deserialize, Serialize};

use crate::{
    component::ExecResult, components::Component, problems::SingleObjectiveProblem,
    state::StateReq, CustomState, Individual, State,
};

/// An archive for storing elitist individuals.
#[derive(Default, Tid)]
pub struct ElitistArchive<P: SingleObjectiveProblem + 'static>(Vec<Individual<P>>);

impl<P: SingleObjectiveProblem> CustomState<'_> for ElitistArchive<P> {}

impl<P: SingleObjectiveProblem> ElitistArchive<P> {
    /// Creates a new, empty `ElitistArchive`.
    fn new() -> Self {
        Self(Vec::new())
    }

    /// Updates the archive using the `population`, keeping the best `num_elitist` elitists
    /// from both.
    fn update(&mut self, population: &[Individual<P>], num_elitists: usize) {
        self.0.extend_from_slice(population);
        self.0.sort_unstable_by_key(|i| *i.objective());
        self.0.truncate(num_elitists);
    }

    /// Returns a reference to the elitists.
    pub fn elitists(&self) -> &[Individual<P>] {
        &self.0
    }

    /// Returns a mutable reference to the elitists.
    pub fn elitists_mut(&mut self) -> &mut [Individual<P>] {
        &mut self.0
    }
}

/// Updates the [`ElitistArchive`] with the current population, keeping `num_elitist` elitists.
#[derive(Clone, Serialize, Deserialize)]
pub struct ElitistArchiveUpdate {
    /// The number of elitists to keep in the archive.
    pub num_elitists: usize,
}

impl<P> Component<P> for ElitistArchiveUpdate
where
    P: SingleObjectiveProblem,
{
    fn init(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
        state.insert(ElitistArchive::<P>::new());
        Ok(())
    }

    fn execute(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
        state
            .borrow_mut::<ElitistArchive<P>>()
            .update(state.populations().current(), self.num_elitists);
        Ok(())
    }
}

/// Inserts the elitists from the [`ElitistArchive`] into the population, given the population
/// does not already contain them.
#[derive(Clone, Serialize, Deserialize)]
pub struct ElitistArchiveIntoPopulation;

impl<P> Component<P> for ElitistArchiveIntoPopulation
where
    P: SingleObjectiveProblem,
{
    fn require(&self, _problem: &P, state_req: &StateReq<P>) -> ExecResult<()> {
        state_req.require::<Self, ElitistArchive<P>>()?;
        Ok(())
    }

    fn execute(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
        let archive = state.borrow::<ElitistArchive<P>>();
        let mut populations = state.populations_mut();
        let population = populations.current_mut();

        for elitist in archive.elitists() {
            if !population.contains(elitist) {
                population.push(elitist.clone());
            }
        }

        Ok(())
    }
}
