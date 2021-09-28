//! Numerical Black-Box Optimization Benchmarking Framework
//!
//! Implementation is base on <https://github.com/numbbo/coco>

use std::{any::Any, ops::Range};

pub mod functions;
pub mod problems;
pub mod suits;
pub mod transformations;

pub type Function = fn(x: &[f64]) -> f64;

pub trait Transformation {
    fn transform_input(&self, x: &[f64], out: &mut [f64]) {
        out.clone_from_slice(x);
    }

    fn transform_output(&self, y: f64) -> f64 {
        y
    }
}

pub struct Problem {
    /// Name of the function or transformation
    pub name: &'static str,
    /// Subproblem or function
    pub function: ProblemFunction,
    /// Inclusive min and max values
    pub domain: Range<f64>,
}
impl Problem {
    pub fn evaluate(&self, x: &mut [f64], buffer: &mut [f64]) -> f64 {
        debug_assert_eq!(x.len(), buffer.len());
        match &self.function {
            ProblemFunction::Transformation(t, p) => {
                t.transform_input(x, buffer);
                let y = p.evaluate(buffer, x);
                t.transform_output(y)
            }
            ProblemFunction::Function(f) => f(x),
        }
    }

    pub fn extend(
        name: &'static str,
        subproblem: Problem,
        transform: impl Transformation + Any,
    ) -> Problem {
        Problem {
            name,
            domain: subproblem.domain.clone(),
            function: ProblemFunction::Transformation(Box::new(transform), Box::new(subproblem)),
        }
    }
}

pub struct Instance {
    problem: Problem,
    dimension: usize,
}
impl Instance {
    pub fn new(problem: Problem, dimension: usize) -> Self {
        Instance { problem, dimension }
    }
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
impl crate::problem::LimitedVectorProblem for Instance {
    fn range(&self, _dimension: usize) -> std::ops::Range<Self::T> {
        self.problem.domain.clone()
    }
}

pub enum ProblemFunction {
    Transformation(Box<dyn Transformation>, Box<Problem>),
    Function(Function),
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

    #[test]
    fn translate_sphere() {
        let problem = problems::sphere();
        let problem = problems::translate_input(vec![-1.0, -1.0, -1.0], problem);
        let problem = problems::translate_output(5.0, problem);
        let out = problem.evaluate(&mut [1.0, 1.0, 1.0], &mut [0.0, 0.0, 0.0]);
        assert_float_eq!(out, 5.0, abs <= 0.0);
    }
}
