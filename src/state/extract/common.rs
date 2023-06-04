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
        extract::{Extract, ExtractMap, ExtractMutRef, ExtractRef},
        StateRegistry,
    },
    CustomState, Problem, SingleObjective,
};

#[derive(Derivative)]
#[derivative(Default(bound = ""), Clone(bound = ""))]
pub struct IdFn<T>(PhantomData<fn() -> T>);

impl<T> EntryName for IdFn<T> {
    fn entry_name() -> &'static str {
        type_name::<T>()
    }
}

impl<E> IdFn<E>
where
    Self: Extract<Target = E>,
    E: Serialize + Send,
{
    pub fn entry<P: Problem>() -> Box<dyn EntryExtractor<P>> {
        Box::<Self>::default()
    }
}

impl<E> Extract for IdFn<E>
where
    E: for<'a> CustomState<'a> + Clone,
{
    type Target = E;

    fn extract(state: &StateRegistry) -> ExecResult<Self::Target> {
        Ok(state.try_borrow::<E>()?.clone())
    }
}

impl<E> ExtractRef for IdFn<E>
where
    E: for<'a> CustomState<'a>,
{
    type Target = E;

    fn extract_ref<'a>(state: &'a StateRegistry) -> ExecResult<Ref<'a, Self::Target>> {
        Ok(state.try_borrow::<E>()?)
    }
}

impl<E> ExtractMutRef for IdFn<E>
where
    E: for<'a> CustomState<'a>,
{
    fn extract_mut<'a>(state: &'a StateRegistry) -> ExecResult<RefMut<'a, Self::Target>> {
        Ok(state.try_borrow_mut::<E>()?)
    }
}

#[derive(Derivative)]
#[derivative(Default(bound = ""), Clone(bound = ""))]
pub struct ValueOf<T>(PhantomData<fn() -> T>);

impl<T> EntryName for ValueOf<T> {
    fn entry_name() -> &'static str {
        type_name::<T>()
    }
}

impl<E> ValueOf<E>
where
    Self: Extract<Target = E>,
    E: Serialize + Send,
{
    pub fn entry<P: Problem>() -> Box<dyn EntryExtractor<P>> {
        Box::<Self>::default()
    }
}

impl<E> Extract for ValueOf<E>
where
    E: Deref + for<'a> CustomState<'a>,
    <E as Deref>::Target: Sized + Clone,
{
    type Target = E::Target;

    fn extract(state: &StateRegistry) -> ExecResult<Self::Target> {
        Ok(state.try_get_value::<E>()?)
    }
}

impl<E> ExtractRef for ValueOf<E>
where
    E: Deref + for<'a> CustomState<'a>,
    <E as Deref>::Target: Sized,
{
    type Target = E::Target;

    fn extract_ref<'a>(state: &'a StateRegistry) -> ExecResult<Ref<'a, Self::Target>> {
        Ok(state.try_borrow_value::<E>()?)
    }
}

impl<E> ExtractMutRef for ValueOf<E>
where
    E: DerefMut + for<'a> CustomState<'a>,
    <E as Deref>::Target: Sized,
{
    fn extract_mut<'a>(state: &'a StateRegistry) -> ExecResult<RefMut<'a, Self::Target>> {
        Ok(state.try_borrow_value_mut::<E>()?)
    }
}

#[derive(Serialize, Derivative)]
#[derivative(Default(bound = ""), Clone(bound = ""))]
pub struct ExPopulationSize<P>(PhantomData<P>);

impl<P> EntryName for ExPopulationSize<P> {
    fn entry_name() -> &'static str {
        "PopulationSize"
    }
}

impl<P: Problem> ExtractMap for ExPopulationSize<P> {
    type Source = Populations<P>;
    type Target = u32;

    fn map(source: &Self::Source) -> Self::Target {
        source.current().len() as u32
    }
}

#[derive(Serialize, Derivative)]
#[derivative(Default(bound = ""), Clone(bound = ""))]
pub struct ExBestSolution<P>(PhantomData<fn() -> P>);

impl<P> EntryName for ExBestSolution<P> {
    fn entry_name() -> &'static str {
        "BestSolution"
    }
}

impl<P> ExBestSolution<P>
where
    P: SingleObjectiveProblem,
    P::Encoding: Clone,
    Self: Extract,
    <Self as Extract>::Target: Serialize + Send,
{
    pub fn entry() -> Box<dyn EntryExtractor<P>> {
        Box::<Self>::default()
    }
}

impl<P> Extract for ExBestSolution<P>
where
    P: SingleObjectiveProblem,
    P::Encoding: Clone,
{
    type Target = P::Encoding;

    fn extract(state: &StateRegistry) -> ExecResult<Self::Target> {
        state
            .try_borrow::<BestIndividual<P>>()?
            .as_ref()
            .map(|i| i.solution().clone())
            .ok_or_else(|| eyre!("no best individual found yet"))
    }
}

#[derive(Serialize, Derivative)]
#[derivative(Default(bound = ""), Clone(bound = ""))]
pub struct ExBestObjectiveValue<P>(PhantomData<fn() -> P>);

impl<P> EntryName for ExBestObjectiveValue<P> {
    fn entry_name() -> &'static str {
        "BestObjectiveValue"
    }
}

impl<P> ExBestObjectiveValue<P>
where
    P: SingleObjectiveProblem,
    Self: Extract,
    <Self as Extract>::Target: Serialize + Send,
{
    pub fn entry() -> Box<dyn EntryExtractor<P>> {
        Box::<Self>::default()
    }
}

impl<P: SingleObjectiveProblem> Extract for ExBestObjectiveValue<P> {
    type Target = SingleObjective;

    fn extract(state: &StateRegistry) -> ExecResult<Self::Target> {
        state
            .try_borrow::<BestIndividual<P>>()?
            .as_ref()
            .map(|i| *i.objective())
            .ok_or_else(|| eyre!("no best individual found yet"))
    }
}

impl<P: SingleObjectiveProblem> ExtractRef for ExBestObjectiveValue<P> {
    type Target = SingleObjective;

    fn extract_ref<'a>(state: &'a StateRegistry) -> ExecResult<Ref<'a, Self::Target>> {
        Ref::filter_map(state.try_borrow::<BestIndividual<P>>()?, |best| {
            best.as_ref().map(|i| i.objective())
        })
        .map_err(|_| eyre!("no best individual found yet"))
    }
}
