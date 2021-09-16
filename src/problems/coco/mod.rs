//! Numerical Black-Box Optimization Benchmarking Framework
//!
//! Implementation is base on <https://github.com/numbbo/coco>

use std::any::Any;

pub mod functions;
pub mod problems;
pub mod transformations;

pub type Function = fn(x: &[f64]) -> f64;

pub trait Transformation {
    fn transform(&self, x: &[f64], out: &mut [f64]);
}

pub struct Problem {
    pub name: &'static str,
    pub function: ProblemFunction,
}
impl Problem {
    pub fn evaluate(&self, x: &mut [f64], buffer: &mut [f64]) -> f64 {
        debug_assert_eq!(x.len(), buffer.len());
        match &self.function {
            ProblemFunction::Transformation(t, p) => {
                t.transform(x, buffer);
                p.evaluate(buffer, x)
            }
            ProblemFunction::Function(f) => f(x),
        }
    }
}

pub struct Instance {
    problem: Problem,
    dimension: usize,
}
impl crate::problem::Problem for Instance {
    type Encoding = Vec<f64>;

    fn evaluate(&self, solution: &Self::Encoding) -> f64 {
        debug_assert_eq!(self.dimension, solution.len());
        let b1 = &mut solution.clone();
        let b2 = &mut solution.clone();
        self.problem.evaluate(b1, b2)
    }

    fn name(&self) -> &str {
        "coco"
    }
}
impl crate::problem::VectorProblem for Instance {
    type T = f64;

    fn dimension(&self) -> usize {
        self.dimension
    }
}

pub enum ProblemFunction {
    Transformation(Box<dyn Transformation>, Box<Problem>),
    Function(Function),
}
impl ProblemFunction {
    pub fn new_transformation(
        transform: impl Transformation + Any,
        problem: Problem,
    ) -> ProblemFunction {
        ProblemFunction::Transformation(Box::new(transform), Box::new(problem))
    }
}

#[cfg(test)]
mod tests {
    use float_eq::assert_float_eq;

    use super::*;

    #[test]
    fn create_permutated_sphere() {
        let problem = problems::permutation(vec![2, 1, 0], problems::sphere());
        let out = problem.evaluate(&mut [1.0, 2.0, 3.0], &mut [0.0, 0.0, 0.0]);
        assert_float_eq!(out, 1.0 + 4.0 + 9.0, abs <= 0.0);
    }
}
