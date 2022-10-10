//! Collection of common test problems.
//!
//! Every problem implements the [Problem] trait.
//!
//! If a given problem has certain properties, then those will be expressed as traits as well.
//! Those traits are quite specific and when writing a component they'll allow you to only require those you really need.

use crate::framework::{Individual, MultiObjective, Objective, SingleObjective};
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

    /// Evaluate a single solution.
    ///
    /// Likely going to be moved into its own in the future.
    fn evaluate_solution(&self, solution: &Self::Encoding) -> Self::Objective;

    /// Evaluate an individual using [Self::evaluate_solution].
    fn evaluate(&self, individual: &mut Individual<Self>) {
        let objective = self.evaluate_solution(individual.solution());
        individual.evaluate(objective);
    }

    /// The name of the problem.
    fn name(&self) -> &str;
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

/// A [Problem] which can be evaluated in batches.
pub trait BatchEvaluationProblem: Problem {
    fn evaluate_batch(&self, population: &mut [Individual<Self>]) {
        for individual in population.iter_mut() {
            self.evaluate(individual);
        }
    }
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
