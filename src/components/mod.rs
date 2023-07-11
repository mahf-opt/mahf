//! Metaheuristic algorithm components.
//!
//! This module contains definition and implementation of [`Component`]s.

#![allow(clippy::new_ret_no_self)]

use crate::{
    component::{AnyComponent, ExecResult},
    state::StateReq,
    Problem, State,
};

pub mod archive;
pub mod boundary;
pub mod control_flow;
pub mod diversity;
pub mod evaluation;
pub mod generative;
pub mod initialization;
pub mod mapping;
pub mod misc;
pub mod mutation;
pub mod recombination;
pub mod replacement;
pub mod selection;
pub mod swarm;
pub mod utils;

pub use control_flow::{Block, Branch, Loop, Scope};

/// Trait to represent a *component*, a (small) functionality with a uniform interface.
///
/// `Component`s encapsulate any functionality that may arise in the context of metaheuristics,
/// which includes evolutionary operators, any metaheuristic-specific operator,
/// logging during the execution, calculation of metrics, and even control flow itself.
///
/// # Phases
///
/// `Component`s define three phases:
/// - `initialize` (optional): Used to initialize custom state, and is only called once*.
/// - `require` (optional): Used to check if the custom state was properly initialized in the
/// `initialize` phase, and is only called once*.
/// Specifically used to check if necessary custom state initialized by other components is present.
/// - `execute`: Executes the functionality the component constitutes.
/// This phase is usually called in a loop during the optimization process.
///
/// *More complex control flow might reinitialize components during execution, which means that
/// `initialize` and `require` get called multiple times.
/// See their documentation for more details.
///
/// # State
///
/// The interface of `Component`s can be this generic because the [`State`] erases the
/// types of the state contained within.
/// It therefore can be interpreted as an interface checked at runtime, as it
/// internally relies on boxed [`CustomState`] objects.
///
/// `Component`s are immutable and fully described by their parameters.
/// Therefore, all mutable state has to be stored in the [`State`].
///
/// [`CustomState`]: crate::CustomState
///
/// # Panic versus error
///
/// All methods on this trait return an [`ExecResult`], and therefore may fail.
/// In general, they should fail when the interface of the component was violated
/// (by other components).
///
/// It is advised to add an error description before propagating the error using the
/// [`wrap_err`] method or similar.
///
/// If a `Component` panics, it is assumed to be because of an internal implementation error,
/// not because its interface was violated.
///
/// [`wrap_err`]: eyre::WrapErr::wrap_err
///
/// # Construction
///
/// Contrary to [Rust convention], the `new` method of component structs should return a
/// `Box<dyn Component<P>>`, not `Self`, as components are usually used only in their boxed form.
/// `Component`s typically define a `from_params` method that is used to construct `Self` directly, e.g. for testing.
///
/// [Rust convention]: https://rust-unofficial.github.io/patterns/idioms/ctor.html
///
/// # Examples
///
/// A simple component that prints the current population:
///
/// ```
/// use std::fmt::Debug;
///
/// use mahf::{Component, ExecResult, Problem, State};
/// use serde::Serialize;
///
/// #[derive(Clone, Serialize)]
/// pub struct PrintPopulation;
///
/// impl PrintPopulation {
///     pub fn from_params() -> Self {
///         Self
///     }
///
///     pub fn new<P>() -> Box<dyn Component<P>>
///     where
///         P: Problem,
///         P::Encoding: Debug,
///     {
///         Box::new(Self::from_params())
///     }
/// }
///
/// impl<P> Component<P> for PrintPopulation
/// where
///     P: Problem,
///     P::Encoding: Debug,
/// {
///     fn execute(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
///         println!("Current population: {:?}", state.populations().current());
///         Ok(())
///     }
/// }
/// ```
pub trait Component<P: Problem>: AnyComponent {
    /// Can be used to initialize custom state required by the component.
    #[allow(unused_variables)]
    fn init(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        Ok(())
    }

    /// Can be used to specify custom state requirements.
    #[allow(unused_variables)]
    fn require(&self, problem: &P, state_req: &StateReq<P>) -> ExecResult<()> {
        Ok(())
    }

    /// Executes the component, performing the actual logic.
    fn execute(&self, problem: &P, state: &mut State<P>) -> ExecResult<()>;
}

erased_serde::serialize_trait_object!(<P: Problem> Component<P>);
dyn_clone::clone_trait_object!(<P: Problem> Component<P>);
