//! Collection of common test problems.

use crate::fitness::Fitness;

pub mod bmf;
pub mod tsp;

/// Represents the (global) optimum of the search space.
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct Optimum<S> {
    pub fitness: Fitness,
    pub solution: Option<S>,
}
