use crate::framework::{Objective, SingleObjective, MultiObjective};
use std::any::Any;
use std::fmt::{Debug, Formatter};

/// An encoded solution with its associated fitness value.
pub struct Individual {
    solution: Box<dyn Any>,
    objective: Option<Box<dyn Any>>,
    clone_solution: fn(&Box<dyn Any>) -> Box<dyn Any>,
    clone_objective: fn(&Box<dyn Any>) -> Box<dyn Any>,
    partial_eq_solution: fn(&Box<dyn Any>, &Box<dyn Any>) -> bool,
}

impl Individual {
    /// Constructs a new `Individual`.
    pub fn new<T: Any + Clone + PartialEq, O: Objective>(solution: T, objective: O) -> Self {
        Self::new_with_optional_objective(solution, Some(objective))
    }

    pub fn new_unevaluated<T: Any + Clone + PartialEq, O: Objective>(solution: T) -> Self {
        Self::new_with_optional_objective::<T, O>(solution, None)
    }

    fn new_with_optional_objective<T: Any + Clone + PartialEq, O: Objective>(
        solution: T,
        objective: Option<O>,
    ) -> Self {
        let solution = Box::new(solution);
        let objective = objective.map(|objective| -> Box<dyn Any> { Box::new(objective) });
        let clone_solution = T::typed_clone;
        let clone_objective = O::typed_clone;
        let partial_eq_solution = T::typed_partial_eq;
        Individual {
            solution,
            objective,
            clone_solution,
            clone_objective,
            partial_eq_solution,
        }
    }

    pub fn evaluate<O: Objective>(&mut self, objective: O) {
        if self.objective.is_some() {
            panic!("Individual got evaluated twice");
        }
        self.objective = Some(Box::new(objective));
    }

    /// Construct a pseudo individual.
    ///
    /// Should only be used for testing.
    pub fn new_test_unit<O: Objective>(objective: O) -> Self {
        Individual::new((), objective)
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
        self.objective = None;
        self.solution.downcast_mut().unwrap()
    }

    /// Returns the individuals solution.
    ///
    /// # Panics
    /// This will panic when `E` is not the right type.
    pub fn into_solution<E: Any>(self) -> E {
        *self.solution.downcast().unwrap()
    }

    pub fn is_evaluated(&self) -> bool {
        self.objective.is_some()
    }

    pub fn optional_objective<O: Any>(&self) -> Option<&O> {
        match self.objective.as_ref() {
            None => None,
            Some(objective) => Some(objective.downcast_ref().unwrap()),
        }
    }

    pub fn objective<O: Any>(&self) -> &O {
        self.objective.as_ref().unwrap().downcast_ref().unwrap()
    }
}

impl Clone for Individual {
    fn clone(&self) -> Self {
        Individual {
            solution: (self.clone_solution)(&self.solution),
            objective: self.objective.as_ref().map(|o| (self.clone_objective)(o)),
            clone_solution: self.clone_solution,
            clone_objective: self.clone_objective,
            partial_eq_solution: self.partial_eq_solution,
        }
    }
}

impl PartialEq for Individual {
    fn eq(&self, other: &Self) -> bool {
        (self.partial_eq_solution)(&self.solution, &other.solution)
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

impl Debug for Individual {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let objective_repr = if let Some(objective) = self.objective.as_ref() {
            if let Some(single) = objective.downcast_ref::<SingleObjective>() {
                single.value().to_string()
            } else if let Some(multi) = objective.downcast_ref::<MultiObjective>() {
                format!("{:?}", multi.value())
            }
            else {
                format!("Unknown")
            }
        } else {
            format!("Unevaluated")
        };

        write!(f, "Individual(objective={})", objective_repr)
    }
}
