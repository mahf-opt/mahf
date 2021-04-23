//! Collection of test functions

use std::f64::consts::PI;
use std::ops::RangeInclusive;

/// A benchmark function
pub type Function = fn(&[f64]) -> f64;

/// A [`Function`] with specified dimension
pub struct Problem {
    pub function: Function,
    pub dimension: u32,
    pub range: RangeInclusive<f64>,
}

/// Sphere function
///
/// Typical evaluation range: [-5.12, 5.12]
///
/// Optimum: 0 at (0,...,0)
pub fn sphere(x: &[f64]) -> f64 {
    x.iter().map(|xi| xi * xi).sum()
}

/// Rastrinin function
///
/// Typical evaluation range: [-5.12, 5.12]
///
/// Optimum: 0 at (0,...,0)
pub fn rastrigin(x: &[f64]) -> f64 {
    let n = x.len() as f64;
    10.0 * n
        + x.iter()
            .map(|xi| xi * xi - 10.0 * (2.0 * PI * xi).cos())
            .sum::<f64>()
}

/// Ackley function
///
/// Typical evaluation range: [-32, 32]
///
/// Optimum: 0 at (0,...,0)
pub fn ackley(x: &[f64]) -> f64 {
    let a = 20.;
    let b = 0.2;
    let c = 2. * PI;

    let n_inverse = 1. / x.len() as f64;
    let squared_sum = x.iter().map(|xi| xi.powi(2)).sum::<f64>();
    let cosine_sum = x.iter().map(|xi| (c * xi).cos()).sum::<f64>();

    a + (1.0f64).exp() + (-a) * ((-b) * (n_inverse * squared_sum).sqrt()).exp()
        - (n_inverse * cosine_sum).exp()
}

/// The same functions scaled tp [-1.0, 1.0]
pub mod scaled {
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
