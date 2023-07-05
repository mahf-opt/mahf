//! Evaluate [`Individual`]s according to some objective function.

use std::marker::PhantomData;

use better_any::{Tid, TidAble};
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};

use crate::{CustomState, Individual, Problem, State};

/// Trait for evaluating individuals, i.e. evaluate their solutions to an optimization problem.
///
/// Implement [`ObjectiveFunction`] instead if the objective function does not require `&mut self`
/// to automatically implement this trait and gain default implementations for sequential
/// and parallel evaluation through [`Sequential`] and [`Parallel`]
/// (the latter requires the problem to be `Sync`).
///
/// # Examples
///
/// A simple implementation of the n-dimensional real-valued sphere function `f(x) = x^2`.
///
/// Note that implementing [`ObjectiveFunction`] would be preferred in this case, because the
/// objective function only depends on `x`.
///
/// ```
/// use mahf::{Individual, Problem, SingleObjective, State, problems::Evaluate};
///
/// pub struct Sphere {
///     pub dim: usize,
/// }
///
/// impl Problem for Sphere {
///     type Encoding = Vec<f64>;
///     type Objective = SingleObjective;
///
///     fn name(&self) -> &str {
///         "Sphere"
///     }
/// }
///
/// pub struct SequentialSphereEvaluator;
///
/// impl Evaluate for SequentialSphereEvaluator {
///     type Problem = Sphere;
///
///     /// Implements `f(x) = \sum (x_i)^2`.
///     fn evaluate(
///         &mut self,
///         _problem: &Self::Problem,
///         _state: &mut State<Self::Problem>,
///         individuals: &mut [Individual<Self::Problem>])
///     {
///         for individual in individuals {
///             individual.evaluate_with(|solution| {
///                 solution
///                     .iter()
///                     .map(|x| x.powi(2))
///                     .sum::<f64>()
///                     .try_into()
///                     .unwrap()
///             })
///         }
///     }
/// }
/// ```
///
/// The evaluator is specified when executing the configuration:
///
/// ```
/// # use mahf::{Individual, Problem, SingleObjective, State, problems::Evaluate};
/// use mahf::prelude::*;
///
/// # pub struct Sphere {
/// #     pub dim: usize,
/// # }
/// #
/// # impl Problem for Sphere {
/// #     type Encoding = ();
/// #     type Objective = SingleObjective;
/// #
/// #    fn name(&self) -> &str { unimplemented!() }
/// # }
/// #
/// # pub struct SequentialSphereEvaluator;
/// #
/// # impl SequentialSphereEvaluator {
/// #    pub fn new() -> Self {
/// #        Self
/// #    }
/// # }
/// #
/// # impl Evaluate for SequentialSphereEvaluator {
/// #    type Problem = Sphere;
/// #
/// #    fn evaluate(
/// #        &mut self,
/// #        _problem: &Self::Problem,
/// #        _state: &mut State<Self::Problem>,
/// #        individuals: &mut [Individual<Self::Problem>])
/// #    {
/// #        unimplemented!()
/// #    }
/// # }
/// #
/// # fn example(config: Configuration<Sphere>, problem: Sphere) -> ExecResult<()> {
///  // Implicit ...
///  let state = config.optimize(&problem, SequentialSphereEvaluator::new())?;
///  // ... or explicit insertion into the state.
///  let state = config.optimize_with(&problem, |state| {
///     state.insert_evaluator(SequentialSphereEvaluator::new());
///     Ok(())
///  })?;
/// # Ok(())
/// # }
/// ```
pub trait Evaluate: Send {
    /// The type of optimization problem.
    type Problem: Problem;

    /// Evaluates individuals on the [`Problem`].
    ///
    /// [`Problem`]: Evaluate::Problem
    fn evaluate(
        &mut self,
        problem: &Self::Problem,
        state: &mut State<Self::Problem>,
        individuals: &mut [Individual<Self::Problem>],
    );
}

/// Trait for a non-mutable objective function of an optimization problem.
///
/// [`Sequential`] and [`Parallel`] provide a default implementation of sequential and parallel
/// evaluation using the [`objective`], respectively.
/// The latter requires the [`Problem`] to be `Sync`.
///
/// If your objective function takes `&mut self`, implement [`Evaluate`] directly.
///
/// [`objective`]: ObjectiveFunction::objective
///
/// # Examples
///
/// A simple implementation of the n-dimensional real-valued sphere function `f(x) = x^2`.
///
/// ```
/// use mahf::{Individual, Problem, SingleObjective, State, problems::ObjectiveFunction};
///
/// pub struct Sphere {
///     pub dim: usize,
/// }
///
/// impl Problem for Sphere {
///     type Encoding = Vec<f64>;
///     type Objective = SingleObjective;
///
///     fn name(&self) -> &str {
///         "Sphere"
///     }
/// }
///
/// impl ObjectiveFunction for Sphere {
///     /// Implements `f(x) = \sum (x_i)^2`.
///     fn objective(&self, solution: &Self::Encoding) -> Self::Objective {
///             debug_assert_eq!(self.dim, solution.len());
///             solution
///                 .iter()
///                 .map(|x| x.powi(2))
///                 .sum::<f64>()
///                 .try_into()
///                 .unwrap()
///     }
/// }
/// ```
///
/// [`Sequential`] and [`Parallel`] can be used as evaluators:
///
/// ```
/// # use mahf::{Individual, Problem, SingleObjective, State, problems::ObjectiveFunction};
/// use mahf::prelude::*;
/// #
/// # pub struct Sphere {
/// #     pub dim: usize,
/// # }
/// #
/// # impl Problem for Sphere {
/// #     type Encoding = Vec<f64>;
/// #     type Objective = SingleObjective;
/// #
/// #    fn name(&self) -> &str {
/// #        "Sphere"
/// #    }
/// # }
/// #
/// # impl ObjectiveFunction for Sphere {
/// #    fn objective(&self, solution: &Self::Encoding) -> Self::Objective {
/// #            unimplemented!()
/// #    }
/// # }
///
/// # fn example(config: Configuration<Sphere>, problem: Sphere) -> ExecResult<()> {
///  // Implicit insertion into the state ...
///  let state = config.optimize(&problem, evaluate::Sequential::new())?;
///  // ... or explicit.
///  let state = config.optimize_with(&problem, |state| {
///     state.insert_evaluator(evaluate::Sequential::new());
///     Ok(())
///  })?;
/// # Ok(())
/// # }
/// ```
pub trait ObjectiveFunction: Problem {
    /// Calculates the objective value of the given `solution`.
    fn objective(&self, solution: &Self::Encoding) -> Self::Objective;
}

/// A sequential evaluator for an optimization problem, i.e. [`ObjectiveFunction`].
///
/// The evaluator simply evaluates all individuals sequentially in order.
#[derive(Tid)]
pub struct Sequential<P: ObjectiveFunction + 'static>(PhantomData<fn() -> P>);

impl<P: ObjectiveFunction> Sequential<P> {
    /// Creates a new instance of a sequential evaluator for a problem `P`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mahf::{Individual, Problem, SingleObjective, State, problems::ObjectiveFunction};
    /// use mahf::problems::evaluate::Sequential;
    ///
    /// # fn example<P: ObjectiveFunction>() {
    /// // Create a sequential evaluator for `P`.
    /// let sequential_evaluator = Sequential::<P>::new();
    /// # }
    /// ```
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<P: ObjectiveFunction> Default for Sequential<P> {
    fn default() -> Self {
        Self::new()
    }
}

impl<P> Evaluate for Sequential<P>
where
    P: ObjectiveFunction,
{
    type Problem = P;

    fn evaluate(
        &mut self,
        problem: &Self::Problem,
        _state: &mut State<Self::Problem>,
        individuals: &mut [Individual<Self::Problem>],
    ) {
        for individual in individuals {
            individual.evaluate_with(|solution| problem.objective(solution));
        }
    }
}

impl<P: ObjectiveFunction> CustomState<'_> for Sequential<P> {}

/// A parallel evaluator for an optimization problem.
///
/// Requires `P` to be `Sync`.
///
/// The evaluator evaluates the individuals in parallel using the [`rayon`] library.
#[derive(Tid)]
pub struct Parallel<P: ObjectiveFunction + 'static>(PhantomData<fn() -> P>);

impl<P: ObjectiveFunction> Parallel<P> {
    /// Creates a new instance of a parallel evaluator for a problem `P`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mahf::{Individual, Problem, SingleObjective, State, problems::ObjectiveFunction};
    /// use mahf::problems::evaluate::Parallel;
    ///
    /// # fn example<P: ObjectiveFunction>() {
    /// // Create a sequential evaluator for `P`.
    /// let parallel_evaluator = Parallel::<P>::new();
    /// # }
    /// ```
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<P: ObjectiveFunction> Default for Parallel<P> {
    fn default() -> Self {
        Self::new()
    }
}

impl<P> Evaluate for Parallel<P>
where
    P: ObjectiveFunction + Problem + Sync,
{
    type Problem = P;

    fn evaluate(
        &mut self,
        problem: &Self::Problem,
        _state: &mut State<Self::Problem>,
        individuals: &mut [Individual<Self::Problem>],
    ) {
        individuals.par_iter_mut().for_each(|individual| {
            individual.evaluate_with(|solution| problem.objective(solution))
        });
    }
}

impl<P: ObjectiveFunction> CustomState<'_> for Parallel<P> {}

impl<P> Default for Box<dyn Evaluate<Problem = P>>
where
    P: ObjectiveFunction,
{
    fn default() -> Self {
        // Use sequential evaluation by default.
        Box::new(Sequential::<P>::new())
    }
}
