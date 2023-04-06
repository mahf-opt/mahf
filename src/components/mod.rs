//! Definition and collection of components.
//!
//! Components are implementors of the [Component] trait.

#![allow(clippy::new_ret_no_self)]

use crate::{framework::AnyComponent, problems::Problem, state::State};

pub mod constraints;
pub mod control_flow;
pub use control_flow::{Block, Branch, Loop, Scope};
pub mod evaluation;
pub mod generation;
pub mod initialization;
pub mod misc;
pub mod replacement;
pub mod selection;
pub mod mapping;

/// Trait to represent a *component*, a (small) functionality with a common interface.
///
/// `Component`s are immutable, their properties describe them and can not
/// change during a run. All mutable state has to be stored in the [State].
pub trait Component<P: Problem>: AnyComponent {
    /// Can be used to initialize custom state required by the component.
    #[allow(unused_variables)]
    fn initialize(&self, problem: &P, state: &mut State<P>) {}
    /// Can be used to specify custom state requirements.
    #[allow(unused_variables)]
    fn require(&self, problem: &P, state: &State<P>) {}
    /// Executes the component, performing the actual logic.
    fn execute(&self, problem: &P, state: &mut State<P>);
}

erased_serde::serialize_trait_object!(<P: Problem> Component<P>);
dyn_clone::clone_trait_object!(<P: Problem> Component<P>);
