use std::fmt::{Debug, Formatter};

use crate::problems::Problem;

/// An encoded solution with an associated (optional) objective value.
pub struct Individual<P: Problem + ?Sized> {
    solution: P::Encoding,
    objective: Option<P::Objective>,
}

impl<P: Problem + ?Sized> Individual<P> {
    /// Constructs a new `Individual` from a given solution and objective value.
    pub fn new(solution: P::Encoding, objective: P::Objective) -> Self {
        Self {
            solution,
            objective: Some(objective),
        }
    }

    // Constructs a new `Individual` from a solution, leaving it unevaluated.
    pub fn new_unevaluated(solution: P::Encoding) -> Self {
        Self {
            solution,
            objective: None,
        }
    }

    /// Evaluates the `Individual` with some objective value.
    pub fn evaluate_with<F>(&mut self, objective_fn: F)
    where
        F: Fn(&P::Encoding) -> P::Objective,
    {
        self.objective = Some(objective_fn(&self.solution));
    }

    pub fn set_objective(&mut self, objective: P::Objective) -> bool {
        let evaluated = self.objective.is_some();
        self.objective = Some(objective);
        evaluated
    }

    /// Returns the individuals solution.
    pub fn solution(&self) -> &P::Encoding {
        &self.solution
    }

    /// Returns the mutable individuals solution, resetting the objective value.
    pub fn solution_mut(&mut self) -> &mut P::Encoding {
        self.objective = None;
        &mut self.solution
    }

    /// Returns the individuals solution, consuming the `Individual` in the process.
    pub fn into_solution(self) -> P::Encoding {
        self.solution
    }

    /// Returns if the `Individual` contains an objective value.
    pub fn is_evaluated(&self) -> bool {
        self.objective.is_some()
    }

    /// Returns the objective value.
    pub fn get_objective(&self) -> Option<&P::Objective> {
        self.objective.as_ref()
    }

    /// Returns the objective value.
    ///
    /// # Panics
    ///
    /// Panics if the individual is not evaluated.
    ///
    /// Use [Individual::is_evaluated] to check for evaluation
    /// beforehand or use [Individual::get_objective].
    pub fn objective(&self) -> &P::Objective {
        self.objective.as_ref().unwrap()
    }
}

impl<P> Individual<P>
where
    P: Problem,
    P::Encoding: Default,
{
    /// Construct a pseudo individual.
    ///
    /// Should only be used for testing.
    pub fn new_test_unit(objective: P::Objective) -> Self {
        Self::new(P::Encoding::default(), objective)
    }
}

impl<P> Default for Individual<P>
where
    P: Problem,
    P::Encoding: Default,
{
    fn default() -> Self {
        Self::new_unevaluated(P::Encoding::default())
    }
}

impl<P: Problem> Clone for Individual<P> {
    fn clone(&self) -> Self {
        Self {
            solution: self.solution.clone(),
            objective: self.objective.clone(),
        }
    }
}

impl<P: Problem> PartialEq for Individual<P> {
    fn eq(&self, other: &Self) -> bool {
        self.solution == other.solution && self.objective == other.objective
    }
}

impl<E: Debug, P: Problem<Encoding = E>> Debug for Individual<P> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut debug = f.debug_struct("Individual");

        debug.field("solution", &self.solution);

        if let Some(objective) = self.get_objective() {
            debug.field("objective", objective);
        }

        debug.finish()
    }
}
