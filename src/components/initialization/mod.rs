use crate::{
    component::{AnyComponent, ExecResult},
    state::random::Random,
    Problem, State,
};

pub mod common;
pub mod functional;

pub use common::{Empty, RandomBitstring, RandomPermutation, RandomSpread};

use crate::population::IntoIndividuals;

pub trait Initialization<P: Problem>: AnyComponent {
    fn initialize(&self, problem: &P, rng: &mut Random) -> Vec<P::Encoding>;
}

erased_serde::serialize_trait_object!(<P: Problem> Initialization<P>);
dyn_clone::clone_trait_object!(<P: Problem> Initialization<P>);

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
