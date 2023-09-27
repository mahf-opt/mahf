//! Encoded solution to an optimization problem with an associated
//! (optional) objective value.

use std::fmt::{Debug, Formatter};

use mahf::state::registry::LocalStateRegistry;

use crate::problems::Problem;

/// An encoded solution with an associated (optional) objective value.
///
/// The objective value is automatically reset when mutating the underlying solution.
pub struct Individual<P: Problem + ?Sized> {
    solution: P::Encoding,
    objective: Option<P::Objective>,
    registry: LocalStateRegistry,
}

impl<P: Problem + ?Sized> Individual<P> {
    /// Constructs a new individual from a given solution and objective value.
    ///
    /// # Examples
    ///
    /// Creating `n` individuals with some default solution and infinite objective value:
    ///
    /// ```
    /// use mahf::{Individual, SingleObjectiveProblem};
    ///
    /// pub fn create_n_default_individuals<P>(n: usize) -> Vec<Individual<P>>
    /// where
    ///     P: SingleObjectiveProblem,
    ///     P::Encoding: Default,
    /// {
    ///     (0..n)
    ///         .map(|_| Individual::new(P::Encoding::default(), f64::INFINITY.try_into().unwrap()))
    ///         .collect()
    /// }
    /// ```
    pub fn new(solution: P::Encoding, objective: P::Objective) -> Self {
        Self {
            solution,
            objective: Some(objective),
            registry: LocalStateRegistry::new(),
        }
    }

    /// Constructs a new `Individual` from a solution, leaving it unevaluated.
    ///
    /// Note that the [`IntoIndividuals`] trait allows this for collections of solutions.
    ///
    /// [`IntoIndividuals`]: crate::population::IntoIndividuals
    ///
    /// # Examples
    ///
    /// Creating `n` individuals with some default solution and no objective value.
    ///
    /// Note that [`Individual::default`] could be used instead.
    ///
    /// ```
    /// use mahf::{Individual, Problem};
    ///
    /// pub fn create_n_default_individuals<P>(n: usize) -> Vec<Individual<P>>
    /// where
    ///     P: Problem,
    ///     P::Encoding: Default,
    /// {
    ///     (0..n)
    ///         .map(|_| Individual::new_unevaluated(P::Encoding::default()))
    ///         .collect()
    /// }
    pub fn new_unevaluated(solution: P::Encoding) -> Self {
        Self {
            solution,
            objective: None,
            registry: LocalStateRegistry::new(),
        }
    }

    /// Evaluates the the solution with some objective function.
    ///
    /// This method is usually only called in [`Evaluate`] implementations.
    ///
    /// [`Evaluate`]: crate::problems::Evaluate
    pub fn evaluate_with<F>(&mut self, mut objective_fn: F)
    where
        F: FnMut(&P::Encoding) -> P::Objective,
    {
        self.objective = Some(objective_fn(&self.solution));
    }

    /// Sets the objective value directly, returning if an existing value was overwritten.
    ///
    /// This method is usually only called in [`Evaluate`] implementations.
    ///
    /// [`Evaluate`]: crate::problems::Evaluate
    pub fn set_objective(&mut self, objective: P::Objective) -> bool {
        let evaluated = self.objective.is_some();
        self.objective = Some(objective);
        evaluated
    }

    /// Returns a reference to the solution.
    ///
    /// # Examples
    ///
    /// ```
    /// use mahf::{problems::VectorProblem, Individual};
    ///
    /// pub fn example<P>(problem: &P)
    /// where
    ///     P: VectorProblem<Element = f64>,
    /// {
    ///     // The default solution for `VectorProblem<Element=f64>` is all zeros.
    ///     assert_eq!(
    ///         Individual::<P>::default().solution(),
    ///         &vec![0.0; problem.dimension()]
    ///     )
    /// }
    /// ```
    pub fn solution(&self) -> &P::Encoding {
        &self.solution
    }

    /// Returns the mutable reference to the solution, resetting the objective value.
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use mahf::{problems::VectorProblem, Individual, SingleObjective};
    ///
    /// pub fn example<P>(problem: &P)
    /// where
    ///     P: VectorProblem<Element = f64, Objective = SingleObjective>,
    /// {
    ///     let mut individual =
    ///         Individual::<P>::new(P::Encoding::default(), f64::INFINITY.try_into().unwrap());
    ///     assert!(individual.is_evaluated());
    ///     // Overwrite the solution with all ones.
    ///     *individual.solution_mut() = vec![1.0; problem.dimension()];
    ///     // The objective value was reset.
    ///     assert!(!individual.is_evaluated());
    /// }
    /// ```
    pub fn solution_mut(&mut self) -> &mut P::Encoding {
        self.objective = None;
        &mut self.solution
    }

    /// Returns the solution, consuming the individual in the process.
    ///
    /// # Examples
    ///
    /// ```
    /// use mahf::{problems::VectorProblem, Individual};
    ///
    /// pub fn example<P>(problem: &P)
    /// where
    ///     P: VectorProblem<Element = f64>,
    /// {
    ///     // The default solution for `VectorProblem<Element=f64>` is all zeros.
    ///     assert_eq!(
    ///         Individual::<P>::default().into_solution(),
    ///         vec![0.0; problem.dimension()]
    ///     )
    /// }
    /// ```
    pub fn into_solution(self) -> P::Encoding {
        self.solution
    }

    /// Returns if the `Individual` contains an objective value.
    ///
    /// # Examples
    ///
    /// ```
    /// use mahf::{problems::VectorProblem, Individual, SingleObjective};
    ///
    /// pub fn example<P>(problem: &P)
    /// where
    ///     P: VectorProblem<Element = f64, Objective = SingleObjective>,
    /// {
    ///     // Explicitly assign Inf as objective value.
    ///     let individual =
    ///         Individual::<P>::new(P::Encoding::default(), f64::INFINITY.try_into().unwrap());
    ///     assert!(individual.is_evaluated());
    ///     // `Individual::default` constructs the individual without objective value.
    ///     let individual = Individual::<P>::default();
    ///     assert!(!individual.is_evaluated());
    /// }
    /// ```
    pub fn is_evaluated(&self) -> bool {
        self.objective.is_some()
    }

    /// Returns the objective value.
    ///
    /// # Examples
    ///
    /// ```
    /// use mahf::{problems::VectorProblem, Individual, SingleObjective};
    ///
    /// pub fn example<P>(problem: &P)
    /// where
    ///     P: VectorProblem<Element = f64, Objective = SingleObjective>,
    /// {
    ///     // Explicitly assign Inf as objective value.
    ///     let individual =
    ///         Individual::<P>::new(P::Encoding::default(), f64::INFINITY.try_into().unwrap());
    ///     assert_eq!(
    ///         individual.get_objective(),
    ///         Some(&f64::INFINITY.try_into().unwrap())
    ///     );
    ///     // `Individual::default` constructs the individual without objective value.
    ///     let individual = Individual::<P>::default();
    ///     assert_eq!(individual.get_objective(), None);
    /// }
    /// ```
    pub fn get_objective(&self) -> Option<&P::Objective> {
        self.objective.as_ref()
    }

    /// Returns the objective value.
    ///
    /// # Panics
    ///
    /// Panics if the individual is not evaluated.
    ///
    /// Use [`is_evaluated`] to check for evaluation beforehand or use [`get_objective`].
    ///
    /// [`is_evaluated`]: Individual::is_evaluated
    /// [`get_objective`]: Individual::get_objective
    ///
    /// # Examples
    ///
    /// ```
    /// use mahf::{problems::VectorProblem, Individual, SingleObjective};
    ///
    /// pub fn example<P>(problem: &P)
    /// where
    ///     P: VectorProblem<Element = f64, Objective = SingleObjective>,
    /// {
    ///     // Explicitly assign Inf as objective value.
    ///     let individual =
    ///         Individual::<P>::new(P::Encoding::default(), f64::INFINITY.try_into().unwrap());
    ///     assert_eq!(individual.objective(), &f64::INFINITY.try_into().unwrap());
    ///     // `Individual::default` constructs the individual without objective value.
    ///     let individual = Individual::<P>::default();
    ///     // `individual.objective()` panics.
    /// }
    /// ```
    pub fn objective(&self) -> &P::Objective {
        self.objective.as_ref().unwrap()
    }

    pub fn state(&self) -> &LocalStateRegistry {
        &self.registry
    }

    pub fn state_mut(&mut self) -> &mut LocalStateRegistry {
        &mut self.registry
    }
}

impl<P> Individual<P>
where
    P: Problem,
    P::Encoding: Default,
{
    /// Construct a pseudo individual using the default solution.
    ///
    /// Should only be used for testing.
    pub(crate) fn new_test_unit(objective: P::Objective) -> Self {
        Self::new(P::Encoding::default(), objective)
    }
}

impl<P> Default for Individual<P>
where
    P: Problem,
    P::Encoding: Default,
{
    /// Constructs a new individual using the default solution and no objective value.
    ///
    /// # Examples
    ///
    /// ```
    /// use mahf::{problems::VectorProblem, Individual};
    ///
    /// pub fn example<P>(problem: &P)
    /// where
    ///     P: VectorProblem<Element = f64>,
    /// {
    ///     assert_eq!(
    ///         Individual::<P>::default(),
    ///         Individual::new_unevaluated(P::Encoding::default())
    ///     );
    /// }
    /// ```
    fn default() -> Self {
        Self::new_unevaluated(P::Encoding::default())
    }
}

impl<P: Problem> Clone for Individual<P> {
    fn clone(&self) -> Self {
        Self {
            solution: self.solution.clone(),
            objective: self.objective.clone(),
            registry: self.registry.clone(),
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
