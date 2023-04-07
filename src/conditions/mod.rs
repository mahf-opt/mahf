//! Collection of common conditions.
//!
//! Conditions are implementors of the [Condition] trait.

use crate::{framework::AnyComponent, problems::Problem, state::State};

pub mod branching;

pub mod logic;
pub use logic::{And, Not, Or};

pub mod termination;

/// A condition for loops or branches.
///
/// Similar to [Component](crate::Component),
/// but `evaluate` replaces `execute` and returns a `bool`.
///
/// These can be combined using binary AND and OR, and NOT (`|`, `&`, and `!`).
pub trait Condition<P: Problem>: AnyComponent {
    #[allow(unused_variables)]
    fn initialize(&self, problem: &P, state: &mut State<P>) {}
    #[allow(unused_variables)]
    fn require(&self, problem: &P, state: &State<P>) {}
    fn evaluate(&self, problem: &P, state: &mut State<P>) -> bool;
}

erased_serde::serialize_trait_object!(<P: Problem> Condition<P>);
dyn_clone::clone_trait_object!(<P: Problem> Condition<P>);
