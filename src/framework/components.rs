//! Framework components.

use crate::{
    framework::{Individual, State},
    problems::Problem,
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
pub trait Component: Any + DynSerialize + Send + Sync {}
impl<T> Component for T where T: Any + DynSerialize + Send + Sync {}

/// Initializes the population.
///
/// See [crate::operators::initialization] for existing implementations.
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
///
/// See [crate::operators::selection] for existing implementations.
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
///
/// See [crate::operators::generation] for existing implementations.
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

/// Schedules the [Generation] operators.
///
/// This function defines which operators should be called how often and in what order.
///
/// See [crate::operators::schedulers] for existing implementations.
pub trait Scheduler: Component {
    /// Schedule the operators.
    ///
    /// `choices` is the number of operators to choose from.
    /// The selected operators must be added to `schedule` in the order they should be called in.
    /// Operators can be added multiple times to the `schedule`.
    ///
    /// Operators are represented by their index. Valid operators are thus in the range \[0..choices].
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
///
/// See [crate::operators::replacement] for existing implementations.
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

/// Exchanges individuals between population and archive after replacement.
///
/// See [crate::operators::archive] for existing implementations.
pub trait Archiving<P: Problem>: Component {
    fn archive(
        &self,
        state: &mut State,
        rng: &mut Random,
        _problem: &P,
        population: &mut Vec<Individual>,
        offspring: &mut Vec<Individual>,
    );
}
erased_serde::serialize_trait_object!(<P> Archiving<P>);

/// Decides when to terminate.
///
/// See [crate::operators::termination] for existing implementations.
pub trait Termination: Component {
    fn terminate(&self, state: &mut State) -> bool;
}
erased_serde::serialize_trait_object!(Termination);

/// Can be inserted between steps.
///
/// See [crate::operators::postprocess] for existing implementations.
pub trait Postprocess<P: Problem>: Component {
    /// Called exactly once.
    fn initialize(
        &self,
        state: &mut State,
        problem: &P,
        rng: &mut Random,
        population: &[Individual],
    );

    /// Called after initialization and every replacement.
    fn postprocess(
        &self,
        state: &mut State,
        problem: &P,
        rng: &mut Random,
        population: &[Individual],
    );
}
erased_serde::serialize_trait_object!(<P> Postprocess<P>);
