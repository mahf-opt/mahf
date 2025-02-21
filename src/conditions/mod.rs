//! Metaheuristic algorithm conditions.
//!
//! This module contains definition and implementation of [`Condition`]s.

#![allow(clippy::new_ret_no_self)]

use crate::{
    component::{AnyComponent, ExecResult},
    state::StateReq,
    Problem, State,
};

pub mod common;
pub mod cro;
pub mod logical;

pub use common::{ChangeOf, EqualToN, EveryN, LessThanN, OptimumReached, RandomChance, StagnationForN};
pub use logical::{And, Not, Or};

/// Trait to represent a condition *component* for loops or branches.
///
/// This is the twin trait to [`Component`], with the difference that the `evaluate`
/// method replaces `execute` and returns a `bool`.
///
/// Specifically, everything in the documentation of [`Component`] also applies to `Condition`s.
///
/// [`Component`]: crate::Component
///
/// # Combining conditions
///
/// `Condition`s can be combined using [`BitAnd`], [`BitOr`], and [`Not`] (`|`, `&`, and `!` operators).
///
/// [`BitAnd`]: std::ops::BitAnd
/// [`BitOr`]: std::ops::BitOr
/// [`Not`]: std::ops::Not
pub trait Condition<P: Problem>: AnyComponent {
    /// Can be used to initialize custom state required by the condition.
    #[allow(unused_variables)]
    fn init(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        Ok(())
    }

    /// Can be used to specify custom state requirements.
    #[allow(unused_variables)]
    fn require(&self, problem: &P, state_req: &StateReq<P>) -> ExecResult<()> {
        Ok(())
    }

    /// Evaluates the condition, performing the actual logic.
    fn evaluate(&self, problem: &P, state: &mut State<P>) -> ExecResult<bool>;
}

erased_serde::serialize_trait_object!(<P: Problem> Condition<P>);
dyn_clone::clone_trait_object!(<P: Problem> Condition<P>);
