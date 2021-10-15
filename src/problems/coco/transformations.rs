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

pub struct TranslateInput {
    pub translation: Vec<f64>,
}
impl TranslateInput {
    pub fn new(translation: Vec<f64>) -> Self {
        TranslateInput { translation }
    }
}
impl Transformation for TranslateInput {
    fn transform_input(&self, x: &[f64], out: &mut [f64]) {
        for (i, xi) in x.iter().enumerate() {
            out[i] = xi - self.translation[i];
        }
    }
}

pub struct OscillateInput;
impl Transformation for OscillateInput {
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

pub struct ConditionInput {
    pub alpha: f64,
}
impl Transformation for ConditionInput {
    fn transform_input(&self, x: &[f64], out: &mut [f64]) {
        for i in 0..x.len() {
            let scale = (self.alpha * i as f64) / (x.len() as f64 - 1.0);
            out[i] = f64::powf(self.alpha, 0.5 * scale) * x[i];
        }
    }
}

pub struct AsymmetricInput {
    pub beta: f64,
}
impl Transformation for AsymmetricInput {
    fn transform_input(&self, x: &[f64], out: &mut [f64]) {
        for i in 0..x.len() {
            if x[i] > 0.0 {
                let scale = (self.beta * i as f64) / (x.len() as f64 - 1.0);
                let exponent = 1.0 + scale * f64::sqrt(x[i]);
                out[i] = f64::powf(x[i], exponent);
            } else {
                out[i] = x[i];
            }
        }
    }
}

pub struct TranslateOutput {
    pub translation: f64,
}
impl TranslateOutput {
    pub fn new(translation: f64) -> Self {
        TranslateOutput { translation }
    }
}
impl Transformation for TranslateOutput {
    fn transform_output(&self, y: f64) -> f64 {
        y + self.translation
    }
}
