//! Step size measures for changes caused by operators.
//!
//! # References
//!
//! \[1\]


use std::any::type_name;
use std::marker::PhantomData;
use better_any::{Tid, TidAble};
use derivative::Derivative;
use serde::Serialize;
use statrs::statistics::Statistics;
use crate::component::AnyComponent;
use crate::{Component, CustomState, ExecResult, Problem, State};
use crate::components::archive;
use crate::lens::{AnyLens, Lens, LensMap};
use crate::logging::extractor::{EntryExtractor, EntryName};
use crate::population::AsSolutions;
use crate::problems::VectorProblem;
use crate::utils::{SerializablePhantom, squared_euclidean};

/// Trait for representing a component that measures the step size of the change caused by an operator.
pub trait StepSizeMeasure<P: Problem>: AnyComponent {
    /// Calculates the step size between two `solutions`.
    fn measure(&self, problem: &P, previous: &[&P::Encoding], current: &[&P::Encoding]) -> (Vec<f64>, Vec<f64>);
}

/// A default implementation of [`Component::execute`] for types implementing [`StepSizeMeasure`].
pub fn step_size_measure<P, T>(component: &T, problem: &P, state: &mut State<P>) -> ExecResult<()>
where
    P: Problem,
    T: StepSizeMeasure<P> + 'static,
{
    let populations = state.populations();
    let current_pop = populations.current();
    let archive = state.borrow_mut::<archive::IntermediateArchive<P>>();
    let previous_pop= archive.archived_population();
    let mut step_size = state.borrow_mut::<StepSize<T>>();

    if current_pop.is_empty() || previous_pop.is_empty() {
        step_size.update((vec![0.0], vec![0.0]));
    } else {
        step_size.update(component.measure(problem, &previous_pop.as_solutions(), &current_pop.as_solutions()));
    }

    Ok(())
}

/// The step size between two snapshots of the population as measured by the component `I`.
#[derive(Tid)]
pub struct StepSize<I: AnyComponent + 'static> {
    /// Mean over all solutions.
    pub step_size: f64,
    /// Variance over all solutions.
    pub variance: f64,
    /// Individual step sizes depending on aspect of interest.
    pub all_steps: Vec<f64>,
    /// Variance of individual step sizes. Not applicable for EuclideanStepSize.
    pub all_var: Vec<f64>,
    marker: PhantomData<I>,
}

impl<I: AnyComponent> StepSize<I> {
    /// Creates a new `StepSize` with initial values of 0 and empty vectors.
    pub fn new() -> Self {
        Self {
            step_size: 0.,
            variance: 0.,
            all_steps: Vec::new(),
            all_var: Vec::new(),
            marker: PhantomData,
        }
    }

    /// Updates the step size using the step size vector.
    pub fn update(&mut self, all_steps: (Vec<f64>, Vec<f64>)) {
        let (a, b) = all_steps;
        self.all_steps = a.clone();
        self.all_var = b.clone();
        self.variance = a.clone().variance();
        self.step_size = a.mean();
    }
}

impl<I: AnyComponent> Default for StepSize<I> {
    fn default() -> Self { Self::new() }
}

impl<I: AnyComponent+ 'static> CustomState<'_> for StepSize<I> {}

/// Lens for accessing the individual step sizes of [`StepSize`].
#[derive(Serialize, Derivative)]
#[serde(bound = "")]
#[derivative(Default(bound = ""), Clone(bound = ""))]
pub struct IndividualStepSizeLens<I>(SerializablePhantom<I>);

impl<I: AnyComponent + 'static> AnyLens for IndividualStepSizeLens<I> {
    type Target = Vec<f64>;
}

impl<I> EntryName for IndividualStepSizeLens<I> {
    fn entry_name() -> &'static str { type_name::<I>() }
}

impl<I> IndividualStepSizeLens<I> {
    /// Construct the lens.
    pub fn new() -> Self { Self(SerializablePhantom::default()) }

    /// Constructs the lens for logging.
    pub fn entry<P>() -> Box<dyn EntryExtractor<P>>
    where
        P: VectorProblem<Element = f64>,
        Self: Lens<P>,
        <Self as AnyLens>::Target: Serialize + Send + 'static, { Box::<Self>::default() }
}

impl<I: AnyComponent + 'static> LensMap for IndividualStepSizeLens<I> {
    type Source = StepSize<I>;

    fn map(&self, source: &Self::Source) -> Self::Target { source.all_steps.clone() }
}

/// Lens for accessing the individual variances of [`StepSize`].
#[derive(Serialize, Derivative)]
#[serde(bound = "")]
#[derivative(Default(bound = ""), Clone(bound = ""))]
pub struct IndividualVarianceLens<I>(SerializablePhantom<I>);

impl<I: AnyComponent + 'static> AnyLens for IndividualVarianceLens<I> {
    type Target = Vec<f64>;
}

impl<I> EntryName for IndividualVarianceLens<I> {
    fn entry_name() -> &'static str { "Individual Variances" }
}

impl<I> IndividualVarianceLens<I> {
    /// Construct the lens.
    pub fn new() -> Self { Self(SerializablePhantom::default()) }

    /// Constructs the lens for logging.
    pub fn entry<P>() -> Box<dyn EntryExtractor<P>>
        where
            P: VectorProblem<Element = f64>,
            Self: Lens<P>,
            <Self as AnyLens>::Target: Serialize + Send + 'static, { Box::<Self>::default() }
}

impl<I: AnyComponent + 'static> LensMap for IndividualVarianceLens<I> {
    type Source = StepSize<I>;

    fn map(&self, source: &Self::Source) -> Self::Target { source.all_var.clone() }
}

/// Lens for accessing the mean step size of [`StepSize`].
#[derive(Serialize, Derivative)]
#[serde(bound = "")]
#[derivative(Default(bound = ""), Clone(bound = ""))]
pub struct MeanStepSizeLens<I>(SerializablePhantom<I>);

impl<I: AnyComponent + 'static> AnyLens for MeanStepSizeLens<I> {
    type Target = f64;
}

impl<I> EntryName for MeanStepSizeLens<I> {
    fn entry_name() -> &'static str { "Mean Step Size" }
}

impl<I> MeanStepSizeLens<I> {
    /// Construct the lens.
    pub fn new() -> Self { Self(SerializablePhantom::default()) }

    /// Constructs the lens for logging.
    pub fn entry<P>() -> Box<dyn EntryExtractor<P>>
        where
            P: VectorProblem<Element = f64>,
            Self: Lens<P>,
            <Self as AnyLens>::Target: Serialize + Send + 'static, { Box::<Self>::default() }
}

impl<I: AnyComponent + 'static> LensMap for MeanStepSizeLens<I> {
    type Source = StepSize<I>;

    fn map(&self, source: &Self::Source) -> Self::Target { source.step_size }
}

/// Lens for accessing the variance of [`StepSize`].
#[derive(Serialize, Derivative)]
#[serde(bound = "")]
#[derivative(Default(bound = ""), Clone(bound = ""))]
pub struct StepSizeVarianceLens<I>(SerializablePhantom<I>);

impl<I: AnyComponent + 'static> AnyLens for StepSizeVarianceLens<I> {
    type Target = f64;
}

impl<I> EntryName for StepSizeVarianceLens<I> {
    fn entry_name() -> &'static str { "Step Size Variance" }
}

impl<I> StepSizeVarianceLens<I> {
    /// Construct the lens.
    pub fn new() -> Self { Self(SerializablePhantom::default()) }

    /// Constructs the lens for logging.
    pub fn entry<P>() -> Box<dyn EntryExtractor<P>>
        where
            P: VectorProblem<Element = f64>,
            Self: Lens<P>,
            <Self as AnyLens>::Target: Serialize + Send + 'static, { Box::<Self>::default() }
}

impl<I: AnyComponent + 'static> LensMap for StepSizeVarianceLens<I> {
    type Source = StepSize<I>;

    fn map(&self, source: &Self::Source) -> Self::Target { source.variance }
}

/// Measures the step size in terms of the Euclidean distance between two solutions.
///
/// The value is stored in the [`StepSize<EuclideanStepSize>`] state.
#[derive(Clone, Serialize)]
pub struct EuclideanStepSize;

impl EuclideanStepSize {
    pub fn from_params() -> Self { Self }

    pub fn new<P>() -> Box<dyn Component<P>>
    where
        P: VectorProblem<Element = f64>, { Box::new(Self::from_params()) }
}

impl<P> StepSizeMeasure<P> for EuclideanStepSize
where
    P: VectorProblem<Element = f64>,
{
    fn measure(&self, _problem: &P, previous: &[&Vec<f64>], current: &[&Vec<f64>]) -> (Vec<f64>, Vec<f64>) {

        let steps: Vec<f64> = previous
            .iter()
            .zip(current.iter())
            .map(|(p, q)| squared_euclidean(p, q).sqrt())
            .collect();
        let vars = vec![0.0; steps.len()];
        (steps, vars)
    }
}

impl<P> Component<P> for EuclideanStepSize
where
    P: VectorProblem<Element = f64>,
{
    fn init(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
        state.insert(StepSize::<Self>::new());
        Ok(())
    }

    fn execute(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        step_size_measure(self, problem, state)
    }
}

/// Measures the step size by calculating the mean distance of the values at the same positions of two solutions.
///
/// The value is stored in the [`StepSize<PositionalStepSize>`] state.
#[derive(Clone, Serialize)]
pub struct PositionalStepSize;

impl PositionalStepSize {
    pub fn from_params() -> Self { Self }

    pub fn new<P>() -> Box<dyn Component<P>>
    where
        P: VectorProblem<Element = f64>, { Box::new(Self::from_params()) }
}

impl<P> StepSizeMeasure<P> for PositionalStepSize
where
    P: VectorProblem<Element = f64>,
{
    fn measure(&self, _problem: &P, previous: &[&P::Encoding], current: &[&P::Encoding]) -> (Vec<f64>, Vec<f64>) {
        let diffs: Vec<Vec<f64>> = previous
            .iter()
            .zip(current.iter())
            .map(|(p, q)|
                p.iter().zip(q.iter()).map(|(v, w)| (v - w).abs()).collect())
            .collect();

        let mut steps: Vec<f64> = vec![];
        let mut vars: Vec<f64> = vec![];
        for i in diffs {
            steps.push(i.clone().mean());
            vars.push(i.variance());
        }
        (steps, vars)
    }
}

impl<P> Component<P> for PositionalStepSize
where
    P: VectorProblem<Element = f64>,
{
    fn init(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
        state.insert(StepSize::<Self>::new());
        Ok(())
    }

    fn execute(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        step_size_measure(self, problem, state)
    }
}

/// Measures the step size by calculating the mean difference per dimension of all solutions.
///
/// The value is stored in the [`StepSize<DimensionalStepSize>`] state.
#[derive(Clone, Serialize)]
pub struct DimensionalStepSize;

impl DimensionalStepSize {
    pub fn from_params() -> Self { Self }

    pub fn new<P>() -> Box<dyn Component<P>>
    where
        P: VectorProblem<Element = f64>, { Box::new(Self::from_params()) }
}

impl<P> StepSizeMeasure<P> for DimensionalStepSize
where
    P: VectorProblem<Element = f64>,
{
    fn measure(&self, problem: &P, previous: &[&P::Encoding], current: &[&P::Encoding]) -> (Vec<f64> ,Vec<f64>) {
        let mut diffs: Vec<Vec<f64>> = vec![];
        let dims = problem.dimension();
        for d in 0..dims {
            let summed: Vec<f64> = previous
                .iter()
                .zip(current.iter())
                .map(|(p, q)| (p[d] - q[d]).abs()).collect();
            diffs.push(summed);
        }

        let mut steps: Vec<f64> = vec![];
        let mut vars: Vec<f64> = vec![];
        for i in diffs {
            steps.push(i.clone().mean());
            vars.push(i.variance());
        }
        (steps, vars)
    }
}

impl<P> Component<P> for DimensionalStepSize
where
    P: VectorProblem<Element = f64>,
{
    fn init(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
        state.insert(StepSize::<Self>::new());
        Ok(())
    }

    fn execute(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        step_size_measure(self, problem, state)
    }
}
