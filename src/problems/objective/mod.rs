//! Objective types.

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
/// - `-Inf` would symbolize a infinitely good objective value for minimization, which is not sensible to allow.
///
/// # Examples
///
/// Constructing objective values from invalid values fails:
///
/// ```
/// use mahf::{MultiObjective, SingleObjective};
///
/// assert!(SingleObjective::try_from(f64::NAN).is_err());
/// assert!(SingleObjective::try_from(f64::NEG_INFINITY).is_err());
///
/// assert!(MultiObjective::try_from(vec![f64::NAN, 0.0]).is_err());
/// assert!(MultiObjective::try_from(vec![0.0, f64::NEG_INFINITY]).is_err());
/// ```
#[derive(Debug, Error)]
pub enum IllegalObjective {
    /// `NaN` (Not A Number)
    #[error("NaN is not a valid objective value")]
    NaN,
    /// `-Inf` (Negative Infinity)
    #[error("Negative infinity is not a valid objective value")]
    NegativeInfinity,
}
