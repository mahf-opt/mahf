//! Access arbitrary state without knowing the exact type of the container.
//!
//! This is especially handy if your component can work with arbitrary data of some specific
//! type ([`AnyLens::Target`]), and the caller should be able to specify at construction where
//! the data comes from.
//!
//! # Usages
//!
//! [`AnyLens`] is the base trait of several other lens traits:
//! - To access owned `Clone`-able or generated data, see [`Lens`].
//! - To access a reference to owned data, see [`LensRef`].
//! - To access a mutable reference to owned data, see [`LensMut`].
//!
//!
//! # Lenses
//!
//! This module is inspired by the concept of [`lenses`] from functional programming.
//! While its application is somewhat different from the original idea of having direct view
//! (a lens) on inner fields of (deeply) nested structures, [`AnyLens`] and subtraits
//! are a lens into the [`State`] and [`Problem`] only.
//!
//! [`lenses`]: https://rust-unofficial.github.io/patterns/functional/lenses.html

use std::{
    cell::{Ref, RefMut},
    ops::Deref,
};

use serde::Serialize;
use trait_set::trait_set;

use crate::{component::ExecResult, CustomState, Problem, State};

pub mod common;

pub use common::{IdLens, ValueOf};

/// Collection of traits required by every lens.
pub trait AnyLens: Clone + Serialize + Send + Sync + 'static {
    /// The target type of the lens.
    type Target;
}

/// Trait for extracting owned data from the state using a source specified by the caller.
///
/// Use this trait as bound if you want an owned `Target`.
///
/// # Examples
///
/// Using the trait as trait bound to work with any integer in a component:
///
/// ```
/// use serde::Serialize;
/// use mahf::lens::{AnyLens, Lens};
/// use mahf::prelude::*;
///
/// #[derive(Clone, Serialize)]
/// struct SomeComponentInvolvingAnInteger<I: AnyLens> {
///     pub lens: I,
/// }
///
/// impl<I: AnyLens> SomeComponentInvolvingAnInteger<I> {
///     pub fn from_params(lens: I) -> Self {
///         Self { lens }
///     }
///
///     pub fn new<P>(lens: I) -> Box<dyn Component<P>>
///     where
///         P: Problem,
///         I: Lens<P, Target = u32>,
///     {
///         Box::new(Self::from_params(lens))
///     }
/// }
///
/// impl<P, I, > Component<P> for SomeComponentInvolvingAnInteger<I>
/// where
///     P: Problem,
///     // Specify that you just want an `u32`, and the caller should specify where it comes from.
///     I: Lens<P, Target = u32>,
/// {
///     fn execute(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
///         let some_integer: u32 = self.lens.get(problem, state)?;
///         // Do some calculation using the integer.
///         Ok(())
///     }
/// }
///
/// # pub fn example<P: Problem>() -> Configuration<P> {
/// // `SomeComponentInvolvingAnInteger` can work with any lens that extracts an `u32`.
/// Configuration::builder()
///     // Uses the number of iterations as `some_integer`.
///     .do_(SomeComponentInvolvingAnInteger::new(ValueOf::<common::Iterations>::new()))
///     // Uses the number of evaluations as `some_integer`.
///     .do_(SomeComponentInvolvingAnInteger::new(ValueOf::<common::Evaluations>::new()))
///     .build()
/// # }
/// ```
///
/// Implementing your own lens on a field of some [`CustomState`] with multiple fields:
///
/// ```
/// use better_any::{Tid, TidAble};
/// use serde::Serialize;
/// use mahf::{CustomState, ExecResult, Individual, Problem, State};
/// use mahf::lens::{AnyLens, Lens};
///
/// #[derive(Tid)]
/// pub struct StateWithManyFields<P: Problem + 'static> {
///     pub integer: u32,
///     pub float: f64,
///     pub individual: Individual<P>,
/// }
/// impl<P: Problem> CustomState<'_> for StateWithManyFields<P> {}
///
/// #[derive(Clone, Serialize)]
/// pub struct StateWithManyFieldsFloatLens;
///
/// impl AnyLens for StateWithManyFieldsFloatLens {
///     type Target = f64;
/// }
///
/// impl<P: Problem> Lens<P> for StateWithManyFieldsFloatLens {
///     fn get(&self, problem: &P, state: &State<P>) -> ExecResult<Self::Target> {
///         Ok(state.try_borrow::<StateWithManyFields<P>>()?.float)
///     }
/// }
/// ```
pub trait Lens<P: Problem>: AnyLens {
    /// Tries to extract an owned value from the `problem` and/or `state`.
    fn get(&self, problem: &P, state: &State<P>) -> ExecResult<Self::Target>;
}

/// Trait for extracting owned data from a given source type.
///
/// Implementing this trait automatically implements [`Lens`], where `Source` is extracted
/// from the [`State`].
///
/// # Examples
///
/// TODO
pub trait LensMap: AnyLens {
    /// The source type to map to `Target` from.
    type Source;

    /// Maps from `&Source` to `Target`.
    fn map(&self, source: &Self::Source) -> Self::Target;
}

impl<P, E> Lens<P> for E
where
    P: Problem,
    E: LensMap,
    E::Source: for<'a> CustomState<'a>,
{
    fn get(&self, _problem: &P, state: &State<P>) -> ExecResult<Self::Target> {
        Ok(self.map(state.try_borrow::<E::Source>()?.deref()))
    }
}

/// Trait for extracting a reference to data from the state using a source specified by the caller.
///
/// Use this trait as bound if you want a `Ref<Target>`.
///
/// # Examples
///
/// Using the trait as trait bound to extract a reference to some type implementing a trait:
///
/// TODO
///
/// Implementing your own lens:
///
/// TODO
pub trait LensRef<P: Problem>: AnyLens {
    /// Tries to extract a reference to a value from the `problem` and/or `state`.
    fn get_ref<'a>(&self, problem: &P, state: &'a State<P>) -> ExecResult<Ref<'a, Self::Target>>;
}

/// Trait for extracting a reference to data from a given source type.
///
/// Implementing this trait automatically implements [`LensRef`], where `Ref<Source>` is extracted
/// from the [`State`].
///
/// # Examples
///
/// TODO
pub trait LensMapRef: AnyLens {
    /// The source type to map to `&Target` from.
    type Source;

    /// Maps from `&Source` to `&Target`.
    fn map<'a>(&self, source: &'a Self::Source) -> &'a Self::Target;
}

impl<P, E> LensRef<P> for E
where
    P: Problem,
    E: LensMapRef,
    E::Source: for<'a> CustomState<'a>,
{
    fn get_ref<'a>(&self, _problem: &P, state: &'a State<P>) -> ExecResult<Ref<'a, Self::Target>> {
        Ok(Ref::map(state.try_borrow::<E::Source>()?, |source| {
            self.map(source)
        }))
    }
}

/// Trait for extracting a mutable reference to data from the state using a source specified by the caller.
///
/// Use this trait as bound if you want a `RefMut<Target>`.
///
/// # Examples
///
/// Using the trait as trait bound to extract a mutable reference to some type implementing a trait:
///
/// TODO
///
/// Implementing your own lens:
///
/// TODO
pub trait LensMut<P: Problem>: LensRef<P> {
    /// Tries to extract a mutable reference to a value from the `problem` and/or `state`.
    fn get_mut<'a>(&self, problem: &P, state: &'a State<P>)
        -> ExecResult<RefMut<'a, Self::Target>>;
}

/// Trait for assigning an owned value to data from the state using a source specified by the caller.
///
/// Use this trait as bound if you want to assign to `Target`.
///
/// This trait is automatically implemented for lenses which implement [`LensMut`].
///
/// # Examples
///
/// Using the trait as trait bound to assign a value to some integer:
///
/// TODO
///
pub trait LensAssign<P: Problem>: LensMut<P> {
    /// Tries to assign `value` to the `state`.
    fn assign(&self, value: Self::Target, problem: &P, state: &State<P>) -> ExecResult<()>;
}

impl<P, E> LensAssign<P> for E
where
    P: Problem,
    E: LensMut<P>,
{
    fn assign(&self, value: Self::Target, problem: &P, state: &State<P>) -> ExecResult<()> {
        *self.get_mut(problem, state)? = value;
        Ok(())
    }
}

trait_set! {
    /// Trait for extracting and assigning an owned value to data from the state using a source specified by the caller.
    pub trait ValueLens<P: Problem, T> = Lens<P, Target=T> + LensAssign<P, Target=T>;
}
