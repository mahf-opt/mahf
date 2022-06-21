//! Collection of common test problems.

use crate::framework::{Individual, Objective, MultiObjective, SingleObjective};
use std::{any::Any, ops::Range};

pub mod bmf;
pub mod coco_bound;
pub mod tsp;

#[cfg(never)]
pub mod coco;

/// Represents the (global) optimum of the search space.
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct Optimum<S> {
    pub fitness: Fitness,
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

    fn evaluate_population(&self, population: &mut [Individual]) {
        for individual in population.iter_mut() {
            self.evaluate(individual);
        }
    }

    fn name(&self) -> &str;
}

pub trait SingleObjectiveProblem: Problem<Objective=SingleObjective> {}

pub trait MultiObjectiveProblem: Problem<Objective=MultiObjective> {}

pub trait VectorProblem {
    type T: Any + Clone;

    fn dimension(&self) -> usize;
}

pub trait LimitedVectorProblem: VectorProblem {
    fn range(&self, dimension: usize) -> Range<Self::T>;
}

pub trait HasKnownOptimum {
    fn known_optimum(&self) -> f64;
}
