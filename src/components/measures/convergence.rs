//! Convergence rate measures for best solutions.
//!
//! # References
//!
//! \[1\] Halim, A.H., Ismail, I. & Das, S. 2020.
//! Performance assessment of the metaheuristic optimization algorithms: an exhaustive review.
//! Artif Intell Rev 54, 2323–2409 (2021).
//! DOI: <https://doi.org/10.1007/s10462-020-09906-6>

use crate::component::AnyComponent;
use crate::components::archive;
use crate::lens::{AnyLens, Lens, LensMap};
use crate::logging::extractor::{EntryExtractor, EntryName};
use crate::problems::{KnownOptimumProblem, VectorProblem};
use crate::utils::SerializablePhantom;
use crate::{Component, CustomState, ExecResult, Problem, SingleObjectiveProblem, State};
use better_any::{Tid, TidAble};
use derivative::Derivative;
use serde::Serialize;
use std::{any::type_name, marker::PhantomData};

/// Trait for representing a component that measures the convergence rate.
pub trait ConvergenceRateMeasure<P: Problem>: AnyComponent {
    /// Calculates the convergence rate.
    fn measure(&self, problem: &P, previous: f64, current: f64) -> f64;
}

pub fn convergence_rate_measure<P, T>(
    component: &T,
    problem: &P,
    state: &mut State<P>,
) -> ExecResult<()>
where
    P: Problem + SingleObjectiveProblem,
    T: ConvergenceRateMeasure<P> + 'static,
{
    let mut convergence_rate = state.borrow_mut::<ConvergenceRate<T>>();
    let archive = state.borrow_mut::<archive::BestIndividualsArchive<P>>();
    let best_individuals = archive.archived_best_individuals();

    let len = best_individuals.len();
    if len > 1 {
        let current_best = best_individuals[len - 1].clone().objective().value();
        let previous_best = best_individuals[len - 2].clone().objective().value();
        convergence_rate.update(component.measure(problem, previous_best, current_best));
    } else {
        convergence_rate.update(0.0);
    }

    Ok(())
}

/// The convergence rate as measured by the component `I`.
#[derive(Tid)]
pub struct ConvergenceRate<I: AnyComponent + 'static> {
    pub convergence_rate: f64,
    marker: PhantomData<I>,
}

impl<I: AnyComponent> ConvergenceRate<I> {
    /// Creates a new `ConvergenceRate` with initial values of 0.
    pub fn new() -> Self {
        Self {
            convergence_rate: 0.,
            marker: PhantomData,
        }
    }

    /// Updates the convergence rate.
    pub fn update(&mut self, convergence_rate: f64) {
        self.convergence_rate = convergence_rate;
    }
}

impl<I: AnyComponent> Default for ConvergenceRate<I> {
    fn default() -> Self {
        Self::new()
    }
}

impl<I: AnyComponent + 'static> CustomState<'_> for ConvergenceRate<I> {}

/// Lens for accessing the convergence rate of [`ConvergenceRate`].
#[derive(Serialize, Derivative)]
#[serde(bound = "")]
#[derivative(Default(bound = ""), Clone(bound = ""))]
pub struct ConvergenceRateLens<I>(SerializablePhantom<I>);

impl<I: AnyComponent + 'static> AnyLens for ConvergenceRateLens<I> {
    type Target = f64;
}

impl<I> EntryName for ConvergenceRateLens<I> {
    fn entry_name() -> &'static str {
        type_name::<I>()
    }
}

impl<I> ConvergenceRateLens<I> {
    /// Constructs the lens.
    pub fn new() -> Self {
        Self(SerializablePhantom::default())
    }

    /// Constructs the lens for logging.
    pub fn entry<P>() -> Box<dyn EntryExtractor<P>>
    where
        P: VectorProblem<Element = f64>,
        Self: Lens<P>,
        <Self as AnyLens>::Target: Serialize + Send + 'static,
    {
        Box::<Self>::default()
    }
}

impl<I: AnyComponent + 'static> LensMap for ConvergenceRateLens<I> {
    type Source = ConvergenceRate<I>;

    fn map(&self, source: &Self::Source) -> Self::Target {
        source.convergence_rate
    }
}

/// Measures the convergence rate between two iterations if the optimum is known.
///
/// The value is stored in the [`ConvergenceRate<KnownOptimumIterationWiseConvergence>`] state.
#[derive(Clone, Serialize)]
pub struct KnownOptimumIterationWiseConvergence;

impl KnownOptimumIterationWiseConvergence {
    pub fn from_params() -> Self {
        Self
    }

    pub fn new<P>() -> Box<dyn Component<P>>
    where
        P: VectorProblem<Element = f64> + KnownOptimumProblem,
    {
        Box::new(Self::from_params())
    }
}

impl<P> ConvergenceRateMeasure<P> for KnownOptimumIterationWiseConvergence
where
    P: VectorProblem<Element = f64> + KnownOptimumProblem,
{
    fn measure(&self, problem: &P, previous: f64, current: f64) -> f64 {
        let optimum = problem.known_optimum().value();

        (optimum - current).abs() / (optimum - previous).abs()
    }
}

impl<P> Component<P> for KnownOptimumIterationWiseConvergence
where
    P: VectorProblem<Element = f64> + KnownOptimumProblem,
{
    fn init(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
        state.insert(ConvergenceRate::<Self>::new());
        Ok(())
    }

    fn execute(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        convergence_rate_measure(self, problem, state)
    }
}

/// Measures the convergence progressive rate between two iterations if the optimum is known.
///
/// The value is stored in the [`ConvergenceRate<KnownOptimumConvergenceProgressiveRate>`] state.
#[derive(Clone, Serialize)]
pub struct KnownOptimumConvergenceProgressiveRate;

impl KnownOptimumConvergenceProgressiveRate {
    pub fn from_params() -> Self {
        Self
    }

    pub fn new<P>() -> Box<dyn Component<P>>
    where
        P: VectorProblem<Element = f64> + SingleObjectiveProblem + KnownOptimumProblem,
    {
        Box::new(Self::from_params())
    }
}

impl<P> ConvergenceRateMeasure<P> for KnownOptimumConvergenceProgressiveRate
where
    P: VectorProblem<Element = f64> + SingleObjectiveProblem + KnownOptimumProblem,
{
    fn measure(&self, problem: &P, _previous: f64, current: f64) -> f64 {
        let optimum = problem.known_optimum().value();

        (optimum - current).abs()
    }
}

impl<P> Component<P> for KnownOptimumConvergenceProgressiveRate
where
    P: VectorProblem<Element = f64> + SingleObjectiveProblem + KnownOptimumProblem,
{
    fn init(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
        state.insert(ConvergenceRate::<Self>::new());
        Ok(())
    }

    fn execute(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        convergence_rate_measure(self, problem, state)
    }
}

/// Measures the convergence progressive rate between two iterations if the optimum is unknown.
///
/// The value is stored in the [`ConvergenceRate<UnknownOptimumConvergenceProgressiveRate>`] state.
#[derive(Clone, Serialize)]
pub struct UnknownOptimumConvergenceProgressiveRate;

impl UnknownOptimumConvergenceProgressiveRate {
    pub fn from_params() -> Self {
        Self
    }

    pub fn new<P>() -> Box<dyn Component<P>>
    where
        P: VectorProblem<Element = f64> + SingleObjectiveProblem,
    {
        Box::new(Self::from_params())
    }
}

impl<P> ConvergenceRateMeasure<P> for UnknownOptimumConvergenceProgressiveRate
where
    P: VectorProblem<Element = f64> + SingleObjectiveProblem,
{
    fn measure(&self, _problem: &P, previous: f64, current: f64) -> f64 {
        (current - previous).abs()
    }
}

impl<P> Component<P> for UnknownOptimumConvergenceProgressiveRate
where
    P: VectorProblem<Element = f64> + SingleObjectiveProblem,
{
    fn init(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
        state.insert(ConvergenceRate::<Self>::new());
        Ok(())
    }

    fn execute(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        convergence_rate_measure(self, problem, state)
    }
}

/// Measures the logarithmic convergence rate between two iterations if the optimum is known.
///
/// The value is stored in the [`ConvergenceRate<KnownOptimumLogarithmicConvergenceRate>`] state.
#[derive(Clone, Serialize)]
pub struct KnownOptimumLogarithmicConvergenceRate;

impl KnownOptimumLogarithmicConvergenceRate {
    pub fn from_params() -> Self {
        Self
    }

    pub fn new<P>() -> Box<dyn Component<P>>
    where
        P: VectorProblem<Element = f64> + SingleObjectiveProblem + KnownOptimumProblem,
    {
        Box::new(Self::from_params())
    }
}

impl<P> ConvergenceRateMeasure<P> for KnownOptimumLogarithmicConvergenceRate
where
    P: VectorProblem<Element = f64> + SingleObjectiveProblem + KnownOptimumProblem,
{
    fn measure(&self, problem: &P, _previous: f64, current: f64) -> f64 {
        let optimum = problem.known_optimum().value();

        let convergence_rate = (optimum - current).abs();
        convergence_rate.log10()
    }
}

impl<P> Component<P> for KnownOptimumLogarithmicConvergenceRate
where
    P: VectorProblem<Element = f64> + SingleObjectiveProblem + KnownOptimumProblem,
{
    fn init(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
        state.insert(ConvergenceRate::<Self>::new());
        Ok(())
    }

    fn execute(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        convergence_rate_measure(self, problem, state)
    }
}
