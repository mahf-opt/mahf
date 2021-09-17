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
            out[i] = xi + self.translation[i];
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
