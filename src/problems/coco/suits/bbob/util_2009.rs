//! BBOB legacy code from 2009
//!
//! This reuses the C legacy code to guarantee the same behaviour
//! as the official coco benchmark suite.

extern "C" {
    fn bbob2009_compute_xopt(xopt: *mut f64, seed: i64, dim: isize);
    fn bbob2009_compute_fopt(function: isize, instance: isize) -> f64;
}

pub fn compute_xopt(seed: usize, dimension: usize) -> Vec<f64> {
    let mut xopt = vec![0.0; dimension];

    unsafe {
        bbob2009_compute_xopt(xopt.as_mut_ptr(), seed as i64, dimension as isize);
    }

    xopt
}

pub fn compute_fopt(function: usize, instance: usize) -> f64 {
    unsafe { bbob2009_compute_fopt(function as isize, instance as isize) }
}

#[cfg(test)]
mod tests {
    use float_eq::assert_float_eq;

    #[test]
    fn compute_xopt() {
        let rseed = 4 + 10000 * 2;
        let x = super::compute_xopt(rseed, 5);
        let xe = vec![-3.123200, -1.584800, -3.537600, 1.694400, 3.956000];
        assert_float_eq!(&x, &xe, abs_all <= 0.0001);
    }

    #[test]
    fn compute_fopt() {
        let y = super::compute_fopt(4, 2);
        assert_float_eq!(y, 77.66, abs <= 0.0);
    }
}
