//! Framework components.

use crate::{
    heuristic::{Individual, State},
    problem::Problem,
};

/// Initializes the population.
pub trait Initialization<P: Problem> {
    fn initialize(&mut self, problem: &P, population: &mut Vec<P::Encoding>);
}

/// Selects individuals for reproduction or modification.
pub trait Selection {
    fn select<'p>(
        &mut self,
        state: &mut State,
        population: &'p [Individual],
        selection: &mut Vec<&'p Individual>,
    );
}

/// Generates new solutions from the selected population.
pub trait Generation<P: Problem> {
    fn generate(
        &mut self,
        state: &mut State,
        problem: &P,
        parents: &mut Vec<&P::Encoding>,
        offspring: &mut Vec<P::Encoding>,
    );
}

/// Replaces old individuals with new ones.
pub trait Replacement {
    fn replace(
        &mut self,
        state: &mut State,
        population: &mut Vec<Individual>,
        offspring: &mut Vec<Individual>,
    );
}

/// Decides when to terminate.
pub trait Termination {
    fn terminate(&mut self, state: &mut State) -> bool;
}
