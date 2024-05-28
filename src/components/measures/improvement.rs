//! Improvement measures for changes by operators.
//!
//! # References
//!
//! \[1\] A. Scheibenpflug, S. Wagner, E. Pitzer, B. Burlacu, M. Affenzeller. 2012
//! On the analysis, classification and prediction of metaheuristic algorithm behavior for combinatorial optimization problems.
//! 24th European Modeling and Simulation Symposium, EMSS 1, (2012), 368-372

use std::{any::type_name, marker::PhantomData};

use better_any::{Tid, TidAble};
use derivative::Derivative;
use serde::Serialize;

use crate::{
    component::AnyComponent,
    components::archive,
    lens::{AnyLens, Lens, LensMap},
    logging::extractor::{EntryExtractor, EntryName},
    problems::{LimitedVectorProblem, VectorProblem},
    utils::SerializablePhantom,
    Component, CustomState, ExecResult, Individual, Problem, SingleObjectiveProblem, State,
};

/// Trait for representing a component that measures the improvement of the solutions an operator caused.
pub trait ImprovementMeasure<P: Problem>: AnyComponent {
    /// Calculates the amount of improvement between two `solutions`.
    fn measure(
        &self,
        problem: &P,
        previous: &[Individual<P>],
        current: &[Individual<P>],
    ) -> (Vec<f64>, Vec<f64>);
}

/// A default implementation of [`Component::execute`] for types implementing [`ImprovementMeasure`].
///
/// Note that, if called between or directly after operators, solutions have to be evaluated beforehand in the main loop.
pub fn improvement_measure<P, T>(component: &T, problem: &P, state: &mut State<P>) -> ExecResult<()>
where
    P: Problem,
    T: ImprovementMeasure<P> + 'static,
{
    let current_pop = state.populations_mut().pop();

    let cur = current_pop.clone();

    state.populations_mut().push(cur);

    let archive = state.borrow_mut::<archive::IntermediateArchive<P>>();
    let previous_pop = archive.archived_population();
    let mut improvement = state.borrow_mut::<Improvement<T>>();

    if previous_pop.is_empty() {
        improvement.update((vec![0.0], vec![0.0]));
    } else {
        improvement.update(component.measure(problem, previous_pop, &current_pop));
    }

    Ok(())
}

/// The improvement between two snapshots of the population as measured by the component `I`.
#[derive(Tid)]
pub struct Improvement<I: AnyComponent + 'static> {
    /// Percentages better than previous solutions.
    pub percent_improvement: Vec<f64>,
    /// Amounts better than previous solutions.
    pub total_improvement: Vec<f64>,
    marker: PhantomData<I>,
}

impl<I: AnyComponent> Improvement<I> {
    /// Creates a new `Improvement` with empty vectors.
    pub fn new() -> Self {
        Self {
            percent_improvement: Vec::new(),
            total_improvement: Vec::new(),
            marker: PhantomData,
        }
    }

    /// Updates the improvement using the total and the percentage vectors.
    pub fn update(&mut self, improvement: (Vec<f64>, Vec<f64>)) {
        let (a, b) = improvement;
        self.percent_improvement.clone_from(&a);
        self.total_improvement.clone_from(&b);
    }
}

impl<I: AnyComponent> Default for Improvement<I> {
    fn default() -> Self {
        Self::new()
    }
}

impl<I: AnyComponent + 'static> CustomState<'_> for Improvement<I> {}

/// Lens for accessing the improvement values of [`Improvement`].
#[derive(Serialize, Derivative)]
#[serde(bound = "")]
#[derivative(Default(bound = ""), Clone(bound = ""))]
pub struct TotalImprovementLens<I>(SerializablePhantom<I>);

impl<I: AnyComponent + 'static> AnyLens for TotalImprovementLens<I> {
    type Target = Vec<f64>;
}

impl<I> EntryName for TotalImprovementLens<I> {
    fn entry_name() -> &'static str {
        type_name::<I>()
    }
}

impl<I> TotalImprovementLens<I> {
    /// Construct the lens.
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

impl<I: AnyComponent + 'static> LensMap for TotalImprovementLens<I> {
    type Source = Improvement<I>;

    fn map(&self, source: &Self::Source) -> Self::Target {
        source.total_improvement.clone()
    }
}

/// Lens for accessing the improvement percentages of [`Improvement`].
#[derive(Serialize, Derivative)]
#[serde(bound = "")]
#[derivative(Default(bound = ""), Clone(bound = ""))]
pub struct PercentageImprovementLens<I>(SerializablePhantom<I>);

impl<I: AnyComponent + 'static> AnyLens for PercentageImprovementLens<I> {
    type Target = Vec<f64>;
}

impl<I> EntryName for PercentageImprovementLens<I> {
    fn entry_name() -> &'static str {
        "Improvement in percent"
    }
}

impl<I> PercentageImprovementLens<I> {
    /// Construct the lens.
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

impl<I: AnyComponent + 'static> LensMap for PercentageImprovementLens<I> {
    type Source = Improvement<I>;

    fn map(&self, source: &Self::Source) -> Self::Target {
        source.percent_improvement.clone()
    }
}

/// Measures the improvement by calculating the total and percental difference between a solution
/// before and after the application of an operator.
///
/// Note that the results are flawed if the operator shuffles the population.
///
/// The values are stored in the [`Improvement<FitnessImprovement>`] state.
#[derive(Clone, Serialize)]
pub struct FitnessImprovement;

impl FitnessImprovement {
    pub fn from_params() -> Self {
        Self
    }

    pub fn new_with_id<P>() -> Box<dyn Component<P>>
    where
        P: LimitedVectorProblem<Element = f64>,
        P: SingleObjectiveProblem,
    {
        Box::new(Self::from_params())
    }
}

impl FitnessImprovement {
    pub fn new<P>() -> Box<dyn Component<P>>
    where
        P: VectorProblem<Element = f64>,
        P: SingleObjectiveProblem,
    {
        Box::new(Self::from_params())
    }
}

impl<P> ImprovementMeasure<P> for FitnessImprovement
where
    P: VectorProblem<Element = f64>,
    P: SingleObjectiveProblem,
{
    fn measure(
        &self,
        _problem: &P,
        previous: &[Individual<P>],
        current: &[Individual<P>],
    ) -> (Vec<f64>, Vec<f64>) {
        let mut diffs = vec![];
        let mut percents = vec![];

        for u in 0..current.len() {
            let diff = previous[u].objective().value() - current[u].objective().value();
            let frac = (previous[u].objective().value() / current[u].objective().value()) * 100.0;
            diffs.push(diff);
            percents.push(frac);
        }
        (percents, diffs)
    }
}

impl<P> Component<P> for FitnessImprovement
where
    P: VectorProblem<Element = f64>,
    P: SingleObjectiveProblem,
{
    fn init(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
        state.insert(Improvement::<Self>::new());
        Ok(())
    }

    fn execute(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        improvement_measure(self, problem, state)
    }
}
