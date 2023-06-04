use std::{marker::PhantomData, ops::Range};

use better_any::{Tid, TidAble};
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};
use trait_set::trait_set;

use crate::{
    encoding::AnyEncoding,
    individual::Individual,
    objective::{MultiObjective, Objective, SingleObjective},
    CustomState, State,
};

/// Metadata of an optimization problem.
/// This trait is the base trait for all problems, and itself only defines
/// - solution encoding ([`Self::Encoding`]),
/// - objective type ([`Self::Objective`]), and
/// - the name of the problem ([`Self::name`]).
///
/// # Example
///
/// A simple implementation of the one-dimensional real-valued sphere function `f(x) = x^2`:
///
/// ```
/// use mahf::objective::SingleObjective;
/// use mahf::Problem;
///
/// pub struct Sphere1D;
///
/// impl Problem for Sphere1D {
///     type Encoding = Vec<f64>; // real-valued vector
///     type Objective = SingleObjective;
///
///     fn name(&self) -> &str { "Sphere1D" }
/// }
/// ```
pub trait Problem: 'static {
    /// The encoding of a solution to the optimization problem.
    type Encoding: AnyEncoding;

    /// The objective type.
    /// See [`SingleObjective`] and [`MultiObjective`].
    type Objective: Objective;

    /// The name of the problem.
    fn name(&self) -> &str;
}

trait_set! {
    /// An optimization problem with a single objective.
    ///
    /// This trait should be used in favor over specifying the objective type
    /// of [`Problem`] directly, and is automatically implemented for all problems
    /// with [`SingleObjective`] as [`Problem::Objective`].
    pub trait SingleObjectiveProblem = Problem<Objective = SingleObjective>;

    /// An optimization problem with multiple objectives.
    ///
    /// This trait should be used in favor over specifying the objective type
    /// of [`Problem`] directly, and is automatically implemented for all problems
    /// with [`MultiObjective`] as [`Problem::Objective`].
    pub trait MultiObjectiveProblem = Problem<Objective = MultiObjective>;
}

pub trait ObjectiveFunction: Problem + Send {
    fn objective(solution: &Self::Encoding) -> Self::Objective;
}

pub trait Evaluate: Send + Default {
    type Problem: Problem;

    fn evaluate(
        &mut self,
        problem: &Self::Problem,
        state: &mut State<Self::Problem>,
        individuals: &mut [Individual<Self::Problem>],
    );
}

trait_set! {
    pub trait Evaluator = Evaluate + for<'a> CustomState<'a> + for<'a> TidAble<'a>;
}

#[derive(Tid)]
pub struct Sequential<P: ObjectiveFunction + 'static>(PhantomData<P>);

impl<P: ObjectiveFunction> Sequential<P> {
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

#[derive(Tid)]
pub struct Parallel<P: ObjectiveFunction + 'static>(PhantomData<P>);

impl<P: ObjectiveFunction> Parallel<P> {
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

pub trait VectorProblem: Problem<Encoding = Vec<Self::Element>> {
    type Element: Clone;

    fn dimension(&self) -> usize;
}

pub trait LimitedVectorProblem: VectorProblem {
    fn domain(&self) -> Vec<Range<Self::Element>>;
}

pub trait OptimumReachedProblem: SingleObjectiveProblem {
    fn optimum_reached(&self, objective: SingleObjective) -> bool;
}

pub trait KnownOptimumProblem: SingleObjectiveProblem {
    fn known_optimum(&self) -> SingleObjective;
}

impl<P: KnownOptimumProblem> OptimumReachedProblem for P {
    fn optimum_reached(&self, objective: SingleObjective) -> bool {
        let provided = objective.value();
        let known = self.known_optimum().value();

        (provided - known) < 1e-8
    }
}

pub trait TravellingSalespersonProblem:
    SingleObjectiveProblem + VectorProblem<Element = usize>
{
    fn distance(&self, edge: (usize, usize)) -> f64;
}
