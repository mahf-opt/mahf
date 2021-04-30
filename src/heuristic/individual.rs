use crate::fitness::Fitness;
use std::any::Any;

pub struct Individual {
    solution: Box<dyn Any>,
    fitness: Fitness,
}

impl Individual {
    pub fn new(solution: Box<dyn Any>, fitness: Fitness) -> Self {
        Individual { solution, fitness }
    }

    pub fn solution<E: Any>(&self) -> &E {
        &self.solution.downcast_ref().unwrap()
    }

    pub fn fitness(&self) -> Fitness {
        self.fitness
    }
}
