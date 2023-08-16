//! Diversity measures for populations.
//!
//! # References
//!
//! \[1\] Shi Cheng, Yuhui Shi, Quande Qin, Qingyu Zhang, and Ruibin Bai. 2014.
//! Population Diversity Maintenance In Brain Storm Optimization Algorithm.
//! Journal of Artificial Intelligence and Soft Computing Research 4, 2 (April 2014), 83–97.
//! DOI:<https://doi.org/10/ggrd47>
//!
//! \[2\] Guillaume Corriveau, Raynald Guilbault, Antoine Tahan, and Robert Sabourin. 2012.
//! Review and Study of Genotypic Diversity Measures for Real-Coded Representations.
//! IEEE Transactions on Evolutionary Computation 16, 5 (October 2012), 695–710.
//! DOI:<https://doi.org/10/f4ct44>

use std::{any::type_name, marker::PhantomData};

use better_any::{Tid, TidAble};
use derivative::Derivative;
use serde::Serialize;

use crate::{
    component::{ComponentLike, ExecResult},
    components::Component,
    lens::{BaseLens, Lens, LensMap},
    logging::extractor::{EntryExtractor, EntryName},
    population::AsSolutions,
    problems::VectorProblem,
    utils::SerializablePhantom,
    CustomState, Problem, State,
};

/// Trait for representing a component that measures the diversity of the population.
pub trait DiversityMeasure<P: Problem>: ComponentLike {
    /// Calculates the diversity of the `solutions`.
    fn measure(&self, problem: &P, solutions: &[&P::Encoding]) -> f64;
}

/// A default implementation of [`Component::execute`] for types implementing [`DiversityMeasure`].
pub fn diversity_measure<P, T>(component: &T, problem: &P, state: &mut State<P>) -> ExecResult<()>
where
    P: Problem,
    T: DiversityMeasure<P> + 'static,
{
    let populations = state.populations();
    let population = populations.current();
    let mut diversity = state.borrow_mut::<Diversity<T>>();

    if population.is_empty() {
        diversity.update(0.);
    } else {
        diversity.update(component.measure(problem, &population.as_solutions()));
    }

    Ok(())
}

/// The diversity of the population as measured by the component `I`.
///
/// The normalized diversity value can be accessed using the [`NormalizedDiversityLens<I>`].
#[derive(Tid)]
pub struct Diversity<I: ComponentLike + 'static> {
    /// Normalized diversity.
    pub diversity: f64,
    /// Non-normalized maximal diversity.
    pub max_diversity: f64,
    marker: PhantomData<I>,
}

impl<I: ComponentLike> Diversity<I> {
    /// Creates a new `Diversity` with initial values of 0.
    pub fn new() -> Self {
        Self {
            diversity: 0.,
            max_diversity: 0.,
            marker: PhantomData,
        }
    }

    /// Updates the normalized and maximal diversity using `diversity`.
    pub fn update(&mut self, diversity: f64) {
        if diversity > self.max_diversity {
            self.max_diversity = diversity;
        }
        self.diversity = diversity / self.max_diversity;
    }
}

impl<I: ComponentLike> Default for Diversity<I> {
    fn default() -> Self {
        Self::new()
    }
}

impl<I: ComponentLike + 'static> CustomState<'_> for Diversity<I> {}

/// Lens for accessing the normalized diversity of [`Diversity`].
///
/// The diversity is normalized by dividing through the maximal yet encountered diversity,
/// scaling it between 0 and 1.
#[derive(Serialize, Derivative)]
#[serde(bound = "")]
#[derivative(Default(bound = ""), Clone(bound = ""))]
pub struct NormalizedDiversityLens<I>(SerializablePhantom<I>);

impl<I: ComponentLike + 'static> BaseLens for NormalizedDiversityLens<I> {
    type Target = f64;
}

impl<I> EntryName for NormalizedDiversityLens<I> {
    fn entry_name() -> &'static str {
        type_name::<I>()
    }
}

impl<I> NormalizedDiversityLens<I> {
    /// Constructs the lens.
    pub fn new() -> Self {
        Self(SerializablePhantom::default())
    }

    /// Constructs the lens for logging.
    pub fn entry<P>() -> Box<dyn EntryExtractor<P>>
    where
        P: VectorProblem<Element = f64>,
        Self: Lens<P>,
        <Self as BaseLens>::Target: Serialize + Send + 'static,
    {
        Box::<Self>::default()
    }
}

impl<I: ComponentLike + 'static> LensMap for NormalizedDiversityLens<I> {
    type Source = Diversity<I>;

    fn map(&self, source: &Self::Source) -> Self::Target {
        source.diversity
    }
}

/// Measures the dimension-wise diversity of the population.
///
/// The value is stored in the [`Diversity<DimensionWiseDiversity>`] state.
#[derive(Clone, Serialize)]
pub struct DimensionWiseDiversity;

impl DimensionWiseDiversity {
    pub fn from_params() -> Self {
        Self
    }

    pub fn new<P>() -> Box<dyn Component<P>>
    where
        P: VectorProblem<Element = f64>,
    {
        Box::new(Self::from_params())
    }
}

impl<P> DiversityMeasure<P> for DimensionWiseDiversity
where
    P: VectorProblem<Element = f64>,
{
    fn measure(&self, problem: &P, solutions: &[&Vec<f64>]) -> f64 {
        let n = solutions.len() as f64;
        let d = problem.dimension();

        (0..d)
            .map(|k| {
                let xk = solutions.iter().map(|s| s[k]).sum::<f64>() / n;
                solutions.iter().map(|s| (s[k] - xk).abs()).sum::<f64>() / n
            })
            .sum::<f64>()
            / (d as f64)
    }
}

impl<P> Component<P> for DimensionWiseDiversity
where
    P: VectorProblem<Element = f64>,
{
    fn init(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
        state.insert(Diversity::<Self>::new());
        Ok(())
    }

    fn execute(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        diversity_measure(self, problem, state)
    }
}

/// Measures the pairwise distance between solutions in the population.
///
/// The value is stored in the [`Diversity<PairwiseDistanceDiversity>`] state.
#[derive(Clone, Serialize)]
pub struct PairwiseDistanceDiversity;

impl PairwiseDistanceDiversity {
    pub fn from_params() -> Self {
        Self
    }

    pub fn new<P>() -> Box<dyn Component<P>>
    where
        P: VectorProblem<Element = f64>,
    {
        Box::new(Self::from_params())
    }
}

impl<P> DiversityMeasure<P> for PairwiseDistanceDiversity
where
    P: VectorProblem<Element = f64>,
{
    fn measure(&self, problem: &P, solutions: &[&Vec<f64>]) -> f64 {
        let n = solutions.len() as f64;
        let d = problem.dimension();

        let mut diversity = 0.0;
        let mut sum = 0.0;

        for i in 1..n as usize {
            for j in 0..=i - 1 {
                sum += (0..d)
                    .map(|k| (solutions[i][k] - solutions[j][k]).powi(2))
                    .sum::<f64>();
                diversity += sum.sqrt();
            }
        }

        diversity * 2.0 / (n * (n - 1.0))
    }
}

impl<P> Component<P> for PairwiseDistanceDiversity
where
    P: VectorProblem<Element = f64>,
{
    fn init(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
        state.insert(Diversity::<Self>::new());
        Ok(())
    }

    fn execute(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        diversity_measure(self, problem, state)
    }
}

/// Measures the average standard deviation of each solution in the population, i.e, "true diversity".
///
/// The value is stored in the [`Diversity<TrueDiversity>`] state.
#[derive(Clone, Serialize)]
pub struct TrueDiversity;

impl TrueDiversity {
    pub fn from_params() -> Self {
        Self
    }

    pub fn new<P>() -> Box<dyn Component<P>>
    where
        P: VectorProblem<Element = f64>,
    {
        Box::new(Self::from_params())
    }
}

impl<P> DiversityMeasure<P> for TrueDiversity
where
    P: VectorProblem<Element = f64>,
{
    fn measure(&self, problem: &P, solutions: &[&Vec<f64>]) -> f64 {
        let n = solutions.len() as f64;
        let d = problem.dimension();

        (0..d)
            .map(|k| {
                let xk = solutions.iter().map(|s| s[k]).sum::<f64>() / n;
                let sum = solutions.iter().map(|i| i[k].powi(2)).sum::<f64>() / n;
                sum - xk.powi(2)
            })
            .sum::<f64>()
            .sqrt()
            / (d as f64)
    }
}

impl<P> Component<P> for TrueDiversity
where
    P: VectorProblem<Element = f64>,
{
    fn init(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
        state.insert(Diversity::<Self>::new());
        Ok(())
    }

    fn execute(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        diversity_measure(self, problem, state)
    }
}

/// Measures the distance to the average solution for all solutions in the population.
///
/// The value is stored in the [`Diversity<DistanceToAveragePointDiversity>`] state.
#[derive(Clone, Serialize)]
pub struct DistanceToAveragePointDiversity;

impl DistanceToAveragePointDiversity {
    pub fn from_params() -> Self {
        Self
    }

    pub fn new<P>() -> Box<dyn Component<P>>
    where
        P: VectorProblem<Element = f64>,
    {
        Box::new(Self::from_params())
    }
}

impl<P> DiversityMeasure<P> for DistanceToAveragePointDiversity
where
    P: VectorProblem<Element = f64>,
{
    fn measure(&self, problem: &P, solutions: &[&Vec<f64>]) -> f64 {
        let n = solutions.len() as f64;
        let d = problem.dimension();

        let mut sum = 0.0;

        for i in solutions {
            sum += (0..d)
                .map(|k| {
                    let xk = solutions.iter().map(|s| s[k]).sum::<f64>() / n;
                    (i[k] - xk).powi(2)
                })
                .sum::<f64>()
                .sqrt();
        }

        sum / n
    }
}

impl<P> Component<P> for DistanceToAveragePointDiversity
where
    P: VectorProblem<Element = f64>,
{
    fn init(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
        state.insert(Diversity::<Self>::new());
        Ok(())
    }

    fn execute(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        diversity_measure(self, problem, state)
    }
}
