//! Framework components.

use crate::{
    dynser::Serialize,
    heuristic::{Individual, State},
    problem::Problem,
};
use std::any::Any;

/// Defines the traits required by any component.
///
/// This will be implemented automatically for all structs satisfying the requirements.
///
/// # Any
/// All components must allow downcasting and thus require [Any].
///
/// # Serialize
/// [Serialize] allows serializing dynamic components for the purpose of logging.
///
/// # Send
/// Most of the time, execution should be multi threaded and having
/// components implement [Send] makes this much easier.
///
pub trait Component: Any + Serialize + Send {}
impl<T> Component for T where T: Any + Serialize + Send {}

/// Initializes the population.
pub trait Initialization<P: Problem>: Component {
    fn initialize(&self, problem: &P, population: &mut Vec<P::Encoding>);
}

/// Selects individuals for reproduction or modification.
pub trait Selection: Component {
    fn select<'p>(
        &self,
        state: &mut State,
        population: &'p [Individual],
        selection: &mut Vec<&'p Individual>,
    );
}

/// Generates new solutions from the selected population.
pub trait Generation<P: Problem>: Component {
    fn generate(
        &self,
        state: &mut State,
        problem: &P,
        parents: &mut Vec<&P::Encoding>,
        offspring: &mut Vec<P::Encoding>,
    );
}

/// Replaces old individuals with new ones.
pub trait Replacement: Component {
    fn replace(
        &self,
        state: &mut State,
        population: &mut Vec<Individual>,
        offspring: &mut Vec<Individual>,
    );
}

/// Decides when to terminate.
pub trait Termination: Component {
    fn terminate(&self, state: &mut State) -> bool;
}
