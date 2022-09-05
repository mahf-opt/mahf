use crate::{
    framework::state::{common, CustomState, State},
    problems::SingleObjectiveProblem,
    tracking::log::Entry,
};
use serde::Serialize;
use std::{any::type_name, ops::Deref};

/// A function to turn some state into an [Entry].
pub type LogFn = fn(&State) -> Entry;

/// A function to log anything that implements [Clone] + [Serialize]
pub fn auto<T: CustomState + Clone + Serialize>(state: &State) -> Entry {
    debug_assert!(state.has::<T>(), "missing state: {}", type_name::<T>());

    let instance = state.get::<T>();
    let value = Box::new(instance.clone());
    let name = type_name::<T>();
    Entry { name, value }
}

/// A function which logs the best individual.
///
/// Requires the [Problem::Encoding](crate::problems::Problem::Encoding) to implement [Clone] and [Serialize].
pub fn best_individual<P>(state: &State) -> Entry
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

pub fn best_objective_value<P>(state: &State) -> Entry
where
    P: SingleObjectiveProblem,
{
    Entry {
        name: "BestObjectiveValue",
        value: Box::new(state.best_objective_value::<P>().cloned()),
    }
}
