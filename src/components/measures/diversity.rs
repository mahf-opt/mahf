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
//!
//! \[3\] A. Mascarenhas, Y. Kobayashi and C. Aranha. 2024.
//! Novel Genotypic Diversity Metrics for Real-Coded Optimization on Multi-Modal Problems.
//! 2024 IEEE Congress on Evolutionary Computation (CEC), Yokohama, Japan, pp. 1-8.
//! DOI: <https://10.1109/CEC60901.2024.10611897>.

use std::{any::type_name, marker::PhantomData};

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
    utils::SerializablePhantom,
    CustomState, Problem, State,
};

/// Trait for representing a component that measures the diversity of the population.
pub trait DiversityMeasure<P: Problem>: AnyComponent {
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
pub struct Diversity<I: AnyComponent + 'static> {
    /// Normalized diversity.
    pub diversity: f64,
    /// Non-normalized maximal diversity.
    pub max_diversity: f64,
    marker: PhantomData<I>,
}

impl<I: AnyComponent> Diversity<I> {
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

impl<I: AnyComponent> Default for Diversity<I> {
    fn default() -> Self {
        Self::new()
    }
}

impl<I: AnyComponent + 'static> CustomState<'_> for Diversity<I> {}

/// Lens for accessing the normalized diversity of [`Diversity`].
///
/// The diversity is normalized by dividing through the maximal yet encountered diversity,
/// scaling it between 0 and 1.
#[derive(Serialize, Derivative)]
#[serde(bound = "")]
#[derivative(Default(bound = ""), Clone(bound = ""))]
pub struct NormalizedDiversityLens<I>(SerializablePhantom<I>);

impl<I: AnyComponent + 'static> AnyLens for NormalizedDiversityLens<I> {
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

/// Measures the minimum sum of individual distances as described by Mascarenhas et al.
///
/// The value is stored in the [`Diversity<MinimumIndividualDistance>`] state.
#[derive(Clone, Serialize)]
pub struct MinimumIndividualDistance;

impl MinimumIndividualDistance {
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

impl<P> DiversityMeasure<P> for MinimumIndividualDistance
where
    P: VectorProblem<Element = f64>,
{
    fn measure(&self, _problem: &P, solutions: &[&Vec<f64>]) -> f64 {
        let n = solutions.len();

        let mut min_dist = vec![-1.0; n];

        for (ind1_i, ind1) in solutions.iter().enumerate() {
            for ind2 in solutions.iter() {
                if ind1 == ind2 {
                    continue;
                }

                let mut sum_d = 0.0;
                for (x1, x2) in ind1.iter().zip(ind2.iter()) {
                    sum_d += (x1 - x2).powi(2);
                }
                let d = sum_d.sqrt(); // Euclidean distance

                if d < min_dist[ind1_i] || min_dist[ind1_i] == -1.0 {
                    min_dist[ind1_i] = d;
                }
            }
        }
        min_dist.iter().sum::<f64>()
    }
}

impl<P> Component<P> for MinimumIndividualDistance
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

/// Measures the radius diversity as described by Mascarenhas et al.
///
/// The value is stored in the [`Diversity<RadiusDiversity>`] state.
///
/// *The code for this measure was generated with the help of ChatGPT (GPT-3.5) using the code
/// provided by Mascarenhas et al. at <https://zenodo.org/records/11077281> and
/// <https://github.com/mascarenhasav/wcci_2024_gdms_paper>.*
#[derive(Clone, Serialize)]
pub struct RadiusDiversity;

impl RadiusDiversity {
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

impl<P> DiversityMeasure<P> for RadiusDiversity
where
    P: VectorProblem<Element = f64>,
{
    fn measure(&self, _problem: &P, solutions: &[&Vec<f64>]) -> f64 {
        // Calculate the distance matrix using the Euclidean distance
        let mut dist_matrix = vec![vec![0.0; solutions.len()]; solutions.len()];
        for i in 0..solutions.len() {
            for j in 0..solutions.len() {
                if i != j {
                    let sum_sq: f64 = solutions[i]
                        .iter()
                        .zip(solutions[j].iter())
                        .map(|(x1, x2)| (x1 - x2).powi(2))
                        .sum();
                    dist_matrix[i][j] = sum_sq.sqrt();
                }
            }
        }

        let mut selected_flag = vec![false; solutions.len()];
        let original_indices: Vec<usize> = (0..solutions.len()).collect();

        let sigma = dist_matrix
            .iter()
            .flat_map(|row| row.iter())
            .cloned()
            .fold(f64::NEG_INFINITY, f64::max);
        let max_indices: Vec<usize> = dist_matrix
            .iter()
            .enumerate()
            .flat_map(|(i, row)| row.iter().enumerate().map(move |(j, &val)| (i, j, val)))
            .filter(|&(_, _, val)| val == sigma)
            .map(|(i, _j, _)| i)
            .take(2)
            .collect();

        selected_flag[max_indices[0]] = true;
        selected_flag[max_indices[1]] = true;

        let mut selected_indices = vec![max_indices[0], max_indices[1]];
        let mut sigma_list = vec![0.0, sigma];

        while selected_indices.len() < solutions.len() {
            let shortest_distances_list: Vec<f64> = (0..solutions.len())
                .filter(|&i| !selected_flag[i])
                .map(|i| {
                    (0..solutions.len())
                        .filter(|&j| selected_flag[j])
                        .map(|j| dist_matrix[i][j])
                        .fold(f64::INFINITY, f64::min)
                })
                .collect();

            let max_index = shortest_distances_list
                .iter()
                .cloned()
                .enumerate()
                .fold((0, f64::NEG_INFINITY), |(max_idx, max_val), (idx, val)| {
                    if val > max_val {
                        (idx, val)
                    } else {
                        (max_idx, max_val)
                    }
                })
                .0;

            let max_original_index = original_indices
                .iter()
                .filter(|&&idx| !selected_flag[idx])
                .nth(max_index)
                .unwrap();

            selected_flag[*max_original_index] = true;
            selected_indices.push(*max_original_index);
            let sigma = shortest_distances_list[max_index];
            sigma_list.push(sigma);
        }

        sigma_list.iter().sum()
    }
}

impl<P> Component<P> for RadiusDiversity
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
