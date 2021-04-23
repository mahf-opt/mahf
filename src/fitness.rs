use std::{convert::TryFrom, fmt};

#[derive(Debug, Clone, Copy)]
pub struct Fitness(f64);

impl Fitness {
    pub fn into(self) -> f64 {
        self.0
    }
}

impl TryFrom<f64> for Fitness {
    type Error = IllegalFitness;

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
