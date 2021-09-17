use std::f64::consts::PI;

/// Sphere
pub fn sphere(x: &[f64]) -> f64 {
    x.iter().map(|xi| xi * xi).sum()
}

pub fn ellipsoid(x: &[f64]) -> f64 {
    let condition = 1.0e6f64;
    let n = x.len() as f64;

    x.iter()
        .enumerate()
        .map(|(i, xi)| (i as f64, *xi))
        .map(|(i, xi)| condition.powf(i / (n - 1.0)) * xi * xi)
        .sum()
}

/// Rastrigin
pub fn rastrigin(x: &[f64]) -> f64 {
    let num = x.len() as f64;
    let sum1 = x.iter().map(|&xi| (2.0 * PI * xi).cos()).sum::<f64>();
    let sum2 = x.iter().map(|&xi| xi * xi).sum::<f64>();

    10.0 * (num - sum1) + sum2
}

/// Büche Rastrigin
///
/// This is the same as [rastrigin]
pub fn bueche_rastrigin(x: &[f64]) -> f64 {
    rastrigin(x)
}

pub fn linear_slope(x: &[f64]) -> f64 {
    let n = x.len() as f64;

    x.iter()
        .enumerate()
        .map(|(i, xi)| (i as f64, *xi))
        .map(|(i, xi)| (10.0f64.powf(i / (n - 1.0)), xi))
        .map(|(si, xi)| 5.0 * (si.abs() - f64::min(5.0, xi)))
        .sum::<f64>()
}

pub fn rosenbrock(x: &[f64]) -> f64 {
    let sum1 = x
        .iter()
        .zip(x.iter().skip(1))
        .map(|(xi, xj)| xi * xi - xj)
        .map(|xi| xi * xi)
        .sum::<f64>();
    let sum2 = x.iter().map(|xi| xi - 1.0).map(|xi| xi * xi).sum::<f64>();

    100.0 * sum1 + sum2
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
pub fn attractive_sector(x: &[f64]) -> f64 {
    fn factor(xi: f64) -> f64 {
        if xi < 0.0 {
            10000.0
        } else {
            1.0
        }
    }

    x.iter().map(|&xi| factor(xi) * xi * xi).sum()
}
