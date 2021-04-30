//! Traits to describe problems.

use std::{any::Any, ops::Range};

pub trait Problem {
    type Encoding: Any;

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
