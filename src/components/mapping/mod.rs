//! Map from some input state to an output state.
//!
//! You can think of a mapping as a function `f` that maps from input `X` to output `Y`
//! ```math
//! f: X -> Y ,
//! ```
//! where `X` and `Y` are the `T::Target` of some input lens `I` and output lens `O`, respectively.
//!
//! The input value `x: X` is retrieved using the input lens `I`, mapped using `f`, and then
//! assigned to the location `y: Y` specified by the output lens `O`.
//!
//! Note that a mapping only defines `f` (and maybe specifies bounds on `X` and `Y`), but
//! the caller decides what `x` and `y` actually are.

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
