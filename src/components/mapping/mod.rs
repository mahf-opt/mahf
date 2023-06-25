use crate::{
    component::ExecResult,
    lens::{Lens, LensAssign},
    state::random::Random,
    Problem, State,
};

pub mod common;
pub mod sa;

pub use common::{Linear, Polynomial, RandomRange};

pub trait Mapping<P: Problem> {
    type Input;
    type Output;

    fn map(&self, value: Self::Input, rng: &mut Random) -> ExecResult<Self::Output>;
}

pub fn mapping<P, T, I, O>(
    component: &T,
    input_lens: &I,
    output_lens: &O,
    problem: &P,
    state: &mut State<P>,
) -> ExecResult<()>
where
    P: Problem,
    T: Mapping<P>,
    I: Lens<P, Target = T::Input>,
    O: LensAssign<P, Target = T::Output>,
{
    let input = input_lens.get(problem, state)?;
    let result = component.map(input, &mut state.random_mut())?;
    output_lens.assign(result, problem, state)?;
    Ok(())
}
