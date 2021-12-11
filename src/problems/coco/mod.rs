//! Numerical Black-Box Optimization Benchmarking Framework
//!
//! Implementation is base on <https://github.com/numbbo/coco>

use std::{mem, ops::Range};

pub mod functions;
pub mod suits;
pub mod transformations;

use functions::FunctionObject;

pub trait InputTransformation: Send + Sync {
    /// Transform the function input.
    fn apply(&self, x: &[f64], out: &mut [f64]);

    /// Reverse the function input transformation.
    ///
    /// NOTE: This is not supported by Coco!
    fn reverse(&self, x: &[f64], out: &mut [f64]) {
        let _ = (x, out);
        unimplemented!("Coco officially does not support this.")
    }
}

pub trait OutputTransformation: Send + Sync {
    /// Transform the function output.
    fn apply(&self, x: f64) -> f64;

    /// Reverse the function output transformation.
    fn reverse(&self, x: f64) -> f64;
}

pub struct Problem {
    /// Transformations applied before evaluation
    pub input_transformations: Vec<Box<dyn InputTransformation>>,
    /// The base function
    pub function: FunctionObject,
    /// Transformations applied after evaluation
    pub output_transformations: Vec<Box<dyn OutputTransformation>>,
    /// Inclusive min and max values
    pub domain: Range<f64>,
}
impl Problem {
    pub fn evaluate<'a>(&self, mut x: &'a mut [f64], mut buffer: &'a mut [f64]) -> f64 {
        debug_assert_eq!(x.len(), buffer.len());

        for transformation in &self.input_transformations {
            transformation.apply(x, buffer);
            mem::swap(&mut x, &mut buffer);
        }

        let mut y = (self.function.evaluate)(buffer);

        for transformation in &self.output_transformations {
            y = transformation.apply(y);
        }

        y
    }
}

#[derive(serde::Serialize)]
pub struct CocoInstance {
    #[serde(skip)]
    problem: Problem,
    suite: &'static str,
    function: usize,
    instance: usize,
    dimension: usize,
}
impl CocoInstance {
    pub fn format_name(&self) -> String {
        format!("f{}_d{}_i{}", self.function, self.dimension, self.instance)
    }
}
impl crate::problems::Problem for CocoInstance {
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
impl crate::problems::VectorProblem for CocoInstance {
    type T = f64;

    fn dimension(&self) -> usize {
        self.dimension
    }
}
impl crate::problems::LimitedVectorProblem for CocoInstance {
    fn range(&self, _dimension: usize) -> std::ops::Range<Self::T> {
        self.problem.domain.clone()
    }
}

#[cfg(test)]
mod tests {
    use float_eq::assert_float_eq;

    use crate::problems::coco::{
        functions,
        transformations::{input, output},
        Problem,
    };

    #[test]
    fn create_permutated_sphere() {
        let problem = Problem {
            input_transformations: vec![input::Permutation::new(vec![2, 1, 0])],
            function: functions::Sphere.into(),
            output_transformations: vec![],
            domain: functions::DEFAULT_DOMAIN,
        };
        let out = problem.evaluate(&mut [1.0, 2.0, 3.0], &mut [0.0, 0.0, 0.0]);
        assert_float_eq!(out, 1.0 + 4.0 + 9.0, abs <= 0.0);
    }

    #[test]
    fn translate_sphere() {
        let problem = Problem {
            input_transformations: vec![input::Translate::new(vec![1.0, 1.0, 1.0])],
            function: functions::Sphere.into(),
            output_transformations: vec![output::Translate::new(5.0)],
            domain: functions::DEFAULT_DOMAIN,
        };
        let out = problem.evaluate(&mut [1.0, 1.0, 1.0], &mut [0.0, 0.0, 0.0]);
        assert_float_eq!(out, 5.0, abs <= 0.0);
    }
}
