//! Select a subset of a population, e.g. for recombination.

use eyre::WrapErr;

use crate::{
    component::{ComponentLike, ExecResult},
    state::random::Random,
    Individual, Problem, State,
};

pub mod common;
pub mod de;
pub mod functional;
pub mod iwo;

pub use common::{
    All, CloneSingle, ExponentialRank, FullyRandom, LinearRank, None, RandomWithoutRepetition,
    RouletteWheel, StochasticUniversalSampling, Tournament,
};

/// Trait for representing a component that selects a subset of a population.
pub trait Selection<P: Problem>: ComponentLike {
    /// Selects a subset of the `population`.
    fn select<'a>(
        &self,
        population: &'a [Individual<P>],
        rng: &mut Random,
    ) -> ExecResult<Vec<&'a Individual<P>>>;
}

/// A default implementation of [`Component::execute`] for types implementing [`Selection`].
///
/// [`Component::execute`]: crate::Component::execute
pub fn selection<P, T>(component: &T, _problem: &P, state: &mut State<P>) -> ExecResult<()>
where
    P: Problem,
    T: Selection<P>,
{
    let mut populations = state.populations_mut();
    let mut rng = state.random_mut();
    let selection = component
        .select(populations.current(), &mut rng)
        .wrap_err("selection failed")?;
    let cloned_selection = selection.into_iter().cloned().collect();
    populations.push(cloned_selection);
    Ok(())
}
