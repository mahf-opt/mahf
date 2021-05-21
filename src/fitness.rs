//! Utility type to store an individuals fitness.

use std::{convert::TryFrom, fmt};

/// Fitness value of an [Individual](crate::heuristic::Individual)
///
/// [Fitness::try_from] can be used to construct a `Fitness` value.
#[derive(Debug, Clone, Copy)]
pub struct Fitness(f64);

impl Fitness {
    /// Returns the actual floating point value.
    pub fn into(self) -> f64 {
        self.0
    }

    /// Returns whether the fitness is finite
    pub fn is_finite(&self) -> bool {
        self.0.is_finite()
    }
}

impl Default for Fitness {
    /// Returns Inf fitness
    fn default() -> Self {
        Fitness(f64::INFINITY)
    }
}

impl TryFrom<f64> for Fitness {
    type Error = IllegalFitness;

    /// Tries to convert a float into a `Fitness` value.
    ///
    /// See [IllegalFitness] for a list of illegal values.
    /// All other values will return `Ok`.
    fn try_from(value: f64) -> Result<Self, IllegalFitness> {
        match value {
            _ if value.is_nan() => Err(IllegalFitness::NaN),
            _ if value.is_infinite() && value.is_sign_negative() => {
                Err(IllegalFitness::NegativeInfinity)
            }
            _ => Ok(Fitness(value)),
        }
    }
}

impl From<Fitness> for f64 {
    fn from(fitness: Fitness) -> Self {
        fitness.0
    }
}

impl PartialEq for Fitness {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl Eq for Fitness {}

impl PartialOrd for Fitness {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}
impl Ord for Fitness {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

/// Error type for illegal fitness values.
///
/// Currently, `NaN` and `-Inf` are considered illegal.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IllegalFitness {
    NaN,
    NegativeInfinity,
}

impl std::fmt::Display for IllegalFitness {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            fmt,
            "illegal fitness: {}",
            match self {
                IllegalFitness::NaN => "NaN",
                IllegalFitness::NegativeInfinity => "-Inf",
            },
        )
    }
}
