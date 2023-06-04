//! TODO

#![allow(clippy::new_ret_no_self)]

use crate::{
    component::{AnyComponent, ExecResult},
    state::StateReq,
    Problem, State,
};

pub mod bound;
pub mod common;
pub mod logical;

pub use common::{DistanceToOptimumGreaterThan, EveryN, LessThan, OptimumReached, RandomChance};
pub use logical::{And, Not, Or};

/// A condition for loops or branches.
///
/// Similar to [Component](crate::Component), but the `evaluate` method replaces `execute` and returns a `bool`.
///
/// These can be combined using [`BitAnd`], [`BitOr`], and [`Not`] (`|`, `&`, and `!` operators).
///
/// [`BitAnd`]: std::ops::BitAnd
/// [`BitOr`]: std::ops::BitOr
/// [`Not`]: std::ops::Not
pub trait Condition<P: Problem>: AnyComponent {
    #[allow(unused_variables)]
    fn init(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        Ok(())
    }

    #[allow(unused_variables)]
    fn require(&self, problem: &P, state_req: &StateReq) -> ExecResult<()> {
        Ok(())
    }

    /// Evaluates the condition.
    fn evaluate(&self, problem: &P, state: &mut State<P>) -> ExecResult<bool>;
}

erased_serde::serialize_trait_object!(<P: Problem> Condition<P>);
dyn_clone::clone_trait_object!(<P: Problem> Condition<P>);
