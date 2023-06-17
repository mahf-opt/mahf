use crate::{
    component::ExecResult,
    state::{
        lens::{Lens, LensAssign},
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

    fn map(&self, value: Self::Input, rng: &mut Random) -> ExecResult<Self::Output>;
}

pub fn mapping<P, T, I, O>(
    component: &T,
    input_lens: &I,
    output_lens: &O,
    _problem: &P,
    state: &mut State<P>,
) -> ExecResult<()>
where
    P: Problem,
    T: Mapping<P>,
    I: Lens<P, Target = T::Input>,
    O: LensAssign<P, Target = T::Output>,
{
    let input = input_lens.get(state)?;
    let result = component.map(input, &mut state.random_mut())?;
    output_lens.assign(result, state)?;
    Ok(())
}
