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
