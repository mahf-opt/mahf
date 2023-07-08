//! Select a subset of a population, e.g. for recombination.

use eyre::WrapErr;

use crate::{
    component::{AnyComponent, ExecResult},
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

pub trait Selection<P: Problem>: AnyComponent {
    fn select<'a>(
        &self,
        population: &'a [Individual<P>],
        rng: &mut Random,
    ) -> ExecResult<Vec<&'a Individual<P>>>;
}

erased_serde::serialize_trait_object!(<P: Problem> Selection<P>);
dyn_clone::clone_trait_object!(<P: Problem> Selection<P>);

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
