use crate::problems::{Problem, SingleObjectiveProblem};
use std::fmt::{Debug, Formatter};

/// An encoded solution with its associated fitness value.
pub struct Individual<P: Problem + ?Sized> {
    solution: P::Encoding,
    objective: Option<P::Objective>,
}

impl<P: Problem> Individual<P> {
    /// Constructs a new `Individual`.
    pub fn new(solution: P::Encoding, objective: P::Objective) -> Self {
        Self {
            solution,
            objective: Some(objective),
        }
    }

    pub fn new_unevaluated(solution: P::Encoding) -> Self {
        Self {
            solution,
            objective: None,
        }
    }

    pub fn evaluate(&mut self, objective: P::Objective) {
        if self.objective.is_some() {
            panic!("Individual got evaluated twice");
        }
        self.objective = Some(objective);
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

    /// Returns the individuals solution.
    pub fn into_solution(self) -> P::Encoding {
        self.solution
    }

    pub fn is_evaluated(&self) -> bool {
        self.objective.is_some()
    }

    pub fn optional_objective(&self) -> Option<&P::Objective> {
        self.objective.as_ref()
    }

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

impl<P> Individual<P>
where
    P: SingleObjectiveProblem,
    P::Encoding: Default,
{
    pub fn new_single_objective_test_unit(objective: f64) -> Self {
        Self::new_test_unit(objective.try_into().unwrap())
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
        self.solution == other.solution
    }
}

impl<E: Debug, P: Problem<Encoding = E>> Debug for Individual<P> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let objective = if let Some(objective) = self.optional_objective() {
            format!("{:?}", objective)
        } else {
            format!("{:?}", self.optional_objective())
        };

        write!(
            f,
            "Individual(solution={:?}, objective={})",
            self.solution, objective
        )
    }
}
