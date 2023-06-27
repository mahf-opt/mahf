//! Diversity measures for populations.

use std::marker::PhantomData;

use better_any::{Tid, TidAble};
use derivative::Derivative;
use serde::Serialize;

use crate::{
    component::{AnyComponent, ExecResult},
    components::Component,
    lens::{AnyLens, Lens, LensMap},
    logging::extractor::{EntryExtractor, EntryName},
    population::AsSolutions,
    problems::VectorProblem,
    CustomState, Problem, State,
};

pub trait DiversityMeasure<P: Problem>: AnyComponent {
    fn measure(&self, problem: &P, solutions: &[&P::Encoding]) -> f64;
}

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

#[derive(Tid)]
pub struct Diversity<I: AnyComponent + 'static> {
    /// Normalized diversity.
    pub diversity: f64,
    /// Non-normalized maximal diversity.
    pub max_diversity: f64,
    marker: PhantomData<I>,
}

impl<I: AnyComponent> Diversity<I> {
    pub fn new() -> Self {
        Self {
            diversity: 0.,
            max_diversity: 0.,
            marker: PhantomData,
        }
    }

    pub fn update(&mut self, diversity: f64) {
        if diversity > self.max_diversity {
            self.max_diversity = diversity;
        }
        self.diversity = diversity / self.max_diversity;
    }
}

impl<I: AnyComponent> Default for Diversity<I> {
    fn default() -> Self {
        Self::new()
    }
}

impl<I: AnyComponent + 'static> CustomState<'_> for Diversity<I> {}

#[derive(Serialize, Derivative)]
#[serde(bound = "")]
#[derivative(Default(bound = ""), Clone(bound = ""))]
pub struct NormalizedDiversityLens<I>(#[serde(skip)] PhantomData<fn() -> I>);

impl<I: AnyComponent + 'static> AnyLens for NormalizedDiversityLens<I> {
    type Target = f64;
}

impl<I> EntryName for NormalizedDiversityLens<I> {
    fn entry_name() -> &'static str {
        "NormalizedDiversity"
    }
}

impl<I> NormalizedDiversityLens<I> {
    pub fn entry<P>() -> Box<dyn EntryExtractor<P>>
    where
        P: VectorProblem<Element = f64>,
        Self: Lens<P>,
        <Self as AnyLens>::Target: Serialize + Send + 'static,
    {
        Box::<Self>::default()
    }
}

impl<I: AnyComponent + 'static> LensMap for NormalizedDiversityLens<I> {
    type Source = Diversity<I>;

    fn map(&self, source: &Self::Source) -> Self::Target {
        source.diversity
    }
}

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
