use crate::{fitness::Fitness, tracking::Log};
use std::convert::TryFrom;

pub mod components;
pub mod heuristic;
pub mod operators;

pub struct State<'a> {
    pub evaluations: u32,
    pub iterations: u32,
    pub progress: f64,
    pub best_so_far: Fitness,
    logger: &'a mut Log,
}

pub type Solution = Vec<f64>;

pub struct Individual {
    solution: Solution,
    fitness: Fitness,
}

impl Individual {
    pub fn solution(&self) -> &Solution {
        &self.solution
    }
    pub fn fitness(&self) -> Fitness {
        self.fitness
    }
}

impl<'a> State<'a> {
    pub fn new(logger: &'a mut Log) -> Self {
        State {
            evaluations: 0,
            iterations: 0,
            progress: 0.0,
            best_so_far: Fitness::try_from(f64::INFINITY).unwrap(),
            logger,
        }
    }

    pub fn log_evaluation(&mut self, fitness: Fitness) {
        self.evaluations += 1;
        if fitness < self.best_so_far {
            self.best_so_far = fitness;
        }
        self.logger
            .log_evaluation(self.evaluations, fitness.into(), self.best_so_far.into());
    }

    pub fn log_iteration(&mut self, diversity: f64) {
        self.iterations += 1;
        self.logger
            .log_iteration(self.iterations, self.best_so_far.into(), diversity);
    }
}
