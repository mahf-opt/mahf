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
    fn transform(&self, x: &[f64], out: &mut [f64]) {
        debug_assert_eq!(x.len(), self.mapping.len());
        for (o, m) in out.iter_mut().zip(self.mapping.iter()) {
            *o = x[*m];
        }
    }
}
