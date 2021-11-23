#![allow(clippy::new_ret_no_self, unused_variables)]

pub mod input {
    use crate::problems::coco::InputTransformation;

    pub struct Permutation {
        pub mapping: Vec<usize>,
    }
    impl Permutation {
        pub fn new(mapping: Vec<usize>) -> Box<dyn InputTransformation> {
            Box::new(Permutation { mapping })
        }
    }
    impl InputTransformation for Permutation {
        fn apply(&self, x: &[f64], out: &mut [f64]) {
            debug_assert_eq!(x.len(), self.mapping.len());
            for (o, m) in out.iter_mut().zip(self.mapping.iter()) {
                *o = x[*m];
            }
        }

        fn reverse(&self, x: &[f64], out: &mut [f64]) {
            debug_assert_eq!(x.len(), self.mapping.len());
            for (o, m) in x.iter().zip(self.mapping.iter()) {
                out[*m] = *o;
            }
        }
    }

    pub struct Translate {
        pub translation: Vec<f64>,
    }
    impl Translate {
        pub fn new(translation: Vec<f64>) -> Box<dyn InputTransformation> {
            Box::new(Translate { translation })
        }
    }
    impl InputTransformation for Translate {
        fn apply(&self, x: &[f64], out: &mut [f64]) {
            for (i, xi) in x.iter().enumerate() {
                out[i] = xi - self.translation[i];
            }
        }

        fn reverse(&self, x: &[f64], out: &mut [f64]) {
            for (i, xi) in x.iter().enumerate() {
                out[i] = xi + self.translation[i];
            }
        }
    }

    pub struct Oscillate;
    impl Oscillate {
        pub fn new() -> Box<dyn InputTransformation> {
            Box::new(Oscillate)
        }
    }
    impl InputTransformation for Oscillate {
        fn apply(&self, x: &[f64], oscillated_x: &mut [f64]) {
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

        fn reverse(&self, x: &[f64], out: &mut [f64]) {
            todo!()
        }
    }

    pub struct Condition {
        pub alpha: f64,
    }
    impl Condition {
        pub fn new(alpha: f64) -> Box<dyn InputTransformation> {
            Box::new(Condition { alpha })
        }
    }
    impl InputTransformation for Condition {
        fn apply(&self, x: &[f64], out: &mut [f64]) {
            for i in 0..x.len() {
                let scale = (i as f64) / (x.len() as f64 - 1.0);
                out[i] = f64::powf(self.alpha, 0.5 * self.alpha * scale) * x[i];
            }
        }

        fn reverse(&self, x: &[f64], out: &mut [f64]) {
            todo!()
        }
    }

    pub struct Asymmetric {
        pub beta: f64,
    }
    impl Asymmetric {
        pub fn new(beta: f64) -> Box<dyn InputTransformation> {
            Box::new(Asymmetric { beta })
        }
    }
    impl InputTransformation for Asymmetric {
        fn apply(&self, x: &[f64], out: &mut [f64]) {
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

        fn reverse(&self, x: &[f64], out: &mut [f64]) {
            todo!()
        }
    }

    /// Implementation of the ominous 's_i scaling' of the BBOB Bueche-Rastrigin problem.
    pub struct Brs;
    impl Brs {
        pub fn new() -> Box<dyn InputTransformation> {
            Box::new(Brs)
        }
    }
    impl InputTransformation for Brs {
        fn apply(&self, x: &[f64], out: &mut [f64]) {
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

        fn reverse(&self, x: &[f64], out: &mut [f64]) {
            todo!()
        }
    }
}

pub mod output {
    use crate::problems::coco::OutputTransformation;

    pub struct Translate {
        pub translation: f64,
    }
    impl Translate {
        pub fn new(translation: f64) -> Box<dyn OutputTransformation> {
            Box::new(Translate { translation })
        }
    }
    impl OutputTransformation for Translate {
        fn apply(&self, y: f64) -> f64 {
            y - self.translation
        }

        fn reverse(&self, y: f64) -> f64 {
            y + self.translation
        }
    }
}
