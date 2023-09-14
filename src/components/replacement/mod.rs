//! Replace a parent population with its child population.

use eyre::WrapErr;

use crate::{
    component::{AnyComponent, ExecResult},
    state::random::Random,
    Individual, Problem, State,
};

pub mod common;
pub mod bh;
pub mod sa;

pub use common::{
    DiscardOffspring, Generational, KeepBetterAtIndex, Merge, MuPlusLambda, RandomReplacement,
};

/// Trait for representing a component that replaces a parent population with its child population.
pub trait Replacement<P: Problem>: AnyComponent {
    fn replace(
        &self,
        parents: Vec<Individual<P>>,
        offspring: Vec<Individual<P>>,
        rng: &mut Random,
    ) -> ExecResult<Vec<Individual<P>>>;
}

/// A default implementation of [`Component::execute`] for types implementing [`Replacement`].
///
/// [`Component::execute`]: crate::Component::execute
pub fn replacement<P, T>(component: &T, _problem: &P, state: &mut State<P>) -> ExecResult<()>
where
    P: Problem,
    T: Replacement<P>,
{
    let mut populations = state.populations_mut();
    let offspring = populations.pop();
    let parents = populations.pop();
    let population = component
        .replace(parents, offspring, &mut state.random_mut())
        .wrap_err("replacement failed")?;
    populations.push(population);
    Ok(())
}
