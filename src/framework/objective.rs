//! Utility type to store an individuals fitness.

use std::any::Any;
use std::fmt;

pub trait Objective: fmt::Debug + Clone + Eq + Any {}

/// Error type for illegal objective values.
///
/// Currently, `NaN` and `-Inf` are considered illegal.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IllegalObjective {
    NaN,
    NegativeInfinity,
}

impl fmt::Display for IllegalObjective {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            fmt,
            "illegal objective: {}",
            match self {
                IllegalObjective::NaN => "NaN",
                IllegalObjective::NegativeInfinity => "-Inf",
            },
        )
    }
}

#[derive(Debug, Clone, Copy, serde::Serialize)]
pub struct SingleObjective(f64);

impl PartialEq for SingleObjective {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl Eq for SingleObjective {}

impl PartialOrd for SingleObjective {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}
impl Ord for SingleObjective {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Default for SingleObjective {
    fn default() -> Self {
        Self(f64::INFINITY)
    }
}

impl Objective for SingleObjective {}

impl SingleObjective {
    #[allow(dead_code)]
    fn is_finite(&self) -> bool {
        self.0.is_finite()
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}

impl From<SingleObjective> for f64 {
    fn from(objective: SingleObjective) -> Self {
        objective.value()
    }
}

impl TryFrom<f64> for SingleObjective {
    type Error = IllegalObjective;

    /// Tries to convert a float into a `SingleObjective` value.
    ///
    /// See [IllegalObjective] for a list of illegal values.
    /// All other values will return `Ok`.
    fn try_from(value: f64) -> Result<Self, IllegalObjective> {
        match value {
            _ if value.is_nan() => Err(IllegalObjective::NaN),
            _ if value.is_infinite() && value.is_sign_negative() => {
                Err(IllegalObjective::NegativeInfinity)
            }
            _ => Ok(SingleObjective(value)),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct MultiObjective(Vec<f64>);

impl PartialEq for MultiObjective {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl Eq for MultiObjective {}

/// Implements Pareto-Domination
impl PartialOrd for MultiObjective {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // Use Eq checking for equality
        if self == other {
            return Some(std::cmp::Ordering::Equal);
        }

        let dominates = self
            .value()
            .iter()
            .zip(other.value().iter())
            .filter(|(own, other)| own >= other)
            .count();

        match dominates {
            // Self dominates
            x if x == self.value().len() => Some(std::cmp::Ordering::Greater),
            // Other dominates
            0 => Some(std::cmp::Ordering::Less),
            // None dominates
            _ => None,
        }
    }
}

impl Objective for MultiObjective {}

impl MultiObjective {
    #[allow(dead_code)]
    fn is_finite(&self) -> bool {
        self.0.iter().all(|o| o.is_finite())
    }

    pub fn value(&self) -> &[f64] {
        &self.0
    }
}

impl From<MultiObjective> for Vec<f64> {
    fn from(objective: MultiObjective) -> Self {
        objective.0
    }
}