//! Evaluate [`Individual`]s according to some objective function.

use std::marker::PhantomData;

use better_any::{Tid, TidAble};
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};
use trait_set::trait_set;

use crate::{CustomState, Individual, Problem, State};

/// Trait for evaluating individuals, i.e. evaluate their solutions to an optimization problem.
///
/// Implement [`ObjectiveFunction`] instead if the objective function is static,
/// i.e. does not depend on `&self` or `&mut self` to automatically implement this trait
/// and gain default implementations for sequential and parallel evaluation through
/// [`Sequential`] and [`Parallel`].
///
/// # Examples
///
/// A simple implementation of the n-dimensional real-valued sphere function `f(x) = x^2`.
///
/// Note that implementing [`ObjectiveFunction`] is preferred in this case, because the
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
/// #[derive(Default)] // No explicit implementation of `TryDefault` necessary through this `derive`.
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

trait_set! {
    /// Collection of traits to allow storing an evaluator, i.e.
    /// a struct implementing [`Evaluate`], in the [`State`].
    pub trait Evaluator = Evaluate + for<'a> CustomState<'a> + for<'a> TidAble<'a>;
}

/// Trait for a static objective function of an optimization problem.
///
/// [`Sequential`] and [`Parallel`] provide a default implementation of sequential and parallel
/// evaluation using the [`objective`], respectively.
///
/// If your objective function is not static, i.e. takes `&self` or `&mut self`, implement
/// [`Evaluate`] directly.
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
///     fn objective(solution: &Self::Encoding) -> Self::Objective {
///            solution
///                .iter()
///                .map(|x| x.powi(2))
///                .sum::<f64>()
///                .try_into()
///                .unwrap()
///     }
/// }
/// ```
pub trait ObjectiveFunction: Problem {
    /// Calculates the objective value of the given `solution`.
    fn objective(solution: &Self::Encoding) -> Self::Objective;
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
    /// use mahf::{Individual, Problem, SingleObjective, State, problems::ObjectiveFunction};
    /// use mahf::problems::evaluate::Sequential;
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
    ///     fn objective(solution: &Self::Encoding) -> Self::Objective {
    ///            solution
    ///                .iter()
    ///                .map(|x| x.powi(2))
    ///                .sum::<f64>()
    ///                .try_into()
    ///                .unwrap()
    ///     }
    /// }
    ///
    /// // Create a sequential evaluator for `Sphere`
    /// let sequential_evaluator = Sequential::<Sphere>::new();
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
        _problem: &Self::Problem,
        _state: &mut State<Self::Problem>,
        individuals: &mut [Individual<Self::Problem>],
    ) {
        for individual in individuals {
            individual.evaluate_with(P::objective);
        }
    }
}

impl<P: ObjectiveFunction> CustomState<'_> for Sequential<P> {}

/// A parallel evaluator for an optimization problem.
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
    /// use mahf::{Individual, Problem, SingleObjective, State, problems::ObjectiveFunction};
    /// use mahf::problems::evaluate::Parallel;
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
    ///     fn objective(solution: &Self::Encoding) -> Self::Objective {
    ///            solution
    ///                .iter()
    ///                .map(|x| x.powi(2))
    ///                .sum::<f64>()
    ///                .try_into()
    ///                .unwrap()
    ///     }
    /// }
    ///
    /// // Create a parallel evaluator for `Sphere`
    /// let parallel_evaluator = Parallel::<Sphere>::new();
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
    P: ObjectiveFunction + Problem,
{
    type Problem = P;

    fn evaluate(
        &mut self,
        _problem: &Self::Problem,
        _state: &mut State<Self::Problem>,
        individuals: &mut [Individual<Self::Problem>],
    ) {
        individuals
            .par_iter_mut()
            .for_each(|individual| individual.evaluate_with(P::objective));
    }
}

impl<P: ObjectiveFunction> CustomState<'_> for Parallel<P> {}
