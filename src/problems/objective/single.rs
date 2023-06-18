//! Objective value for a single objective (also called fitness).

use std::{cmp::Ordering, fmt::Debug};

use derive_more::{Add, Mul, Neg, Sub};
use serde::Serialize;

use crate::problems::objective::{IllegalObjective, Objective};

/// Represents a single real-valued objective (fitness).
///
/// Can be used to represent a single objective in single-objective optimization
/// or a combined objective in multi-objective optimization.
///
/// This objective type is used for [`SingleObjectiveProblem`]s.
///
/// # Restrictions
///
/// This is a wrapper around [`f64`], which can't take NaN values,
/// and therefore can implement [`Eq`] and [`Ord`].
///
/// For details, see [`IllegalObjective`].
///
/// [`SingleObjectiveProblem`]: crate::problems::SingleObjectiveProblem
#[derive(Copy, Clone, Serialize, PartialEq, PartialOrd, Add, Sub, Mul, Neg)]
pub struct SingleObjective(f64);

impl SingleObjective {
    /// Checks if the objective is `+Inf` (positive infinity) or not.
    pub fn is_finite(&self) -> bool {
        self.0.is_finite()
    }

    /// Returns the objective value as float.
    pub fn value(&self) -> f64 {
        self.0
    }
}

impl Debug for SingleObjective {
    /// Returns a string representation of the objective value.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

/// Comparison is sound because the inner value can't be NaN.
/// This can't be derived because the compiler doesn't know this.
impl Eq for SingleObjective {}

/// Ordering is sound because the inner value can't be NaN.
/// This can't be derived because the compiler doesn't know this.
#[allow(clippy::derive_ord_xor_partial_ord)]
impl Ord for SingleObjective {
    fn cmp(&self, other: &Self) -> Ordering {
        // `unwrap()` is guaranteed to never fail.
        self.partial_cmp(other).unwrap()
    }
}

impl Objective for SingleObjective {}

impl Default for SingleObjective {
    /// Creates an objective type with value [`f64::INFINITY`], which
    /// symbolizes an infinitely bad solution (minimization).
    fn default() -> Self {
        Self(f64::INFINITY)
    }
}

impl From<SingleObjective> for f64 {
    fn from(value: SingleObjective) -> Self {
        value.value()
    }
}

impl TryFrom<f64> for SingleObjective {
    type Error = IllegalObjective;

    /// Tries to convert a float into a [`SingleObjective`] value.
    ///
    /// See [`IllegalObjective`] for more information about illegal values.
    fn try_from(value: f64) -> Result<Self, Self::Error> {
        match value {
            x if x.is_nan() => Err(IllegalObjective::NaN),
            x if x.is_infinite() && x.is_sign_negative() => Err(IllegalObjective::NegativeInfinity),
            x => Ok(SingleObjective(x)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SingleObjective;

    #[test]
    fn is_finite_returns_true_for_finite() {
        assert!(SingleObjective(0.).is_finite());
        assert!(SingleObjective(-1.).is_finite());
        assert!(SingleObjective(1.).is_finite());
        assert!(SingleObjective(1e200).is_finite());
        assert!(SingleObjective(-1e200).is_finite());
    }

    #[test]
    fn is_finite_returns_false_for_infinite() {
        assert!(!SingleObjective(f64::INFINITY).is_finite());
    }

    #[test]
    fn try_from_invalid_is_err() {
        assert!(SingleObjective::try_from(f64::NAN).is_err());
        assert!(SingleObjective::try_from(f64::NEG_INFINITY).is_err());
    }
}
