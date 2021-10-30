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

    /// Construct a pseudo individual.
    ///
    /// Should only be used for testing.
    pub fn new_test_unit(fitness: f64) -> Self {
        let solution = Box::new(());
        let fitness = Fitness::try_from(fitness).unwrap();
        Individual { solution, fitness }
    }

    /// Returns the individuals solution.
    ///
    /// # Panics
    /// This will panic when `E` is not the right type.
    pub fn solution<E: Any>(&self) -> &E {
        self.solution.downcast_ref().unwrap()
    }

    /// Returns the individuals solution.
    ///
    /// # Panics
    /// This will panic when `E` is not the right type.
    pub fn into_solution<E: Any>(self) -> E {
        *self.solution.downcast().unwrap()
    }

    /// Returns the individuals fitness value.
    pub fn fitness(&self) -> Fitness {
        self.fitness
    }

    #[allow(clippy::should_implement_trait)]
    pub fn clone<E: Any + Clone>(&self) -> Self {
        Individual {
            solution: Box::new(self.solution::<E>().clone()),
            fitness: self.fitness(),
        }
    }
}
