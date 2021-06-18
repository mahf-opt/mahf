//! Collection of benchmark functions with corresponding tests.

pub mod implementations;
#[cfg(test)]
pub mod tests;

use crate::problem::{LimitedVectorProblem, Problem, VectorProblem};

/// Wraps the benchmark functions as [`Problem`]s.
///
/// All functions have been scaled to [-1, 1].
#[derive(Clone, Copy, serde::Serialize)]
pub struct BenchmarkFunction {
    name: &'static str,
    dimension: usize,
    #[serde(skip)]
    domain: [f64; 2],

    #[serde(skip)]
    implementation: Function,
}

impl Problem for BenchmarkFunction {
    type Encoding = Vec<f64>;

    fn evaluate(&self, solution: &Self::Encoding) -> f64 {
        (self.implementation)(solution)
    }

    fn name(&self) -> &str {
        self.name
    }
}

impl VectorProblem for BenchmarkFunction {
    type T = f64;

    fn dimension(&self) -> usize {
        self.dimension
    }
}

impl LimitedVectorProblem for BenchmarkFunction {
    fn range(&self, _dimension: usize) -> std::ops::Range<Self::T> {
        0.0..1.0
    }
}

/// A benchmark function.
pub type Function = fn(&[f64]) -> f64;
