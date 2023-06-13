use std::fmt::Debug;

use thiserror::Error;
use trait_set::trait_set;

mod multi;
mod single;

pub use multi::MultiObjective;
pub use single::SingleObjective;

trait_set! {
    /// Collection of traits required by every objective.
    ///
    /// An objective type is a (lightweight) numerical type with (at least) a partial order.
    pub trait AnyObjective = Debug + Clone + Eq + PartialOrd + Send;
}

/// Marker trait for objective types.
///
/// The [`SingleObjective`] and [`MultiObjective`] types implement this trait and
/// cover the usual use cases for single- and multi-objective problems.
pub trait Objective: AnyObjective {}

/// Error type for illegal objective values.
/// Both `NaN` and `-Inf` are considered illegal.
///
/// # Reasons
///
/// - `NaN` is not comparable to other objective values, invalidating the required ordering.
/// - `-Inf` would symbolize a infinitely good objective value (minimization), which is not sensible.
#[derive(Debug, Error)]
pub enum IllegalObjective {
    #[error("NaN is not a valid objective value")]
    NaN,
    #[error("Negative infinity is not a valid objective value")]
    NegativeInfinity,
}
