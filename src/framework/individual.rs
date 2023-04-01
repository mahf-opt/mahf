use crate::problems::Problem;
use std::fmt::{Debug, Formatter};

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
    ///
    /// Note that this method is usually called with an objective value generated by a [Problem].
    ///
    /// # Panics
    ///
    /// Panics if the individual already contains a valid objective value.
    pub fn evaluate(&mut self, objective: P::Objective) {
        if self.objective.is_some() {
            // TODO: this should only emit a warning or maybe just be ignored.
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

    /// Returns the individuals solution, consuming the `Individual` in the process.
    pub fn into_solution(self) -> P::Encoding {
        self.solution
    }

    /// Returns `true` if the `Individual` is evaluated.
    pub fn is_evaluated(&self) -> bool {
        self.objective.is_some()
    }

    /// Returns the objective as `Option`.
    pub fn optional_objective(&self) -> Option<&P::Objective> {
        self.objective.as_ref()
    }

    /// Returns the objective value.
    ///
    /// # Panics
    ///
    /// Panics if the individual is not evaluated.
    ///
    /// Use [Individual::is_evaluated] to check for evaluation beforehand,
    /// or use [Individual::optional_objective] directly to avoid panicking.
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
        let objective = if let Some(objective) = self.optional_objective() {
            format!("{:?}", objective)
        } else {
            format!("{:?}", None::<E>)
        };

        write!(
            f,
            "Individual(solution={:?}, objective={})",
            self.solution, objective
        )
    }
}
