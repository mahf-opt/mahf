//! This module allows easy access to instances of the traveling salesman problem taken from tsplib.

/// Symmetric TSP
pub mod symmetric;
pub use symmetric::{Instances, SymmetricTsp};

/// Asymmetric TSP
pub mod asymmetric {}

type Coordinates = Vec<f64>;
type DistanceMeasure = fn(&[f64], &[f64]) -> f64;
type Dimension = usize;

pub type Edge = (usize, usize);
pub type Node = usize;
pub type Route = Vec<Node>;
