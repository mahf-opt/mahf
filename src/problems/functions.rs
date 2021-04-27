//! Collection of test functions

use crate::problem::{LimitedVectorProblem, Problem, VectorProblem};

pub struct BenchmarkFunction {
    name: &'static str,
    implementation: Function,
    dimension: usize,
}

impl BenchmarkFunction {
    pub fn sphere(dimension: usize) -> Self {
        BenchmarkFunction {
            name: "sphere",
            implementation: scaled_implementations::sphere,
            dimension,
        }
    }

    pub fn rastrigin(dimension: usize) -> Self {
        BenchmarkFunction {
            name: "rstrigin",
            implementation: scaled_implementations::rastrigin,
            dimension,
        }
    }

    pub fn ackley(dimension: usize) -> Self {
        BenchmarkFunction {
            name: "ackley",
            implementation: scaled_implementations::ackley,
            dimension,
        }
    }
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

/// A benchmark function
pub type Function = fn(&[f64]) -> f64;

/// The same functions scaled tp [-1.0, 1.0]
pub mod scaled_implementations {
    use std::f64::consts::PI;

    /// Sphere function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: 0 at (0,...,0)
    pub fn sphere(x: &[f64]) -> f64 {
        x.iter().map(|xi| xi * 5.12).map(|xi| xi * xi).sum()
    }

    /// Rastrinin function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: 0 at (0,...,0)
    pub fn rastrigin(x: &[f64]) -> f64 {
        let n = x.len() as f64;
        10.0 * n
            + x.iter()
                .map(|xi| xi * 5.12)
                .map(|xi| xi * xi - 10.0 * (2.0 * PI * xi).cos())
                .sum::<f64>()
    }

    /// Ackley function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: 0 at (0,...,0)
    pub fn ackley(x: &[f64]) -> f64 {
        let a = 20.;
        let b = 0.2;
        let c = 2.0 * PI;

        let n_inverse = 1.0 / x.len() as f64;
        let squared_sum = x
            .iter()
            .map(|xi| xi * 32.0)
            .map(|xi| xi.powi(2))
            .sum::<f64>();
        let cosine_sum = x
            .iter()
            .map(|xi| xi * 32.0)
            .map(|xi| (c * xi).cos())
            .sum::<f64>();

        a + (1.0f64).exp() + (-a) * ((-b) * (n_inverse * squared_sum).sqrt()).exp()
            - (n_inverse * cosine_sum).exp()
    }
}
