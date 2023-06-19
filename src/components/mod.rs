//! TODO

#![allow(clippy::new_ret_no_self)]

use crate::{
    component::{AnyComponent, ExecResult},
    state::StateReq,
    Problem, State,
};

pub mod archive;
pub mod boundary;
pub mod control_flow;
pub mod cro;
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

pub use control_flow::{Block, Branch, Loop, Scope};

/// Trait to represent a *component*, a (small) functionality with a common interface.
///
/// `Component`s are immutable, their properties describe them and can not
/// change during a run. All mutable state has to be stored in the [State].
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
