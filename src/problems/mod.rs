//! Collection of common test problems.

use crate::framework::{Individual, MultiObjective, Objective, SingleObjective};
use std::fmt::Debug;
use std::{any::Any, ops::Range};

pub mod bmf;
pub mod coco_bound;
pub mod tsp;

#[cfg(never)]
pub mod coco;

/// Represents the (global) optimum of the search space.
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct Optimum<S, O: Objective> {
    pub objective: O,
    pub solution: Option<S>,
}

pub trait Problem: 'static {
    type Encoding: Any + Clone + PartialEq;
    type Objective: Objective;

    fn evaluate_solution(&self, solution: &Self::Encoding) -> Self::Objective;

    fn evaluate(&self, individual: &mut Individual) {
        let solution = individual.solution::<Self::Encoding>();
        let objective = self.evaluate_solution(solution);
        individual.evaluate(objective);
    }

    fn name(&self) -> &str;
}

pub trait SingleObjectiveProblem: Problem<Objective = SingleObjective> {}

impl<P: Problem<Objective = SingleObjective>> SingleObjectiveProblem for P {}

pub trait MultiObjectiveProblem: Problem<Objective = MultiObjective> {}

impl<P: Problem<Objective = MultiObjective>> MultiObjectiveProblem for P {}

pub trait VectorProblem: Problem {
    type T: Any + Clone;

    fn dimension(&self) -> usize;
}

pub trait LimitedVectorProblem: VectorProblem {
    fn range(&self, dimension: usize) -> Range<Self::T>;
}

pub trait DebugProblem: Problem {
    fn debug_fmt(&self, individual: &Individual) -> String;
}

pub trait BatchEvaluationProblem: Problem {
    fn evaluate_batch(&self, population: &mut [Individual]) {
        for individual in population.iter_mut() {
            self.evaluate(individual);
        }
    }
}

pub trait HasKnownTarget {
    fn target_hit(&self, target: SingleObjective) -> bool;
}

pub trait HasKnownOptimum {
    fn known_optimum(&self) -> SingleObjective;
}
