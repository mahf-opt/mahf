pub mod input {
    use crate::problems::coco::Transformation;

    pub struct Permutation {
        pub mapping: Vec<usize>,
    }
    impl Permutation {
        pub fn new(mapping: Vec<usize>) -> Self {
            Permutation { mapping }
        }
    }
    impl Transformation for Permutation {
        fn transform_input(&self, x: &[f64], out: &mut [f64]) {
            debug_assert_eq!(x.len(), self.mapping.len());
            for (o, m) in out.iter_mut().zip(self.mapping.iter()) {
                *o = x[*m];
            }
        }
    }

    pub struct Translate {
        pub translation: Vec<f64>,
    }
    impl Translate {
        pub fn new(translation: Vec<f64>) -> Self {
            Translate { translation }
        }
    }
    impl Transformation for Translate {
        fn transform_input(&self, x: &[f64], out: &mut [f64]) {
            for (i, xi) in x.iter().enumerate() {
                out[i] = xi - self.translation[i];
            }
        }
    }

    pub struct Oscillate;
    impl Transformation for Oscillate {
        fn transform_input(&self, x: &[f64], oscillated_x: &mut [f64]) {
            let alpha = 0.1;

            for i in 0..x.len() {
                if x[i] > 0.0 {
                    let tmp = f64::ln(x[i]) / alpha;
                    let base = f64::exp(tmp + 0.49 * (f64::sin(tmp) + f64::sin(0.79 * tmp)));
                    oscillated_x[i] = f64::powf(base, alpha);
                } else if x[i] < 0.0 {
                    let tmp = f64::ln(-x[i]) / alpha;
                    let base = f64::exp(tmp + 0.49 * (f64::sin(0.55 * tmp) + f64::sin(0.31 * tmp)));
                    oscillated_x[i] = -f64::powf(base, alpha);
                } else {
                    oscillated_x[i] = 0.0;
                }
            }
        }
    }

    pub struct Condition {
        pub alpha: f64,
    }
    impl Transformation for Condition {
        fn transform_input(&self, x: &[f64], out: &mut [f64]) {
            for i in 0..x.len() {
                let scale = (i as f64) / (x.len() as f64 - 1.0);
                out[i] = f64::powf(self.alpha, 0.5 * self.alpha * scale) * x[i];
            }
        }
    }

    pub struct Asymmetric {
        pub beta: f64,
    }
    impl Transformation for Asymmetric {
        fn transform_input(&self, x: &[f64], out: &mut [f64]) {
            for i in 0..x.len() {
                if x[i] > 0.0 {
                    let scale = (i as f64) / (x.len() as f64 - 1.0);
                    let exponent = 1.0 + self.beta * scale * f64::sqrt(x[i]);
                    out[i] = f64::powf(x[i], exponent);
                } else {
                    out[i] = x[i];
                }
            }
        }
    }

    /// Implementation of the ominous 's_i scaling' of the BBOB Bueche-Rastrigin problem.
    pub struct Brs;
    impl Transformation for Brs {
        fn transform_input(&self, x: &[f64], out: &mut [f64]) {
            for i in 0..x.len() {
                let scale = (i as f64) / (x.len() as f64 - 1.0);
                /* Function documentation says we should compute 10^(0.5 *
                 * (i-1)/(D-1)). Instead we compute the equivalent
                 * sqrt(10)^((i-1)/(D-1)) just like the legacy code.
                 */
                let mut factor = f64::powf(f64::sqrt(10.0), scale);
                /* Documentation specifies odd indices and starts indexing
                 * from 1, we use all even indices since C starts indexing
                 * with 0.
                 */
                if x[i] > 0.0 && i % 2 == 0 {
                    factor *= 10.0;
                }
                out[i] = factor * x[i];
            }
        }
    }
}

pub mod output {
    use crate::problems::coco::Transformation;

    pub struct Translate {
        pub translation: f64,
    }
    impl Translate {
        pub fn new(translation: f64) -> Self {
            Translate { translation }
        }
    }
    impl Transformation for Translate {
        fn transform_output(&self, y: f64) -> f64 {
            y + self.translation
        }
    }
}
