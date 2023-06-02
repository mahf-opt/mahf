use better_any::{Tid, TidAble};
use serde::Serialize;

use crate::state::CustomState;

#[derive(Clone, Serialize, Tid)]
pub struct PheromoneMatrix {
    dimension: usize,
    inner: Vec<f64>,
}
impl PheromoneMatrix {
    pub fn new(dimension: usize, initial_value: f64) -> Self {
        PheromoneMatrix {
            dimension,
            inner: vec![initial_value; dimension * dimension],
        }
    }
}
impl std::ops::Index<usize> for PheromoneMatrix {
    type Output = [f64];

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.dimension);
        let start = index * self.dimension;
        let end = start + self.dimension;
        &self.inner[start..end]
    }
}
impl std::ops::IndexMut<usize> for PheromoneMatrix {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < self.dimension);
        let start = index * self.dimension;
        let end = start + self.dimension;
        &mut self.inner[start..end]
    }
}
impl std::ops::MulAssign<f64> for PheromoneMatrix {
    fn mul_assign(&mut self, rhs: f64) {
        for x in &mut self.inner {
            *x *= rhs;
        }
    }
}
impl CustomState<'_> for PheromoneMatrix {}
