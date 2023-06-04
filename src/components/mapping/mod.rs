use crate::{
    component::ExecResult,
    state::{
        extract::{Assign, Extract},
        random::Random,
    },
    Problem, State,
};

pub mod common;
pub mod sa;

pub use common::{Linear, Polynomial, RandomRange};

pub trait Mapping<P: Problem> {
    type Input;
    type Output;

    fn map(&self, value: &Self::Input, rng: &mut Random) -> ExecResult<Self::Output>;
}

pub fn mapping<P, T, I, O>(component: &T, _problem: &P, state: &mut State<P>) -> ExecResult<()>
where
    P: Problem,
    T: Mapping<P>,
    I: Extract<Target = T::Input>,
    O: Assign<Target = T::Output>,
{
    let input = I::extract(state)?;
    let result = component.map(&input, &mut state.random_mut())?;
    O::assign(result, state)?;
    Ok(())
}
