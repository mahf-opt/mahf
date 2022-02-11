use crate::framework::Fitness;
use std::any::Any;

/// An encoded solution with its associated fitness value.
pub struct Individual {
    solution: Box<dyn Any>,
    fitness: Fitness,
    clone: fn(&Box<dyn Any>) -> Box<dyn Any>,
}

impl Individual {
    /// Constructs a new `Individual`.
    pub fn new<T: Any + Clone>(solution: T, fitness: Fitness) -> Self {
        let solution = Box::new(solution);
        let clone  = T::typed_clone;
        Individual { solution, fitness, clone }
    }

    /// Construct a pseudo individual.
    ///
    /// Should only be used for testing.
    pub fn new_test_unit(fitness: f64) -> Self {
        let fitness = Fitness::try_from(fitness).unwrap();
        Individual::new((), fitness)
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
}

impl Clone for Individual {
    fn clone(&self) -> Self {
        Individual { solution: (self.clone)(&self.solution), fitness: self.fitness, clone: self.clone }
    }
}

trait TypedClone {
    fn typed_clone(this: &Box<dyn Any>) -> Box<dyn Any>;
}
impl<T> TypedClone for T where T: Any + Clone {
    fn typed_clone(this: &Box<dyn Any>) -> Box<dyn Any> {
        let this: &T = this.downcast_ref().unwrap();
        Box::new(this.clone())
    }
}
