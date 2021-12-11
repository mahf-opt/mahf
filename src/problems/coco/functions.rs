use std::{f64::consts::PI, ops::Range};

pub const DEFAULT_DOMAIN: Range<f64> = -5.0..5.0;
const DEFAULT_OPTIMUM: f64 = 0.0;

pub struct FunctionObject {
    pub evaluate: fn(x: &[f64]) -> f64,
    pub best_parameter: fn(dim: usize) -> Vec<f64>,
    pub domain: fn(dim: usize) -> Vec<Range<f64>>,
    pub best_value: fn(dim: usize) -> f64,
}
impl FunctionObject {
    pub fn new<F: Function>() -> Self {
        FunctionObject {
            evaluate: F::evaluate,
            best_parameter: F::best_parameter,
            domain: F::domain,
            best_value: F::best_value,
        }
    }
}
impl<F: Function> From<F> for FunctionObject {
    fn from(_: F) -> Self {
        FunctionObject::new::<F>()
    }
}

pub trait Function {
    fn evaluate(x: &[f64]) -> f64;

    fn best_parameter(dim: usize) -> Vec<f64> {
        vec![DEFAULT_OPTIMUM; dim]
    }

    fn domain(dim: usize) -> Vec<Range<f64>> {
        vec![DEFAULT_DOMAIN; dim]
    }

    fn best_value(dim: usize) -> f64 {
        Self::evaluate(&Self::best_parameter(dim))
    }
}

/// Sphere
pub struct Sphere;
impl Function for Sphere {
    fn evaluate(x: &[f64]) -> f64 {
        x.iter().map(|xi| xi * xi).sum()
    }
}

pub struct Ellipsoid;
impl Function for Ellipsoid {
    fn evaluate(x: &[f64]) -> f64 {
        let condition = 1.0e6f64;
        let n = x.len() as f64;

        x.iter()
            .enumerate()
            .map(|(i, xi)| (i as f64, *xi))
            .map(|(i, xi)| condition.powf(i / (n - 1.0)) * xi * xi)
            .sum()
    }
}

/// Rastrigin
pub struct Rastrigin;
impl Function for Rastrigin {
    fn evaluate(x: &[f64]) -> f64 {
        let num = x.len() as f64;
        let sum1 = x.iter().map(|&xi| (2.0 * PI * xi).cos()).sum::<f64>();
        let sum2 = x.iter().map(|&xi| xi * xi).sum::<f64>();

        10.0 * (num - sum1) + sum2
    }
}

/// Büche Rastrigin
///
/// This is the same as [rastrigin]
pub use Rastrigin as BuecheRastrigin;

/// Linear Slope
///
/// Important: This deviates from coco!
// Unlike coco's implementation the optimum is always at 5^n.
pub struct LinearSlope;
impl Function for LinearSlope {
    fn evaluate(x: &[f64]) -> f64 {
        let n = x.len() as f64;

        x.iter()
            .enumerate()
            .map(|(i, xi)| (i as f64, *xi))
            .map(|(i, xi)| (10.0f64.powf(i / (n - 1.0)), xi))
            .map(|(si, xi)| 5.0 * (si.abs() - f64::min(5.0, xi)))
            .sum::<f64>()
    }
}
#[cfg(test)]
#[test]
fn linear_slope_optimum_check() {
    let input = &[5.0; 10];
    let output = LinearSlope::evaluate(input);
    float_eq::assert_float_eq!(output, 0.0, abs <= 0.0);
}

pub struct Rosenbrock;
impl Function for Rosenbrock {
    fn evaluate(x: &[f64]) -> f64 {
        let sum1 = x
            .iter()
            .zip(x.iter().skip(1))
            .map(|(xi, xj)| xi * xi - xj)
            .map(|xi| xi * xi)
            .sum::<f64>();
        let sum2 = x.iter().map(|xi| xi - 1.0).map(|xi| xi * xi).sum::<f64>();

        100.0 * sum1 + sum2
    }
}

/// Attractive Sector
///
/// Important: This deviates from coco!
// This is what it is supposed to be:
// ```c
// static double f_attractive_sector_raw(const double *x,
//                                       const size_t number_of_variables,
//                                       f_attractive_sector_data_t *data) {
//   size_t i;
//   double result;
//
//   if (coco_vector_contains_nan(x, number_of_variables))
//   	return NAN;
//
//   result = 0.0;
//   for (i = 0; i < number_of_variables; ++i) {
//     if (data->xopt[i] * x[i] > 0.0) {
//       result += 100.0 * 100.0 * x[i] * x[i];
//     } else {
//       result += x[i] * x[i];
//     }
//   }
//   return result;
// }
// ```
pub struct AttractiveSector;
impl Function for AttractiveSector {
    fn evaluate(x: &[f64]) -> f64 {
        fn factor(xi: f64) -> f64 {
            if xi < 0.0 {
                10000.0
            } else {
                1.0
            }
        }

        x.iter().map(|&xi| factor(xi) * xi * xi).sum()
    }
}
