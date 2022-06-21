//! Utility type to store an individuals fitness.

use std::any::Any;
use std::fmt;

/// Fitness value of an [Individual](crate::framework::Individual)
///
/// [Fitness::try_from] can be used to construct a `Fitness` value.
#[derive(Debug, Clone, Copy, serde::Serialize)]
pub struct Fitness(f64);

impl Fitness {
    /// Returns the actual floating point value.
    pub fn into(self) -> f64 {
        self.0
    }

    /// Returns whether the fitness is finite.
    pub fn is_finite(&self) -> bool {
        self.0.is_finite()
    }
}

impl Default for Fitness {
    /// Returns worst possible (infinity) fitness.
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

impl fmt::Display for IllegalFitness {
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

pub trait Objective: fmt::Debug + Clone + Eq + Any {}

/// Error type for illegal objective values.
///
/// Currently, `NaN` and `-Inf` are considered illegal.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IllegalObjective {
    NaN,
    NegativeInfinity,
    Unevaluated,
    WrongType,
}

impl fmt::Display for IllegalObjective {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            fmt,
            "illegal objective: {}",
            match self {
                IllegalObjective::NaN => "NaN",
                IllegalObjective::NegativeInfinity => "-Inf",
                IllegalObjective::Unevaluated => "Unevaluated",
                IllegalObjective::WrongType => "Wrong objective type",
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

impl Objective for SingleObjective {}

impl SingleObjective {
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

    /// Tries to convert a float into a `Fitness` value.
    ///
    /// See [IllegalFitness] for a list of illegal values.
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
