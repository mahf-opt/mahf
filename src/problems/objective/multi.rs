//! Objective vector with multiple objectives.

use std::{cmp::Ordering, fmt::Debug};

use serde::Serialize;

use crate::problems::objective::{IllegalObjective, Objective};

/// Represents multiple real-valued objectives and can be used to
/// represent an objective vector in multi-objective optimization.
///
/// This objective type is used for [`MultiObjectiveProblem`]s.
///
/// # Restrictions
///
/// This is a wrapper around [`Vec<f64>`], which can't take NaN values,
/// and therefore can implement [`Eq`].
///
/// For details, see [`IllegalObjective`].
///
/// The [PartialOrd] implementation returns the Pareto ordering.
/// As the Pareto ordering leaves some vectors as incomparable, [`Ord`] is not implemented.
///
/// [`MultiObjectiveProblem`]: crate::problems::MultiObjectiveProblem
#[derive(Clone, Serialize, PartialEq)]
pub struct MultiObjective(Vec<f64>);

impl MultiObjective {
    /// Checks if the objective values contain `+Inf` (positive infinity) or not.
    pub fn is_finite(&self) -> bool {
        self.0.iter().all(|o| o.is_finite())
    }

    /// Returns the objective value as a slice of floats.
    pub fn value(&self) -> &[f64] {
        &self.0
    }
}

impl Debug for MultiObjective {
    /// Returns a string representation of the objective values.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl Eq for MultiObjective {}

impl PartialOrd for MultiObjective {
    /// Orders two objective value vectors using Pareto-domination.
    ///
    /// Note that [Ordering::Less] means that `self` dominates `other`:
    /// ```
    /// use std::cmp::Ordering;
    ///
    /// use mahf::MultiObjective;
    ///
    /// let a = MultiObjective::try_from(vec![0., 0.]).unwrap();
    /// let b = MultiObjective::try_from(vec![1., 1.]).unwrap();
    ///
    /// // `a` dominates `b` (minimization).
    /// assert_eq!(a.partial_cmp(&b), Some(Ordering::Less));
    /// ```
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Use Eq checking for equality
        if self == other {
            return Some(Ordering::Equal);
        }

        // No comparison possible if length doesn't match.
        if self.value().len() != other.value().len() {
            return None;
        }

        // Check all dimensions.
        let mut has_better = false;
        let mut has_worse = false;

        for (own, other) in self.value().iter().zip(other.value()) {
            if own < other {
                has_better = true;
            } else if own > other {
                has_worse = true;
            }
        }

        match (has_better, has_worse) {
            // `self` dominates on all indices.
            (true, false) => Some(Ordering::Less),
            // `other` dominates on all indices.
            (false, true) => Some(Ordering::Greater),
            // `self´ and `other` are incomparable.
            _ => None,
        }
    }
}

impl Objective for MultiObjective {}

impl From<MultiObjective> for Vec<f64> {
    fn from(objective: MultiObjective) -> Self {
        objective.0
    }
}

impl TryFrom<Vec<f64>> for MultiObjective {
    type Error = IllegalObjective;

    /// Tries to convert a vector of floats into a `MultiObjective` value.
    ///
    /// See [`IllegalObjective`] for more information about illegal values.
    fn try_from(value: Vec<f64>) -> Result<Self, IllegalObjective> {
        match value {
            x if x.iter().any(|o| o.is_nan()) => Err(IllegalObjective::NaN),
            x if x.iter().any(|o| o.is_infinite() && o.is_sign_negative()) => {
                Err(IllegalObjective::NegativeInfinity)
            }
            _ => Ok(MultiObjective(value)),
        }
    }
}

impl TryFrom<&[f64]> for MultiObjective {
    type Error = IllegalObjective;

    /// Tries to convert a vector of floats into a `MultiObjective` value.
    ///
    /// See [`IllegalObjective`] for more information about illegal values.
    fn try_from(value: &[f64]) -> Result<Self, IllegalObjective> {
        match value {
            x if x.iter().any(|o| o.is_nan()) => Err(IllegalObjective::NaN),
            x if x.iter().any(|o| o.is_infinite() && o.is_sign_negative()) => {
                Err(IllegalObjective::NegativeInfinity)
            }
            _ => Ok(MultiObjective(value.into())),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use super::MultiObjective;

    #[test]
    fn is_finite_returns_true_for_all_finite() {
        assert!(MultiObjective(vec![0., 0.]).is_finite());
        assert!(MultiObjective(vec![-1., 1.]).is_finite());
    }

    #[test]
    fn is_finite_returns_false_for_any_infinite() {
        assert!(!MultiObjective(vec![0., f64::INFINITY]).is_finite());
        assert!(!MultiObjective(vec![f64::INFINITY, f64::INFINITY]).is_finite());
    }

    #[test]
    fn partial_ord_implements_pareto_domination() {
        let a = MultiObjective(vec![0., 0.]);
        let b = MultiObjective(vec![1., 1.]);
        let c = MultiObjective(vec![1., 0.]);
        let d = MultiObjective(vec![0., 1.]);

        // All are equal to self.
        assert_eq!(a.partial_cmp(&a), Some(Ordering::Equal));
        assert_eq!(b.partial_cmp(&b), Some(Ordering::Equal));
        assert_eq!(c.partial_cmp(&c), Some(Ordering::Equal));
        assert_eq!(d.partial_cmp(&d), Some(Ordering::Equal));

        // `a` dominates `b`, `c`, and `d`.
        assert_eq!(a.partial_cmp(&b), Some(Ordering::Less));
        assert_eq!(a.partial_cmp(&c), Some(Ordering::Less));
        assert_eq!(a.partial_cmp(&d), Some(Ordering::Less));

        // `b` is dominated by `a`, `c`, and `d`.
        assert_eq!(b.partial_cmp(&a), Some(Ordering::Greater));
        assert_eq!(b.partial_cmp(&c), Some(Ordering::Greater));
        assert_eq!(b.partial_cmp(&d), Some(Ordering::Greater));

        // `c` is dominated by `a`, dominates `b` and incomparable with `d`.
        assert_eq!(c.partial_cmp(&a), Some(Ordering::Greater));
        assert_eq!(c.partial_cmp(&b), Some(Ordering::Less));
        assert_eq!(c.partial_cmp(&d), None);

        // `d` is dominated by `a`, dominates `b` and incomparable with `c`.
        assert_eq!(d.partial_cmp(&a), Some(Ordering::Greater));
        assert_eq!(d.partial_cmp(&b), Some(Ordering::Less));
        assert_eq!(d.partial_cmp(&c), None);
    }

    #[test]
    fn try_from_invalid_is_err() {
        assert!(MultiObjective::try_from(vec![f64::NAN, 0.]).is_err());
        assert!(MultiObjective::try_from(vec![f64::NAN, f64::NEG_INFINITY]).is_err());

        assert!(MultiObjective::try_from(vec![f64::NEG_INFINITY, 0.]).is_err());
        assert!(MultiObjective::try_from(vec![f64::NEG_INFINITY, f64::NEG_INFINITY]).is_err());
    }
}
