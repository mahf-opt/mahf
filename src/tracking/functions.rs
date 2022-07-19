use crate::{
    framework::{common_state, CustomState, State},
    problems::Problem,
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
    let name = std::any::type_name::<T>();
    Entry { name, value }
}

/// A function which logs the best individual.
///
/// Requires the [Problem::Encoding] to implement [Clone] and [Serialize].
pub fn best_individual<P>(state: &State) -> Entry
where
    P: Problem,
    P::Encoding: Clone + Serialize + Sized + 'static,
{
    debug_assert!(
        state.has::<common_state::BestIndividual>(),
        "missing state: {}",
        type_name::<common_state::BestIndividual>()
    );

    let instance = state.get::<common_state::BestIndividual>();
    let value = Box::new(if let Some(instance) = instance.deref() {
        let individual = instance.solution::<P::Encoding>().clone();
        Some(individual)
    } else {
        None
    });

    let name = std::any::type_name::<common_state::BestIndividual>();
    Entry { name, value }
}
