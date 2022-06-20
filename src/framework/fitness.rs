//! Utility type to store an individuals fitness.

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

pub trait Objective: fmt::Debug + Clone + Eq {
    fn is_finite(&self) -> bool;
    fn is_valid(&self) -> bool;

    fn is_single(&self) -> bool;
    fn is_multi(&self) -> bool;

    fn try_single(self) -> Result<Cost, IllegalObjective>;
    fn try_multi(self) -> Result<Objectives, IllegalObjective>;

    fn single(self) -> Cost {
        self.try_single().unwrap()
    }
    fn multi(self) -> Objectives {
        self.try_multi().unwrap()
    }
}

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
pub struct Cost(Option<f64>);

impl PartialEq for Cost {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl Eq for Cost {}

impl PartialOrd for Cost {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}
impl Ord for Cost {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Objective for Cost {
    fn is_finite(&self) -> bool {
        self.is_valid() && self.0.unwrap().is_finite()
    }

    fn is_valid(&self) -> bool {
        self.0.is_some()
    }

    fn is_single(&self) -> bool {
        true
    }

    fn is_multi(&self) -> bool {
        false
    }

    fn try_single(self) -> Result<Cost, IllegalObjective> {
        Ok(self)
    }

    fn try_multi(self) -> Result<Objectives, IllegalObjective> {
        Err(IllegalObjective::WrongType)
    }
}

impl Cost {
    pub fn option(self) -> Option<f64> {
        self.0
    }

    pub fn value(self) -> f64 {
        self.0.unwrap()
    }
}

impl TryFrom<Cost> for f64 {
    type Error = IllegalObjective;

    fn try_from(value: Cost) -> Result<Self, Self::Error> {
        value.option().ok_or(IllegalObjective::Unevaluated)
    }
}

impl TryFrom<f64> for Cost {
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
            _ => Ok(Cost(Some(value))),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct Objectives(Option<Vec<f64>>);

impl PartialEq for Objectives {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl Eq for Objectives {}

impl Objective for Objectives {
    fn is_finite(&self) -> bool {
        self.is_valid() && self.0.as_ref().unwrap().iter().all(|o| o.is_finite())
    }

    fn is_valid(&self) -> bool {
        self.0.is_some()
    }

    fn is_single(&self) -> bool {
        false
    }

    fn is_multi(&self) -> bool {
        true
    }

    fn try_single(self) -> Result<Cost, IllegalObjective> {
        Err(IllegalObjective::WrongType)
    }

    fn try_multi(self) -> Result<Objectives, IllegalObjective> {
        Ok(self)
    }
}

impl Objectives {
    pub fn option(self) -> Option<Vec<f64>> {
        self.0
    }

    pub fn value(self) -> Vec<f64> {
        self.0.unwrap()
    }
}

impl TryFrom<Objectives> for Vec<f64> {
    type Error = IllegalObjective;

    fn try_from(value: Objectives) -> Result<Self, Self::Error> {
        value.option().ok_or(IllegalObjective::Unevaluated)
    }
}
