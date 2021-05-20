//! This module allows easy access to instances of the traveling salesman problem taken from tsplib.

use crate::fitness::Fitness;

/// Symmetric TSP
pub mod symmetric;
pub use symmetric::Instances;
pub use symmetric::SymmetricTsp;

/// Asymmetric TSP
pub mod asymmetric {}

type Coordinates = Vec<f64>;
type DistanceMeasure = fn(&[f64], &[f64]) -> Fitness;
type Dimension = usize;

pub type Edge = (usize, usize);
pub type Node = usize;
pub type Route = Vec<Node>;

/// Popular distance functions used in TSP.
mod distances {
    use crate::fitness::Fitness;
    use std::convert::{TryFrom, TryInto};

    pub fn euclidean_distance(a: &[f64], b: &[f64]) -> Fitness {
        assert_eq!(a.len(), b.len());
        a.iter()
            .zip(b.iter())
            .map(|(x, y)| (x - y).powi(2))
            .sum::<f64>()
            .sqrt()
            .try_into()
            .unwrap()
    }

    pub fn manhattan_distance(a: &[f64], b: &[f64]) -> Fitness {
        assert_eq!(a.len(), b.len());
        a.iter()
            .zip(b.iter())
            .map(|(x, y)| (x - y).abs())
            .sum::<f64>()
            .try_into()
            .unwrap()
    }

    pub fn maximum_distance(a: &[f64], b: &[f64]) -> Fitness {
        assert_eq!(a.len(), b.len());
        a.iter()
            .zip(b.iter())
            .map(|(x, y)| (x - y).abs())
            .map(|d| Fitness::try_from(d).unwrap())
            .max()
            .unwrap()
    }
}
