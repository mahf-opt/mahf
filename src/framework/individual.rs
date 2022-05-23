use crate::framework::Fitness;
use std::any::Any;

/// An encoded solution with its associated fitness value.
pub struct Individual {
    solution: Box<dyn Any>,
    fitness: Fitness,
    clone: fn(&Box<dyn Any>) -> Box<dyn Any>,
    partial_eq: fn(&Box<dyn Any>, &Box<dyn Any>) -> bool,
}

impl Individual {
    /// Constructs a new `Individual`.
    pub fn new<T: Any + Clone + PartialEq>(solution: T, fitness: Fitness) -> Self {
        let solution = Box::new(solution);
        let clone = T::typed_clone;
        let partial_eq = T::typed_partial_eq;
        Individual {
            solution,
            fitness,
            clone,
            partial_eq,
        }
    }

    pub fn new_unevaluated<T: Any + Clone + PartialEq>(solution: T) -> Self {
        Self::new(solution, Fitness::default())
    }

    pub fn evaluate(&mut self, fitness: Fitness) {
        if self.fitness.is_finite() {
            panic!("Individual got evaluated twice");
        }
        self.fitness = fitness;
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

    /// Returns the mutable individuals solution, resetting the fitness.
    ///
    /// # Panics
    /// This will panic when `E` is not the right type.
    pub fn solution_mut<E: Any>(&mut self) -> &mut E {
        self.fitness = Fitness::default();
        self.solution.downcast_mut().unwrap()
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
        Individual {
            solution: (self.clone)(&self.solution),
            fitness: self.fitness,
            clone: self.clone,
            partial_eq: self.partial_eq,
        }
    }
}

impl PartialEq for Individual {
    fn eq(&self, other: &Self) -> bool {
        self.fitness == other.fitness && (self.partial_eq)(&self.solution, &other.solution)
    }
}

trait TypedClone {
    fn typed_clone(this: &Box<dyn Any>) -> Box<dyn Any>;
}
impl<T> TypedClone for T
where
    T: Any + Clone,
{
    fn typed_clone(this: &Box<dyn Any>) -> Box<dyn Any> {
        let this: &T = this.downcast_ref().unwrap();
        let clone: Self = this.clone();
        Box::new(clone)
    }
}

trait TypedPartialEq {
    fn typed_partial_eq(this: &Box<dyn Any>, other: &Box<dyn Any>) -> bool;
}
impl<T> TypedPartialEq for T
where
    T: Any + PartialEq,
{
    fn typed_partial_eq(this: &Box<dyn Any>, other: &Box<dyn Any>) -> bool {
        let this: &T = this.downcast_ref().unwrap();
        let other: &T = other.downcast_ref().unwrap();
        this == other
    }
}
