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
pub struct Optimum<P: Problem> {
    pub value: P::Objective,
    pub solution: Option<P::Encoding>,
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

pub trait MultiObjectiveProblem: Problem<Objective = MultiObjective> {}

pub trait VectorProblem: Problem {
    type T: Any + Clone;

    fn dimension(&self) -> usize;
}

pub trait LimitedVectorProblem: VectorProblem {
    fn range(&self, dimension: usize) -> Range<Self::T>;
}

pub trait HasKnownOptimum: Problem {
    fn known_optimum(&self) -> Optimum<Self>;
}

pub trait DebugProblem: Problem<Encoding: Debug> {
    fn debug_fmt(&self, individual: &Individual) -> String;
}

pub trait BatchEvaluationProblem: Problem {
    fn evaluate_batch(&self, population: &mut [Individual]) {
        for individual in population.iter_mut() {
            self.evaluate(individual);
        }
    }
}
