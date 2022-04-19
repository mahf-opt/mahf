#![allow(unused_variables)]
//! Framework components.

use crate::{
    framework::{
        components::{AnyComponent, Component, Condition},
        state::State,
        Fitness, Individual,
    },
    problems::Problem,
    random::Random,
};
use serde::Serialize;

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

#[derive(Serialize)]
pub struct Initializer<T>(pub T);

impl<T, P> Component<P> for Initializer<T>
where
    P: Problem,
    T: AnyComponent + Initialization<P> + Serialize,
{
    fn execute(&self, problem: &P, state: &mut State) {
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
    T: AnyComponent + Selection + Serialize,
{
    fn execute(&self, problem: &P, state: &mut State) {
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
    T: AnyComponent + Generation<P> + Serialize,
{
    fn execute(&self, problem: &P, state: &mut State) {
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
    T: AnyComponent + Scheduler + Serialize,
{
    fn execute(&self, problem: &P, state: &mut State) {
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
    T: AnyComponent + Replacement + Serialize,
{
    fn execute(&self, problem: &P, state: &mut State) {
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
    T: AnyComponent + Archiving<P> + Serialize,
{
    fn execute(&self, problem: &P, state: &mut State) {
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

impl<T, P> Condition<P> for Terminator<T>
where
    P: Problem,
    T: AnyComponent + Termination<P> + Serialize,
{
    fn evaluate(&self, problem: &P, state: &mut State) -> bool {
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
    T: AnyComponent + Postprocess<P> + Serialize,
{
    fn initialize(&self, problem: &P, state: &mut State) {
        todo!()
    }

    fn execute(&self, problem: &P, state: &mut State) {
        todo!()
    }
}

/// Evaluates solutions.
///
/// Can be used to customize how solutions should be evaluated.
/// One use case for this would be GPU evaluation.
pub trait Evaluator<P: Problem> {
    fn evaluate(
        &mut self,
        state: &mut State,
        problem: &P,
        offspring: &mut Vec<P::Encoding>,
        evaluated: &mut Vec<Individual>,
    );
}

pub struct SimpleEvaluator;
impl<P: Problem> Evaluator<P> for SimpleEvaluator {
    fn evaluate(
        &mut self,
        _state: &mut State,
        problem: &P,
        offspring: &mut Vec<P::Encoding>,
        evaluated: &mut Vec<Individual>,
    ) {
        for solution in offspring.drain(..) {
            let fitness = Fitness::try_from(problem.evaluate(&solution)).unwrap();
            evaluated.push(Individual::new::<P::Encoding>(solution, fitness));
        }
    }
}
