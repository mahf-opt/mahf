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
//!
//! # Example
//!
//! A specific example for this is adapting the [`InertiaWeight`] used by [`ParticleVelocitiesUpdate`].
//! A common way to adapt the inertia weight of PSO at runtime is to decrease it linearly
//! over the course of iterations, e.g. from 0.9 to 0.4.
//!
//! The [`Linear`] mapping can be used to implement this,
//! using [`ValueOf<Progress<ValueOf<Iterations>>>`] as input lens and
//! [`ValueOf<InertiaWeight>`] as output lens:
//!
//! [`InertiaWeight`]: crate::components::swarm::InertiaWeight
//! [`ParticleVelocitiesUpdate`]: crate::components::swarm::ParticleVelocitiesUpdate
//! [`ValueOf<Progress<ValueOf<Iterations>>>`]: crate::lens::ValueOf
//! [`ValueOf<InertiaWeight>`]: crate::lens::ValueOf
//!
//! ```
//! use mahf::{
//!     components::{mapping::Linear, swarm},
//!     lens::ValueOf,
//!     state::common,
//! #    Component, Problem,
//! };
//!
//! # fn example<P: Problem>() -> Box<dyn Component<P>> {
//! Linear::new(
//!     0.4,
//!     0.9,
//!     ValueOf::<common::Progress<ValueOf<common::Iterations>>>::new(),
//!     ValueOf::<swarm::InertiaWeight<swarm::ParticleVelocitiesUpdate>>::new(),
//! )
//! # }
//! ```

use crate::{
    component::ExecResult,
    lens::{Lens, LensAssign},
    state::random::Random,
    Problem, State,
};

pub mod common;
pub mod sa;

pub use common::{Linear, Polynomial, RandomRange};

/// Trait for representing a component that maps from some input state to an output state.
pub trait Mapping<P: Problem> {
    /// The input type.
    type Input;
    /// The output type.
    type Output;
    /// Maps from the `Input` to the `Output`.
    fn map(&self, value: Self::Input, rng: &mut Random) -> ExecResult<Self::Output>;
}

/// A default implementation of [`Component::execute`] for types implementing [`Mapping`].
///
/// [`Component::execute`]: crate::Component::execute
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
