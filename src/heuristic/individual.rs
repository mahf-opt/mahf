use crate::fitness::Fitness;
use std::any::Any;

/// An encoded solution with its associated fitness value.
pub struct Individual {
    solution: Box<dyn Any>,
    fitness: Fitness,
}

impl Individual {
    /// Constructs a new `Individual`.
    pub fn new(solution: Box<dyn Any>, fitness: Fitness) -> Self {
        Individual { solution, fitness }
    }

    /// Returns the individuals solution.
    ///
    /// # Panics
    /// This will panic when `E` is not the right type.
    pub fn solution<E: Any>(&self) -> &E {
        &self.solution.downcast_ref().unwrap()
    }

    /// Returns the individuals fitness value.
    pub fn fitness(&self) -> Fitness {
        self.fitness
    }
}
