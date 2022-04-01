//! Framework components.

use crate::{
    framework::{Individual, State},
    problems::Problem,
    random::Random,
};
use std::any::Any;
use trait_set::trait_set;

trait_set! {
    pub trait AnyComponent = Any + Send + Sync;
}

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
pub trait Component<P>: AnyComponent {
    fn initialize(&self, problem: P, state: &mut State) {}
    fn execute(&self, problem: P, state: &mut State);
}
erased_serde::serialize_trait_object!(<P: Problem> Component<P>);

/// Initializes the population.
///
/// See [crate::operators::initialization] for existing implementations.
pub trait Initialization<P: Problem> {
    fn initialize(
        &self,
        state: &mut State,
        problem: &P,
        rng: &mut Random,
        population: &mut Vec<P::Encoding>,
    );
}

#[derive(serde::Serialize)]
pub struct Initializer<T>(pub T);

impl<T, P> Component<P> for Initializer<T>
where
    P: Problem,
    T: AnyComponent + Initialization<P>,
{
    fn execute(&self, problem: P, state: &mut State) {
        todo!()
    }
}

/// Selects individuals for reproduction or modification.
///
/// See [crate::operators::selection] for existing implementations.
pub trait Selection {
    fn select<'p>(
        &self,
        state: &mut State,
        rng: &mut Random,
        population: &'p [Individual],
        selection: &mut Vec<&'p Individual>,
    );
}

#[derive(serde::Serialize)]
pub struct Selector<T>(pub T);

impl<T, P> Component<P> for Selector<T>
where
    P: Problem,
    T: AnyComponent + Selection,
{
    fn execute(&self, problem: P, state: &mut State) {
        todo!()
    }
}

/// Generates new solutions from the selected population.
///
/// See [crate::operators::generation] for existing implementations.
pub trait Generation<P: Problem> {
    fn generate(
        &self,
        state: &mut State,
        problem: &P,
        rng: &mut Random,
        parents: &mut Vec<P::Encoding>,
        offspring: &mut Vec<P::Encoding>,
    );
}

#[derive(serde::Serialize)]
pub struct Generator<T>(pub T);

impl<T, P> Component<P> for Generator<T>
where
    P: Problem,
    T: AnyComponent + Generation<P>,
{
    fn execute(&self, problem: P, state: &mut State) {
        todo!()
    }
}

/// Schedules the [Generation] operators.
///
/// This function defines which operators should be called how often and in what order.
///
/// See [crate::operators::schedulers] for existing implementations.
pub trait Scheduler {
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

#[derive(serde::Serialize)]
pub struct Schedule<T>(pub T);

impl<T, P> Component<P> for Schedule<T>
where
    P: Problem,
    T: AnyComponent + Scheduler,
{
    fn execute(&self, problem: P, state: &mut State) {
        todo!()
    }
}

/// Replaces old individuals with new ones.
///
/// See [crate::operators::replacement] for existing implementations.
pub trait Replacement {
    fn replace(
        &self,
        state: &mut State,
        rng: &mut Random,
        population: &mut Vec<Individual>,
        offspring: &mut Vec<Individual>,
    );
}

#[derive(serde::Serialize)]
pub struct Replacer<T>(pub T);

impl<T, P> Component<P> for Replacer<T>
where
    P: Problem,
    T: AnyComponent + Replacement,
{
    fn execute(&self, problem: P, state: &mut State) {
        todo!()
    }
}

/// Exchanges individuals between population and archive after replacement.
///
/// See [crate::operators::archive] for existing implementations.
pub trait Archiving<P: Problem> {
    fn archive(
        &self,
        state: &mut State,
        rng: &mut Random,
        _problem: &P,
        population: &mut Vec<Individual>,
        offspring: &mut Vec<Individual>,
    );
}

#[derive(serde::Serialize)]
pub struct Archiver<T>(pub T);

impl<T, P> Component<P> for Archiver<T>
where
    P: Problem,
    T: AnyComponent + Archiving<P>,
{
    fn execute(&self, problem: P, state: &mut State) {
        todo!()
    }
}

/// Decides when to terminate.
///
/// See [crate::operators::termination] for existing implementations.
pub trait Termination<P: Problem> {
    fn terminate(&self, state: &mut State, problem: &P) -> bool;
}

#[derive(serde::Serialize)]
pub struct Terminator<T>(pub T);

impl<T, P> Component<P> for Terminator<T>
where
    P: Problem,
    T: AnyComponent + Termination<P>,
{
    fn execute(&self, problem: P, state: &mut State) {
        todo!()
    }
}

/// Can be inserted between steps.
///
/// See [crate::operators::postprocess] for existing implementations.
pub trait Postprocess<P: Problem> {
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

#[derive(serde::Serialize)]
pub struct Postprocessor<T>(pub T);

impl<T, P> Component<P> for Postprocessor<T>
where
    P: Problem,
    T: AnyComponent + Postprocess<P>,
{
    fn initialize(&self, problem: P, state: &mut State) {
        todo!()
    }

    fn execute(&self, problem: P, state: &mut State) {
        todo!()
    }
}
