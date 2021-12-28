//! Collection of common test problems.

use crate::framework::Fitness;
use std::{any::Any, ops::Range};

pub mod bmf;
pub mod coco;
pub mod coco_bound;
pub mod tsp;

/// Represents the (global) optimum of the search space.
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct Optimum<S> {
    pub fitness: Fitness,
    pub solution: Option<S>,
}

pub trait Problem {
    type Encoding: Any + Clone;

    fn evaluate(&self, solution: &Self::Encoding) -> f64;

    fn name(&self) -> &str;
}

pub trait VectorProblem {
    type T: Any + Clone;

    fn dimension(&self) -> usize;
}

pub trait LimitedVectorProblem: VectorProblem {
    fn range(&self, dimension: usize) -> Range<Self::T>;
}
