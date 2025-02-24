//! Archive for specified parts of population.

use std::cell::Ref;

use better_any::{Tid, TidAble};
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::{
    component::ExecResult, components::Component, problems::SingleObjectiveProblem,
    state::StateReq, CustomState, Individual, Problem, State,
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

impl ElitistArchiveUpdate {
    pub fn from_params(num_elitists: usize) -> Self {
        Self { num_elitists }
    }

    pub fn new<P>(num_elitists: usize) -> Box<dyn Component<P>>
    where
        P: SingleObjectiveProblem,
    {
        Box::new(Self::from_params(num_elitists))
    }
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

/// Inserts the elitists from the [`ElitistArchive`] into the population.
///
/// It won't add duplicates to the archive.
#[derive(Clone, Serialize, Deserialize)]
pub struct ElitistArchiveIntoPopulation;

impl ElitistArchiveIntoPopulation {
    pub fn from_params() -> Self {
        Self
    }

    pub fn new<P>() -> Box<dyn Component<P>>
    where
        P: SingleObjectiveProblem,
    {
        Box::new(Self::from_params())
    }
}

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

/// An archive for storing individuals between operators, e.g. for subsequent calculation of measures.
#[derive(Default, Tid)]
pub struct IntermediateArchive<P: Problem + 'static>(Vec<Individual<P>>);

impl<P: Problem> CustomState<'_> for IntermediateArchive<P> {}

impl<P: Problem> IntermediateArchive<P> {
    /// Creates a new, empty `IntermediateArchive`.
    fn new() -> Self {
        Self(Vec::new())
    }

    /// Updates the archive using the `population`, keeping all individuals at the current step of the algorithm.
    fn update(&mut self, population: &[Individual<P>]) {
        self.0 = Vec::from(population);
    }

    /// Returns a reference to the archived population.
    pub fn archived_population(&self) -> &[Individual<P>] {
        &self.0
    }

    /// Returns a mutable reference to the archived population.
    pub fn archived_population_mut(&mut self) -> &mut [Individual<P>] {
        &mut self.0
    }
}

/// Updates the [`IntermediateArchive`] with the current population.
#[derive(Clone, Serialize, Deserialize)]
pub struct IntermediateArchiveUpdate;

impl IntermediateArchiveUpdate {
    pub fn from_params() -> Self {
        Self {}
    }

    pub fn new<P>() -> Box<dyn Component<P>>
    where
        P: Problem,
    {
        Box::new(Self::from_params())
    }
}

impl<P> Component<P> for IntermediateArchiveUpdate
where
    P: Problem,
{
    fn init(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
        state.insert(IntermediateArchive::<P>::new());
        Ok(())
    }

    fn execute(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
        state
            .borrow_mut::<IntermediateArchive<P>>()
            .update(state.populations().current());
        Ok(())
    }
}

/// An archive for storing all best individual yet, e.g. for subsequent calculation of measures.
#[derive(Default, Tid)]
pub struct BestIndividualsArchive<P: Problem + 'static>(Vec<Individual<P>>);

impl<P: Problem> CustomState<'_> for BestIndividualsArchive<P> {}

impl<P: Problem> BestIndividualsArchive<P> {
    /// Creates a new, empty `BestIndividualsArchive`.
    fn new() -> Self {
        Self(Vec::new())
    }

    /// Updates the archive using the `BestIndividual`, adding it to a vector of previously found best individuals.
    fn update(&mut self, best_individual: Option<Ref<Individual<P>>>) {
        self.0.push(best_individual.unwrap().clone());
    }

    /// Returns a reference to the archived individuals.
    pub fn archived_best_individuals(&self) -> &[Individual<P>] {
        &self.0
    }

    /// Returns a mutable reference to the archived individuals.
    pub fn archived_best_individuals_mut(&mut self) -> &mut [Individual<P>] {
        &mut self.0
    }
}

/// Updates the [`BestIndividualsArchive`] with the current best individual.
#[derive(Clone, Serialize, Deserialize)]
pub struct BestIndividualsArchiveUpdate;

impl BestIndividualsArchiveUpdate {
    pub fn from_params() -> Self {
        Self {}
    }

    pub fn new<P>() -> Box<dyn Component<P>>
    where
        P: Problem + SingleObjectiveProblem,
    {
        Box::new(Self::from_params())
    }
}

impl<P> Component<P> for BestIndividualsArchiveUpdate
where
    P: Problem + SingleObjectiveProblem,
{
    fn init(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
        state.insert(BestIndividualsArchive::<P>::new());
        Ok(())
    }

    fn execute(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
        state
            .borrow_mut::<BestIndividualsArchive<P>>()
            .update(state.best_individual());
        Ok(())
    }
}

/// An archive for storing parent individuals, e.g. for JADE and SHADE.
#[derive(Default, Tid)]
pub struct DEKeepParentsArchive<P: SingleObjectiveProblem + 'static>(Vec<Individual<P>>);

impl<P: SingleObjectiveProblem> CustomState<'_> for DEKeepParentsArchive<P> {}

impl<P: SingleObjectiveProblem> DEKeepParentsArchive<P> {
    /// Creates a new, empty `DEKeepParentsArchive`.
    fn new() -> Self {
        Self(Vec::new())
    }

    /// Updates the archive using the `population`, keeping the parent idividuals if they are worse
    /// than their offspring.
    /// If the `max_archive` is reached, the oldest parents are removed.
    fn update(&mut self, offspring: Vec<Individual<P>>, parents: Vec<Individual<P>>, max_archive: usize) {

        let mut archive_additions: Vec<Individual<P>> = Vec::new();
        for (i, offspring) in offspring.into_iter().enumerate() {
            if parents[i].objective() < offspring.objective() {
                archive_additions.push(parents[i].clone());
            }
        }
        
        self.0.extend_from_slice(&*archive_additions);
        let length = self.0.len();
        if length > max_archive {
            self.0.drain(0..(length - max_archive));
        }
        
    }

    /// Returns a reference to the elitists.
    pub fn parents(&self) -> &[Individual<P>] {
        &self.0
    }

    /// Returns a mutable reference to the elitists.
    pub fn parents_mut(&mut self) -> &mut [Individual<P>] {
        &mut self.0
    }
}

/// Updates the [`DEKeepParentsArchive`] with the current population, keeping `max_archive` parents.
#[derive(Clone, Serialize, Deserialize)]
pub struct DEKeepParentsArchiveUpdate {
    /// The number of elitists to keep in the archive.
    pub max_archive: usize,
}

impl DEKeepParentsArchiveUpdate {
    pub fn from_params(max_archive: usize) -> Self {
        Self { max_archive }
    }

    pub fn new<P>(max_archive: usize) -> Box<dyn Component<P>>
    where
        P: SingleObjectiveProblem,
    {
        Box::new(Self::from_params(max_archive))
    }
}

impl<P> Component<P> for DEKeepParentsArchiveUpdate
where
    P: SingleObjectiveProblem,
{
    fn init(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
        state.insert(DEKeepParentsArchive::<P>::new());
        Ok(())
    }

    fn execute(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
        let mut populations = state.populations_mut();
        let offspring = populations.pop();
        let parents = populations.pop();
        let o = offspring.clone();
        let p = parents.clone();
        populations.push(parents);
        populations.push(offspring);
        state
            .borrow_mut::<DEKeepParentsArchive<P>>()
            .update(o, p, self.max_archive);
        Ok(())
    }
}

/// Inserts the elitists from the [`ElitistArchive`] into the population.
///
/// It won't add duplicates to the archive.
#[derive(Clone, Serialize, Deserialize)]
pub struct DEKeepParentsArchiveIntoPopulation;

impl DEKeepParentsArchiveIntoPopulation {
    pub fn from_params() -> Self {
        Self
    }

    pub fn new<P>() -> Box<dyn Component<P>>
    where
        P: SingleObjectiveProblem,
    {
        Box::new(Self::from_params())
    }
}

impl<P> Component<P> for DEKeepParentsArchiveIntoPopulation
where
    P: SingleObjectiveProblem,
{
    fn require(&self, _problem: &P, state_req: &StateReq<P>) -> ExecResult<()> {
        state_req.require::<Self, DEKeepParentsArchive<P>>()?;
        Ok(())
    }

    fn execute(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
        let archive = state.borrow::<DEKeepParentsArchive<P>>();
        let mut populations = state.populations_mut();
        let population = populations.current_mut();

        for parent in archive.parents() {
            if !population.contains(parent) {
                population.push(parent.clone());
            }
        }

        Ok(())
    }
}
