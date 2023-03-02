//! Collection of common test problems.
//!
//! Every problem implements the [Problem] trait.
//!
//! If a given problem has certain properties, then those will be expressed as traits as well.
//! Those traits are quite specific and when writing a component they'll allow you to only require those you really need.

use better_any::Tid;

use crate::{
    framework::{Individual, MultiObjective, Objective, SingleObjective},
    state::{CustomState, State},
};
use std::{any::Any, ops::Range};

pub mod bmf;
pub mod coco_bound;
pub mod tsp;

#[cfg(never)]
pub mod coco;

/// Base trait for all problems.
///
/// Defines the problems encoding and objective.
pub trait Problem: 'static {
    /// The datatype representing the problem.
    type Encoding: Any + Clone + PartialEq + Send;

    /// The objective.
    ///
    /// See [SingleObjective] and [MultiObjective].
    type Objective: Objective;

    /// The name of the problem.
    fn name(&self) -> &str;

    fn default_evaluator(&self) -> Box<dyn Evaluator<Problem = Self>>;
}

pub trait Evaluator: Send {
    type Problem: Problem;

    fn evaluate(
        &mut self,
        problem: &Self::Problem,
        state: &mut State,
        individuals: &mut [Individual<Self::Problem>],
    );
}

#[derive(Tid)]
pub struct EvaluatorState<'a, P: 'static> {
    pub(crate) evaluator: Box<dyn Evaluator<Problem = P> + 'a>,
}
impl<'a, P: 'static> CustomState<'a> for EvaluatorState<'a, P> {}
impl<'a, P: 'static> From<Box<dyn Evaluator<Problem = P> + 'a>> for EvaluatorState<'a, P> {
    fn from(evaluator: Box<dyn Evaluator<Problem = P> + 'a>) -> Self {
        EvaluatorState { evaluator }
    }
}
impl<'a, P: 'static> EvaluatorState<'a, P> {
    pub fn new(evaluator: impl Evaluator<Problem = P> + 'a) -> Self {
        EvaluatorState {
            evaluator: Box::new(evaluator),
        }
    }
}

/// A single objective problem.
pub trait SingleObjectiveProblem: Problem<Objective = SingleObjective> {}

impl<P: Problem<Objective = SingleObjective>> SingleObjectiveProblem for P {}

/// A multi objective problem.
pub trait MultiObjectiveProblem: Problem<Objective = MultiObjective> {}

impl<P: Problem<Objective = MultiObjective>> MultiObjectiveProblem for P {}

/// A problem with fixed length array like encoding.
pub trait VectorProblem: Problem {
    /// Type of the vectors elements.
    type T: Any + Clone;

    /// Returns the dimension of the vector.
    fn dimension(&self) -> usize;
}

/// A [VectorProblem] where each dimension has a limited range.
pub trait LimitedVectorProblem: VectorProblem {
    /// Returns the range of the given dimension.
    fn range(&self, dimension: usize) -> Range<Self::T>;
}

/// A [Problem] where one can check for the target.
pub trait HasKnownTarget {
    /// Returns whether the target has been reached.
    fn target_hit(&self, target: SingleObjective) -> bool;
}

/// A [Problem] with known target value.
///
/// This is a stricter requirement than [HasKnownTarget].
/// - When writing a component, prefer [HasKnownTarget] when possible.
/// When implementing this for a problem, always implement [HasKnownTarget] as well.
pub trait HasKnownOptimum {
    fn known_optimum(&self) -> SingleObjective;
}
