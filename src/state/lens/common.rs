use std::{
    any::type_name,
    cell::{Ref, RefMut},
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use derivative::Derivative;
use eyre::eyre;
use serde::Serialize;

use crate::{
    component::ExecResult,
    logging::extractor::{EntryExtractor, EntryName},
    problems::SingleObjectiveProblem,
    state::{
        common::{BestIndividual, Populations},
        lens::{AnyLens, Lens, LensMap, LensMut, LensRef},
    },
    CustomState, Problem, SingleObjective, State,
};

#[derive(Serialize, Derivative)]
#[serde(bound = "")]
#[derivative(Default(bound = ""), Clone(bound = ""))]
pub struct IdLens<T>(PhantomData<fn() -> T>);

impl<T> IdLens<T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<T> EntryName for IdLens<T> {
    fn entry_name() -> &'static str {
        type_name::<T>()
    }
}

impl<T> IdLens<T>
where
    T: Serialize + Send,
{
    pub fn entry<P>() -> Box<dyn EntryExtractor<P>>
    where
        P: Problem,
        Self: Lens<P, Target = T>,
    {
        Box::<Self>::default()
    }
}

impl<T: 'static> AnyLens for IdLens<T> {
    type Target = T;
}

impl<P, T> Lens<P> for IdLens<T>
where
    P: Problem,
    Self: LensRef<P>,
    Self::Target: Clone,
{
    fn get(&self, state: &State<P>) -> ExecResult<Self::Target> {
        self.get_ref(state).map(|target| target.clone())
    }
}

impl<P, T> LensRef<P> for IdLens<T>
where
    P: Problem,
    T: for<'a> CustomState<'a>,
{
    fn get_ref<'a>(&self, state: &'a State<P>) -> ExecResult<Ref<'a, Self::Target>> {
        Ok(state.try_borrow::<T>()?)
    }
}

impl<P, T> LensMut<P> for IdLens<T>
where
    P: Problem,
    T: for<'a> CustomState<'a>,
{
    fn get_mut<'a>(&self, state: &'a State<P>) -> ExecResult<RefMut<'a, Self::Target>> {
        Ok(state.try_borrow_mut::<T>()?)
    }
}

#[derive(Serialize, Derivative)]
#[serde(bound = "")]
#[derivative(Default(bound = ""), Clone(bound = ""))]
pub struct ValueOf<T>(PhantomData<fn() -> T>);

impl<T> ValueOf<T>
where
    T: Deref,
{
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<T> EntryName for ValueOf<T> {
    fn entry_name() -> &'static str {
        type_name::<T>()
    }
}

impl<T> ValueOf<T>
where
    T: Deref,
    <T as Deref>::Target: Serialize + Send + Sized,
{
    pub fn entry<P>() -> Box<dyn EntryExtractor<P>>
    where
        P: Problem,
        Self: Lens<P, Target = T::Target>,
    {
        Box::<Self>::default()
    }
}

impl<T> AnyLens for ValueOf<T>
where
    T: Deref + for<'a> CustomState<'a>,
    <T as Deref>::Target: Sized,
{
    type Target = T::Target;
}

impl<P, T> Lens<P> for ValueOf<T>
where
    P: Problem,
    Self: LensRef<P>,
    Self::Target: Clone,
{
    fn get(&self, state: &State<P>) -> ExecResult<Self::Target> {
        self.get_ref(state).map(|target| target.clone())
    }
}

impl<P, T> LensRef<P> for ValueOf<T>
where
    P: Problem,
    T: Deref + for<'a> CustomState<'a>,
    <T as Deref>::Target: Sized,
{
    fn get_ref<'a>(&self, state: &'a State<P>) -> ExecResult<Ref<'a, Self::Target>> {
        Ok(state.try_borrow_value::<T>()?)
    }
}

impl<P, T> LensMut<P> for ValueOf<T>
where
    P: Problem,
    T: DerefMut + for<'a> CustomState<'a>,
    <T as Deref>::Target: Sized,
{
    fn get_mut<'a>(&self, state: &'a State<P>) -> ExecResult<RefMut<'a, Self::Target>> {
        Ok(state.try_borrow_value_mut::<T>()?)
    }
}

#[derive(Serialize, Derivative)]
#[serde(bound = "")]
#[derivative(Default(bound = ""), Clone(bound = ""))]
pub struct PopulationSizeLens<P>(PhantomData<fn() -> P>);

impl<P> PopulationSizeLens<P>
where
    P: Problem,
    Self: Lens<P, Target = u32>,
{
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<P: Problem> AnyLens for PopulationSizeLens<P> {
    type Target = u32;
}

impl<P> EntryName for PopulationSizeLens<P> {
    fn entry_name() -> &'static str {
        "PopulationSize"
    }
}

impl<P: Problem> LensMap for PopulationSizeLens<P> {
    type Source = Populations<P>;

    fn map(&self, source: &Self::Source) -> Self::Target {
        source.current().len() as u32
    }
}

#[derive(Serialize, Derivative)]
#[serde(bound = "")]
#[derivative(Default(bound = ""), Clone(bound = ""))]
pub struct BestSolutionLens<P>(PhantomData<fn() -> P>);

impl<P> BestSolutionLens<P>
where
    P: Problem,
    Self: Lens<P, Target = P::Encoding>,
{
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<P: Problem> AnyLens for BestSolutionLens<P> {
    type Target = P::Encoding;
}

impl<P> EntryName for BestSolutionLens<P> {
    fn entry_name() -> &'static str {
        "BestSolution"
    }
}

impl<P> BestSolutionLens<P>
where
    P: SingleObjectiveProblem,
    P::Encoding: Clone,
    Self: Lens<P>,
    <Self as AnyLens>::Target: Serialize + Send,
{
    pub fn entry() -> Box<dyn EntryExtractor<P>> {
        Box::<Self>::default()
    }
}

impl<P> Lens<P> for BestSolutionLens<P>
where
    P: Problem,
    Self: LensRef<P>,
    Self::Target: Clone,
{
    fn get(&self, state: &State<P>) -> ExecResult<Self::Target> {
        self.get_ref(state).map(|target| target.clone())
    }
}

impl<P: SingleObjectiveProblem> LensRef<P> for BestSolutionLens<P> {
    fn get_ref<'a>(&self, state: &'a State<P>) -> ExecResult<Ref<'a, Self::Target>> {
        state
            .best_individual()
            .map(|i| Ref::map(i, |i| i.solution()))
            .ok_or_else(|| eyre!("no best individual found yet"))
    }
}

#[derive(Serialize, Derivative)]
#[serde(bound = "")]
#[derivative(Default(bound = ""), Clone(bound = ""))]
pub struct BestObjectiveValueLens<P>(PhantomData<fn() -> P>);

impl<P> BestObjectiveValueLens<P>
where
    P: Problem,
    Self: Lens<P, Target = SingleObjective>,
{
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<P: Problem> AnyLens for BestObjectiveValueLens<P> {
    type Target = SingleObjective;
}

impl<P> EntryName for BestObjectiveValueLens<P> {
    fn entry_name() -> &'static str {
        "BestObjectiveValue"
    }
}

impl<P> BestObjectiveValueLens<P>
where
    P: SingleObjectiveProblem,
    Self: Lens<P>,
    <Self as AnyLens>::Target: Serialize + Send,
{
    pub fn entry() -> Box<dyn EntryExtractor<P>> {
        Box::<Self>::default()
    }
}

impl<P: SingleObjectiveProblem> Lens<P> for BestObjectiveValueLens<P> {
    fn get(&self, state: &State<P>) -> ExecResult<Self::Target> {
        state
            .best_objective_value()
            .ok_or_else(|| eyre!("no best individual found yet"))
    }
}

impl<P: SingleObjectiveProblem> LensRef<P> for BestObjectiveValueLens<P> {
    fn get_ref<'a>(&self, state: &'a State<P>) -> ExecResult<Ref<'a, Self::Target>> {
        Ref::filter_map(state.try_borrow::<BestIndividual<P>>()?, |best| {
            best.as_ref().map(|i| i.objective())
        })
        .map_err(|_| eyre!("no best individual found yet"))
    }
}
