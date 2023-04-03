use crate::{
    problems::{Problem, SingleObjectiveProblem},
    state::{common, CustomState, State},
    tracking::log::Entry,
};
use serde::Serialize;
use std::{any::type_name, ops::Deref};
use crate::state::diversity::DiversityState;

/// A function to turn some state into an [Entry].
pub type Extractor<'a, P> = fn(&State<'a, P>) -> Entry;

/// A function to log anything that implements [Clone] + [Serialize]
pub fn auto<'a, T, P>(state: &State<'a, P>) -> Entry
where
    T: CustomState<'a> + Clone + Serialize + 'static,
    P: Problem,
{
    debug_assert!(state.has::<T>(), "missing state: {}", type_name::<T>());

    let instance = state.get::<T>();
    let value = Box::new(instance.clone());
    let name = type_name::<T>();
    Entry { name, value }
}

/// A function which logs the best individual.
///
/// Requires the [Problem::Encoding](Problem::Encoding) to implement [Clone] and [Serialize].
pub fn best_individual<P>(state: &State<P>) -> Entry
where
    P: SingleObjectiveProblem,
    P::Encoding: Clone + Serialize + Sized + 'static,
{
    debug_assert!(
        state.has::<common::BestIndividual<P>>(),
        "missing state: {}",
        type_name::<common::BestIndividual<P>>()
    );

    let instance = state.get::<common::BestIndividual<P>>();
    let value = Box::new(if let Some(instance) = instance.deref() {
        let individual = instance.solution().clone();
        Some(individual)
    } else {
        None
    });

    let name = type_name::<common::BestIndividual<P>>();
    Entry { name, value }
}

pub fn best_objective_value<P>(state: &State<P>) -> Entry
where
    P: SingleObjectiveProblem,
{
    Entry {
        name: "BestObjectiveValue",
        value: Box::new(state.best_objective_value().cloned()),
    }
}

/// A function for logging the diversity value of the DiversityState.
pub fn normalized_diversity<I: Send + 'static>(state: &State<P>) -> Entry {
    Entry {
        name: type_name::<DiversityState<I>>(),
        value: Box::new(state.get::<DiversityState<I>>().diversity)
    }
}