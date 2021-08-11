//! Framework components.

use crate::{
    heuristic::{Individual, State},
    problem::Problem,
    random::Random,
};
use erased_serde::Serialize as DynSerialize;
use std::any::Any;

/// Defines the traits required by any component.
///
/// This will be implemented automatically for all structs satisfying the requirements.
///
/// # Any
/// All components must allow downcasting and thus require [Any].
///
/// # Serialize
/// [DynSerialize] allows serializing dynamic components for the purpose of logging.
///
/// # Send
/// Most of the time, execution should be multi threaded and having
/// components implement [Send] makes this much easier.
///
pub trait Component: Any + DynSerialize + Send {}
impl<T> Component for T where T: Any + DynSerialize + Send {}

/// Initializes the population.
pub trait Initialization<P: Problem>: Component {
    fn initialize(
        &self,
        state: &mut State,
        problem: &P,
        rng: &mut Random,
        population: &mut Vec<P::Encoding>,
    );
}
erased_serde::serialize_trait_object!(<P> Initialization<P>);

/// Selects individuals for reproduction or modification.
pub trait Selection: Component {
    fn select<'p>(
        &self,
        state: &mut State,
        rng: &mut Random,
        population: &'p [Individual],
        selection: &mut Vec<&'p Individual>,
    );
}
erased_serde::serialize_trait_object!(Selection);

/// Generates new solutions from the selected population.
pub trait Generation<P: Problem>: Component {
    fn generate(
        &self,
        state: &mut State,
        problem: &P,
        rng: &mut Random,
        parents: &mut Vec<P::Encoding>,
        offspring: &mut Vec<P::Encoding>,
    );
}
erased_serde::serialize_trait_object!(<P> Generation<P>);

pub trait Scheduler: Component {
    fn schedule(
        &self,
        state: &mut State,
        rng: &mut Random,
        choices: usize,
        population: &[Individual],
        parents: &[&Individual],
        schedule: &mut Vec<usize>,
    );
}
erased_serde::serialize_trait_object!(Scheduler);

/// Replaces old individuals with new ones.
pub trait Replacement: Component {
    fn replace(
        &self,
        state: &mut State,
        rng: &mut Random,
        population: &mut Vec<Individual>,
        offspring: &mut Vec<Individual>,
    );
}
erased_serde::serialize_trait_object!(Replacement);

/// Decides when to terminate.
pub trait Termination: Component {
    fn terminate(&self, state: &mut State) -> bool;
}
erased_serde::serialize_trait_object!(Termination);

/// Can be inserted between steps.
pub trait Postprocess<P: Problem>: Component {
    fn postprocess(
        &self,
        state: &mut State,
        problem: &P,
        rng: &mut Random,
        population: &[Individual],
    );
}
erased_serde::serialize_trait_object!(<P> Postprocess<P>);
