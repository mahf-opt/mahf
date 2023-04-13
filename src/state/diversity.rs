//! Diversity Measures

use std::marker::PhantomData;

use better_any::Tid;
use serde::Serialize;

use crate::{
    components::Component,
    framework::AnyComponent,
    problems::{Problem, VectorProblem},
    state::{common::Populations, CustomState, State},
};

/// Specialized component trait to measure population diversity.
///
/// # Implementing [Component]
///
/// Types implementing this trait can implement [Component] by wrapping the type in a [DiversityMeasurement].
pub trait DiversityMeasure<P: Problem>: Default {
    fn measure(&self, problem: &P, solutions: &[&P::Encoding]) -> f64;
}

#[derive(serde::Serialize, Clone)]
pub struct DiversityMeasurement<T: Clone>(pub T);

impl<P, I> Component<P> for DiversityMeasurement<I>
where
    P: Problem,
    I: AnyComponent + DiversityMeasure<P> + Serialize + Clone,
{
    fn initialize(&self, _problem: &P, state: &mut State<P>) {
        state.insert(NormalizedDiversity::<I>::default());
    }

    fn execute(&self, problem: &P, state: &mut State<P>) {
        let (populations, diversity_state) =
            state.get_multiple_mut::<(Populations<P>, NormalizedDiversity<I>)>();

        let population = populations.current();

        if population.is_empty() {
            diversity_state.diversity = 0.0;
            return;
        }

        let solutions: Vec<_> = population.iter().map(|i| i.solution()).collect();

        diversity_state.diversity = self.0.measure(problem, solutions.as_slice());

        // Set new maximum diversity found so far
        if diversity_state.diversity > diversity_state.max_diversity {
            diversity_state.max_diversity = diversity_state.diversity
        }

        // Normalize by division with maximum diversity
        diversity_state.diversity /= diversity_state.max_diversity;
    }
}

#[derive(serde::Serialize, Clone, Default)]
pub struct DimensionWiseDiversity;
impl DimensionWiseDiversity {
    pub fn new<P: Problem<Encoding = Vec<f64>> + VectorProblem<T = f64>>() -> Box<dyn Component<P>>
    {
        Box::new(DiversityMeasurement(Self))
    }
}
impl<P: Problem<Encoding = Vec<f64>> + VectorProblem<T = f64>> DiversityMeasure<P>
    for DimensionWiseDiversity
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

#[derive(serde::Serialize, Clone, Default)]
pub struct PairwiseDistanceDiversity;
impl PairwiseDistanceDiversity {
    pub fn new<P: Problem<Encoding = Vec<f64>> + VectorProblem<T = f64>>() -> Box<dyn Component<P>>
    {
        Box::new(DiversityMeasurement(Self))
    }
}
impl<P: Problem<Encoding = Vec<f64>> + VectorProblem<T = f64>> DiversityMeasure<P>
    for PairwiseDistanceDiversity
{
    fn measure(&self, problem: &P, solutions: &[&Vec<f64>]) -> f64 {
        let n = solutions.len() as f64;
        let d = problem.dimension();

        let mut diversity = 0.0;
        let mut sum = 0.0;

        for i in 1..n as usize {
            for j in 0..=i - 1 {
                sum += (0..d)
                    .into_iter()
                    .map(|k| (solutions[i][k] - solutions[j][k]).powi(2))
                    .sum::<f64>();
                diversity += sum.sqrt();
            }
        }

        diversity * 2.0 / (n * (n - 1.0))
    }
}

/// Average standard deviation of each position, i.e, "true diversity".
#[derive(serde::Serialize, Clone, Default)]
pub struct TrueDiversity;
impl TrueDiversity {
    pub fn new<P: Problem<Encoding = Vec<f64>> + VectorProblem<T = f64>>() -> Box<dyn Component<P>>
    {
        Box::new(DiversityMeasurement(Self))
    }
}
impl<P: Problem<Encoding = Vec<f64>> + VectorProblem<T = f64>> DiversityMeasure<P>
    for TrueDiversity
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

#[derive(serde::Serialize, Clone, Default)]
pub struct DistanceToAveragePointDiversity;
impl DistanceToAveragePointDiversity {
    pub fn new<P: Problem<Encoding = Vec<f64>> + VectorProblem<T = f64>>() -> Box<dyn Component<P>>
    {
        Box::new(DiversityMeasurement(Self))
    }
}
impl<P: Problem<Encoding = Vec<f64>> + VectorProblem<T = f64>> DiversityMeasure<P>
    for DistanceToAveragePointDiversity
{
    fn measure(&self, problem: &P, solutions: &[&Vec<f64>]) -> f64 {
        let n = solutions.len() as f64;
        let d = problem.dimension();

        let mut sum = 0.0;

        for i in solutions {
            sum += (0..d)
                .into_iter()
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

/// State for logging/tracking population diversity.
#[derive(Debug, Tid)]
pub struct NormalizedDiversity<I: 'static> {
    /// Normalized diversity.
    pub diversity: f64,
    /// Non-normalized maximal diversity.
    pub max_diversity: f64,
    phantom: PhantomData<I>,
}
impl<I> Default for NormalizedDiversity<I> {
    fn default() -> Self {
        Self {
            diversity: 0.0,
            max_diversity: 0.0,
            phantom: Default::default(),
        }
    }
}

impl<I: Send + 'static> CustomState<'_> for NormalizedDiversity<I> {}
