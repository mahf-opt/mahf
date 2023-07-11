//! Initialize solutions in the search space.

use crate::{
    component::{AnyComponent, ExecResult},
    state::random::Random,
    Problem, State,
};

pub mod common;
pub mod functional;

pub use common::{Empty, RandomBitstring, RandomPermutation, RandomSpread};

use crate::population::IntoIndividuals;

/// Trait for representing a component that initializes solutions in the search space.
pub trait Initialization<P: Problem>: AnyComponent {
    /// Initializes solutions in the search space.
    fn initialize(&self, problem: &P, rng: &mut Random) -> Vec<P::Encoding>;
}

/// A default implementation of [`Component::execute`] for types implementing [`Initialization`].
///
/// [`Component::execute`]: crate::Component::execute
pub fn initialization<P, T>(component: &T, problem: &P, state: &mut State<P>) -> ExecResult<()>
where
    P: Problem,
    T: Initialization<P>,
{
    let population = component
        .initialize(problem, &mut state.random_mut())
        .into_individuals();
    state.populations_mut().push(population);
    Ok(())
}
