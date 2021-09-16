use std::f64::consts::PI;

/// Sphere
pub fn sphere(x: &[f64]) -> f64 {
    x.iter().map(|xi| xi * xi).sum()
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

pub fn rastrigin(x: &[f64]) -> f64 {
    let num = x.len() as f64;
    let sum1 = x.iter().map(|&xi| (2.0 * PI * xi).cos()).sum::<f64>();
    let sum2 = x.iter().map(|&xi| xi * xi).sum::<f64>();

    10.0 * (num - sum1) + sum2
}
