//! Mutate solutions.
//!
//! # Parameters
//!
//! A common pattern for mutation components is to store their mutation rate and -strength in the
//! [`MutationRate`] and [`MutationStrength`] states for adaptation.
//!
//! See the documentation of the respective components for more information.

use std::marker::PhantomData;

use better_any::{Tid, TidAble};
use derive_more::{Deref, DerefMut};
use eyre::ensure;

use crate::{
    component::{ComponentLike, ExecResult},
    population::AsSolutionsMut,
    CustomState, Problem, State,
};

pub mod common;
pub mod de;
pub mod functional;

pub use common::{
    BitFlipMutation, InversionMutation, NormalMutation, PartialRandomBitstring,
    PartialRandomSpread, ScrambleMutation, SwapMutation, TranslocationMutation, UniformMutation,
};

/// Trait for representing a component that mutates solutions.
pub trait Mutation<P: Problem>: ComponentLike {
    fn mutate(
        &self,
        solution: &mut P::Encoding,
        problem: &P,
        state: &mut State<P>,
    ) -> ExecResult<()>;
}

/// A default implementation of [`Component::execute`] for types implementing [`Mutation`].
///
/// [`Component::execute`]: crate::Component::execute
pub fn mutation<T, P>(component: &T, problem: &P, state: &mut State<P>) -> ExecResult<()>
where
    P: Problem,
    T: Mutation<P>,
{
    let mut population = state.populations_mut().pop();
    for solution in population.as_solutions_mut() {
        component.mutate(solution, problem, state)?;
    }
    state.populations_mut().push(population);

    Ok(())
}

/// The mutation strength of a mutation component `T`.
#[derive(Deref, DerefMut, Tid)]
pub struct MutationStrength<T: ComponentLike + 'static>(
    #[deref]
    #[deref_mut]
    f64,
    PhantomData<T>,
);

impl<T: ComponentLike> MutationStrength<T> {
    /// Creates a new `MutationStrength` with initial `value`.
    pub fn new(value: f64) -> Self {
        Self(value, PhantomData)
    }
}

impl<T: ComponentLike> CustomState<'_> for MutationStrength<T> {}

/// The mutation rate of a mutation component `T`.
#[derive(Deref, DerefMut, Tid)]
pub struct MutationRate<T: ComponentLike + 'static>(
    #[deref]
    #[deref_mut]
    f64,
    PhantomData<T>,
);

impl<T: ComponentLike> MutationRate<T> {
    /// Creates a new `MutationRate` with initial `value`.
    pub fn new(value: f64) -> Self {
        Self(value, PhantomData)
    }

    /// Returns the mutation rate, and `Err` if it is not within `[0, 1]`.
    pub fn value(&self) -> ExecResult<f64> {
        ensure!(
            (0.0..=1.0).contains(&self.0),
            "mutation rate must be in [0, 1]"
        );
        Ok(self.0)
    }
}

impl<T: ComponentLike> CustomState<'_> for MutationRate<T> {}
