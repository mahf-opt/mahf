//! Mutation-like Operators

use itertools::izip;
use rand::{prelude::SliceRandom, seq::IteratorRandom, Rng};
use rand_distr::Distribution;
use serde::{Deserialize, Serialize};

use crate::state::common;
use crate::{
    components::Component,
    framework::{AnyComponent, Individual},
    problems::{LimitedVectorProblem, Problem, VectorProblem},
    state::{common::Progress, State},
};

/// Specialized component trait to generate a new population from the current one.
///
/// This trait is especially useful for components that modify solutions independently.
/// For combining multiple solutions, see [Recombination].
///
/// # Implementing [Component]
///
/// Types implementing this trait can implement [Component] by wrapping the type in a [Generator].
pub trait Generation<P: Problem> {
    fn generate_population(
        &self,
        population: &mut Vec<P::Encoding>,
        problem: &P,
        state: &mut State<P>,
    );
}

#[derive(serde::Serialize, Clone)]
pub struct Generator<T: Clone>(pub T);

impl<T, P> Component<P> for Generator<T>
where
    P: Problem,
    T: AnyComponent + Generation<P> + Serialize + Clone,
{
    fn execute(&self, problem: &P, state: &mut State<P>) {
        let population = state.populations_mut().pop();
        let mut population = population
            .into_iter()
            .map(Individual::into_solution)
            .collect();
        self.0.generate_population(&mut population, problem, state);
        let population = population
            .into_iter()
            .map(Individual::<P>::new_unevaluated)
            .collect();
        state.populations_mut().push(population);
    }
}

/// Applies a fixed, component wise delta from a normal distribution.
///
/// Uses a `N(0, deviation)` normal distribution.
/// Currently the same as Gaussian but without mutation rate.
//TODO: maybe change this to generating new value (as in uniform mutation) but with Gaussian distr.
#[derive(Serialize, Deserialize, Clone)]
pub struct FixedDeviationDelta {
    /// Standard Deviation for the mutation.
    pub deviation: f64,
}

impl FixedDeviationDelta {
    pub fn new<P>(deviation: f64) -> Box<dyn Component<P>>
    where
        P: Problem<Encoding = Vec<f64>>,
    {
        Box::new(Generator(Self { deviation }))
    }
}

impl<P> Generation<P> for FixedDeviationDelta
where
    P: Problem<Encoding = Vec<f64>>,
{
    fn generate_population(
        &self,
        population: &mut Vec<P::Encoding>,
        _problem: &P,
        state: &mut State<P>,
    ) {
        let distribution = rand_distr::Normal::new(0.0, self.deviation).unwrap();

        for solution in population {
            for x in solution {
                *x += distribution.sample(state.random_mut());
            }
        }
    }
}

/// Applies an adaptive, component wise delta from a normal distribution as was proposed for IWO.
///
/// The actual deviation gets computed as follows:
/// ```math
/// final_deviation + (1 - progress)^modulation * (initial_deviation - final_deviation)
/// ```
#[derive(Serialize, Deserialize, Clone)]
pub struct IWOAdaptiveDeviationDelta {
    /// Initial standard deviation for the mutation
    pub initial_deviation: f64,
    /// Final standard deviation for the mutation
    ///
    /// Must not be larger than `initial_deviation`.
    pub final_deviation: f64,
    /// Modulation index for the standard deviation.
    pub modulation_index: u32,
}

impl IWOAdaptiveDeviationDelta {
    pub fn new<P>(
        initial_deviation: f64,
        final_deviation: f64,
        modulation_index: u32,
    ) -> Box<dyn Component<P>>
    where
        P: Problem<Encoding = Vec<f64>>,
    {
        assert!(final_deviation <= initial_deviation);
        Box::new(Generator(Self {
            initial_deviation,
            final_deviation,
            modulation_index,
        }))
    }

    fn deviation(&self, progress: f64) -> f64 {
        self.final_deviation
            + (1.0 - progress).powi(self.modulation_index as i32)
                * (self.initial_deviation - self.final_deviation)
    }
}

impl<P> Generation<P> for IWOAdaptiveDeviationDelta
where
    P: Problem<Encoding = Vec<f64>>,
{
    fn generate_population(
        &self,
        population: &mut Vec<P::Encoding>,
        _problem: &P,
        state: &mut State<P>,
    ) {
        let deviation = self.deviation(state.get_value::<Progress<common::Iterations>>());
        let distribution = rand_distr::Normal::new(0.0, deviation).unwrap();

        for solution in population {
            for x in solution {
                *x += distribution.sample(state.random_mut());
            }
        }
    }
}

#[cfg(test)]
mod adaptive_deviation_delta {
    use super::*;

    #[test]
    fn deviation_is_falling() {
        let comp = IWOAdaptiveDeviationDelta {
            initial_deviation: 10.0,
            final_deviation: 1.0,
            modulation_index: 1,
        };
        float_eq::assert_float_eq!(comp.deviation(0.0), 10.0, ulps <= 1);
        float_eq::assert_float_eq!(comp.deviation(0.5), 5.5, ulps <= 1);
        float_eq::assert_float_eq!(comp.deviation(1.0), 1.0, ulps <= 1);
    }
}

/// Applies a uniform mutation to each position depending on mutation rate.
/// The distribution is centered around the middle of the search domain and the value replaces the position of the solution.
///
/// Uses a uniform distribution.
///
/// If rm = 1, all positions of solution are mutated.
#[derive(Serialize, Deserialize, Clone)]
pub struct UniformMutation {
    /// Probability of mutating one position.
    pub rm: f64,
}

impl UniformMutation {
    pub fn new<P>(rm: f64) -> Box<dyn Component<P>>
    where
        P: Problem<Encoding = Vec<f64>> + LimitedVectorProblem<T = f64>,
    {
        Box::new(Generator(Self { rm }))
    }
}

impl<P> Generation<P> for UniformMutation
where
    P: Problem<Encoding = Vec<f64>> + LimitedVectorProblem<T = f64>,
{
    fn generate_population(
        &self,
        population: &mut Vec<P::Encoding>,
        problem: &P,
        state: &mut State<P>,
    ) {
        let rng = state.random_mut();

        for solution in population.iter_mut() {
            for x in solution.iter_mut() {
                if rng.gen_bool(self.rm) {
                    *x = rng.gen_range(problem.range(problem.dimension()));
                }
            }
        }
    }
}

#[cfg(test)]
mod uniform_mutation {
    use crate::problems::bmf::BenchmarkFunction;
    use crate::state::Random;

    use super::*;

    #[test]
    fn all_mutated() {
        let problem = BenchmarkFunction::sphere(3);
        let comp = UniformMutation { rm: 1.0 };
        let mut state = State::new();
        state.insert(Random::testing());
        let mut population = vec![vec![0.1, 0.2, 0.4], vec![0.2, 0.3, 0.6]];
        let parents_length = population.len();
        let solution_length = vec![population[0].len(), population[1].len()];
        comp.generate_population(&mut population, &problem, &mut state);
        assert_eq!(population.len(), parents_length);
        assert_eq!(
            vec![population[0].len(), population[1].len()],
            solution_length
        );
    }
}

/// Applies a gaussian mutation to each position depending on mutation rate.
/// The distribution is centered around 0 and the resulting value is added to the value of the solution.
///
/// Uses a Gaussian distribution.
///
/// If rm = 1, all positions of solution are mutated.
#[derive(Serialize, Deserialize, Clone)]
pub struct GaussianMutation {
    /// Probability of mutating one position.
    pub rm: f64,
    /// Standard Deviation for the mutation.
    pub deviation: f64,
}

impl GaussianMutation {
    pub fn new<P>(rm: f64, deviation: f64) -> Box<dyn Component<P>>
    where
        P: Problem<Encoding = Vec<f64>>,
    {
        Box::new(Generator(Self { rm, deviation }))
    }
}

impl<P> Generation<P> for GaussianMutation
where
    P: Problem<Encoding = Vec<f64>>,
{
    fn generate_population(
        &self,
        population: &mut Vec<P::Encoding>,
        _problem: &P,
        state: &mut State<P>,
    ) {
        let distribution = rand_distr::Normal::new(0.0, self.deviation).unwrap();
        let rng = state.random_mut();

        for solution in population {
            for x in solution {
                if rng.gen_bool(self.rm) {
                    *x += distribution.sample(rng);
                }
            }
        }
    }
}

#[cfg(test)]
mod gaussian_mutation {
    use crate::problems::bmf::BenchmarkFunction;
    use crate::state::Random;

    use super::*;

    #[test]
    fn all_mutated() {
        let problem = BenchmarkFunction::sphere(3);
        let comp = GaussianMutation {
            rm: 1.0,
            deviation: 0.1,
        };
        let mut state = State::new();
        state.insert(Random::testing());
        let mut population = vec![vec![0.1, 0.2, 0.4], vec![0.2, 0.3, 0.6]];
        let parents_length = population.len();
        let solution_length = vec![population[0].len(), population[1].len()];
        comp.generate_population(&mut population, &problem, &mut state);
        assert_eq!(population.len(), parents_length);
        assert_eq!(
            vec![population[0].len(), population[1].len()],
            solution_length
        );
    }
}

/// Applies a bitflip mutation to each position depending on mutation rate.
///
/// Only for binary encodings!
///
/// If rm = 1, all positions of solution are mutated.
#[derive(Serialize, Deserialize, Clone)]
pub struct BitflipMutation {
    /// Probability of mutating one position.
    pub rm: f64,
}

impl BitflipMutation {
    pub fn new<P>(rm: f64) -> Box<dyn Component<P>>
    where
        P: Problem<Encoding = Vec<bool>>,
    {
        Box::new(Generator(Self { rm }))
    }
}

impl<P> Generation<P> for BitflipMutation
where
    P: Problem<Encoding = Vec<bool>>,
{
    fn generate_population(
        &self,
        population: &mut Vec<P::Encoding>,
        _problem: &P,
        state: &mut State<P>,
    ) {
        let rng = state.random_mut();

        for solution in population {
            for x in solution {
                if rng.gen_bool(self.rm) {
                    *x = !*x;
                }
            }
        }
    }
}

/// Applies a swap mutation to n_swap elements depending on mutation probability.
///
/// For more than two elements: swap is performed circular.
#[derive(Serialize, Deserialize, Clone)]
pub struct SwapMutation {
    /// Number of swaps.
    pub n_swap: usize,
}

impl SwapMutation {
    pub fn new<P, D: 'static>(n_swap: usize) -> Box<dyn Component<P>>
    where
        P: Problem<Encoding = Vec<D>>,
    {
        Box::new(Generator(Self { n_swap }))
    }
}

impl<P, D: 'static> Generation<P> for SwapMutation
where
    P: Problem<Encoding = Vec<D>>,
{
    fn generate_population(
        &self,
        population: &mut Vec<P::Encoding>,
        _problem: &P,
        state: &mut State<P>,
    ) {
        assert!(self.n_swap > 1);
        let rng = state.random_mut();

        for solution in population.iter_mut() {
            let dim = solution.len();
            assert!(self.n_swap < dim);
            let mut pos: Vec<usize> = (0..dim).collect();
            pos.shuffle(rng);
            pos.resize(self.n_swap, 0);
            pos.sort_unstable();
            solution.swap(pos[pos.len() - 1], pos[0]);
            if self.n_swap > 2 {
                for _ in 0..self.n_swap - 2 {
                    solution.swap(pos[pos.len() - 1], pos[pos.len() - 2]);
                    pos.remove(pos.len() - 1);
                }
            }
        }
    }
}

#[cfg(test)]
mod swap_mutation {
    use crate::problems::bmf::BenchmarkFunction;
    use crate::state::Random;

    use super::*;

    #[test]
    fn all_mutated() {
        let problem = BenchmarkFunction::sphere(3);
        let comp = SwapMutation { n_swap: 2 };
        let mut state = State::new();
        state.insert(Random::testing());
        let mut population = vec![vec![0.1, 0.2, 0.4, 0.5, 0.9], vec![0.2, 0.3, 0.6, 0.7, 0.8]];
        let parents_length = population.len();
        let solution_length = vec![population[0].len(), population[1].len()];
        comp.generate_population(&mut population, &problem, &mut state);
        assert_eq!(population.len(), parents_length);
        assert_eq!(
            vec![population[0].len(), population[1].len()],
            solution_length
        );
    }
}

/// Applies a scramble mutation to the solution depending on mutation probability.
///
/// Shuffles the solution.
///
/// If pm = 1, the solution is mutated.
#[derive(Serialize, Deserialize, Clone)]
pub struct ScrambleMutation;

impl ScrambleMutation {
    pub fn new<P, D: 'static>() -> Box<dyn Component<P>>
    where
        P: Problem<Encoding = Vec<D>>,
    {
        Box::new(Generator(Self))
    }
}

impl<P, D: 'static> Generation<P> for ScrambleMutation
where
    P: Problem<Encoding = Vec<D>>,
{
    fn generate_population(
        &self,
        population: &mut Vec<P::Encoding>,
        _problem: &P,
        state: &mut State<P>,
    ) {
        let rng = state.random_mut();

        for solution in population.iter_mut() {
            solution.shuffle(rng);
        }
    }
}

#[cfg(test)]
mod scramble_mutation {
    use crate::problems::bmf::BenchmarkFunction;
    use crate::state::Random;

    use super::*;

    #[test]
    fn all_mutated() {
        let problem = BenchmarkFunction::sphere(3);
        let comp = ScrambleMutation;
        let mut state = State::new();
        state.insert(Random::testing());
        let mut population = vec![vec![0.1, 0.2, 0.4, 0.5, 0.9], vec![0.2, 0.3, 0.6, 0.7, 0.8]];
        let parents_length = population.len();
        let solution_length = vec![population[0].len(), population[1].len()];
        comp.generate_population(&mut population, &problem, &mut state);
        assert_eq!(population.len(), parents_length);
        assert_eq!(
            vec![population[0].len(), population[1].len()],
            solution_length
        );
    }
}

/// Applies a insertion mutation to the solution depending on mutation probability.
///
/// Removes one random element of the solution and inserts it on a random position.
///
/// If pm = 1, the solution is mutated.
#[derive(Serialize, Deserialize, Clone)]
pub struct InsertionMutation;

impl InsertionMutation {
    pub fn new<P, D: 'static>() -> Box<dyn Component<P>>
    where
        P: Problem<Encoding = Vec<D>>,
    {
        Box::new(Generator(Self))
    }
}

impl<P, D: 'static> Generation<P> for InsertionMutation
where
    P: Problem<Encoding = Vec<D>>,
{
    fn generate_population(
        &self,
        population: &mut Vec<P::Encoding>,
        _problem: &P,
        state: &mut State<P>,
    ) {
        let rng = state.random_mut();

        for solution in population.iter_mut() {
            let element = solution.remove(rng.gen_range(0..solution.len()));
            solution.insert(rng.gen_range(0..solution.len()), element);
        }
    }
}

#[cfg(test)]
mod insertion_mutation {
    use crate::problems::bmf::BenchmarkFunction;
    use crate::state::Random;

    use super::*;

    #[test]
    fn all_mutated() {
        let problem = BenchmarkFunction::sphere(3);
        let comp = InsertionMutation;
        let mut state = State::new();
        state.insert(Random::testing());
        let mut population = vec![vec![0.1, 0.2, 0.4, 0.5, 0.9], vec![0.2, 0.3, 0.6, 0.7, 0.8]];
        let parents_length = population.len();
        let solution_length = vec![population[0].len(), population[1].len()];
        comp.generate_population(&mut population, &problem, &mut state);
        assert_eq!(population.len(), parents_length);
        assert_eq!(
            vec![population[0].len(), population[1].len()],
            solution_length
        );
    }
}

/// Applies a inversion mutation to the solution depending on mutation probability.
///
/// Takes a random slice of the solution and inverts it.
///
/// If pm = 1, the solution is mutated.
#[derive(Serialize, Deserialize, Clone)]
pub struct InversionMutation;

impl InversionMutation {
    pub fn new<P, D: 'static>() -> Box<dyn Component<P>>
    where
        P: Problem<Encoding = Vec<D>>,
    {
        Box::new(Generator(Self))
    }
}

impl<P, D: 'static> Generation<P> for InversionMutation
where
    P: Problem<Encoding = Vec<D>>,
{
    fn generate_population(
        &self,
        population: &mut Vec<P::Encoding>,
        _problem: &P,
        state: &mut State<P>,
    ) {
        let rng = state.random_mut();
        for solution in population.iter_mut() {
            let dim = solution.len();
            let mut pos: Vec<usize> = (0..dim).collect();
            pos.shuffle(rng);
            pos.resize(2, 0);
            pos.sort_unstable();
            solution[pos[0]..pos[1] + 1].reverse();
        }
    }
}

#[cfg(test)]
mod inversion_mutation {
    use crate::problems::bmf::BenchmarkFunction;
    use crate::state::Random;

    use super::*;

    #[test]
    fn all_mutated() {
        let problem = BenchmarkFunction::sphere(3);
        let comp = InversionMutation;
        let mut state = State::new();
        state.insert(Random::testing());
        let mut population = vec![vec![0.1, 0.2, 0.4, 0.5, 0.9], vec![0.2, 0.3, 0.6, 0.7, 0.8]];
        let parents_length = population.len();
        let solution_length = vec![population[0].len(), population[1].len()];
        comp.generate_population(&mut population, &problem, &mut state);
        assert_eq!(population.len(), parents_length);
        assert_eq!(
            vec![population[0].len(), population[1].len()],
            solution_length
        );
    }
}

/// Applies a translocation mutation to the solution depending on mutation probability.
///
/// Takes a random slice of the solution and inserts it at a new position.
///
/// If pm = 1, the solution is mutated.
#[derive(Serialize, Deserialize, Clone)]
pub struct TranslocationMutation;

impl TranslocationMutation {
    pub fn new<P, D: 'static>() -> Box<dyn Component<P>>
    where
        P: Problem<Encoding = Vec<D>>,
        D: Clone,
    {
        Box::new(Generator(Self))
    }
}

impl<P, D: 'static> Generation<P> for TranslocationMutation
where
    P: Problem<Encoding = Vec<D>>,
    D: Clone,
{
    fn generate_population(
        &self,
        population: &mut Vec<P::Encoding>,
        _problem: &P,
        state: &mut State<P>,
    ) {
        let rng = state.random_mut();
        for solution in population.iter_mut() {
            let dim = solution.len();
            let mut pos: Vec<usize> = (0..dim).choose_multiple(rng, 2);
            pos.sort_unstable();
            // TODO: this is extremely ugly, try to improve later!
            let mut start = solution[0..pos[0]].to_vec();
            let slice = solution[pos[0]..pos[1] + 1].to_vec();
            let mut end = solution[pos[1] + 1..].to_vec();
            start.append(&mut end);
            let r = rng.gen_range(0..start.len());
            for (count, i) in slice.into_iter().enumerate() {
                start.insert(r + count, i);
            }
            *solution = start;
        }
    }
}

#[cfg(test)]
mod translocation_mutation {
    use crate::problems::bmf::BenchmarkFunction;
    use crate::state::Random;

    use super::*;

    #[test]
    fn all_mutated() {
        let problem = BenchmarkFunction::sphere(3);
        let comp = TranslocationMutation;
        let mut state = State::new();
        state.insert(Random::testing());
        let mut population = vec![vec![0.1, 0.2, 0.4, 0.5, 0.9], vec![0.2, 0.3, 0.6, 0.7, 0.8]];
        let parents_length = population.len();
        let solution_length = vec![population[0].len(), population[1].len()];
        comp.generate_population(&mut population, &problem, &mut state);
        assert_eq!(population.len(), parents_length);
        assert_eq!(
            vec![population[0].len(), population[1].len()],
            solution_length
        );
    }
}

/// Performs the special Differential Evolution mutation, similar to an arithmetic crossover.
///
/// Requires a DE selection directly beforehand, e.g., [DEBest][crate::components::selection::DEBest].
#[derive(Serialize, Deserialize, Clone)]
pub struct DEMutation {
    // Number of difference vectors ∈ {1, 2}.
    y: usize,
    // Difference vector scaling ∈ (0, 2].
    f: f64,
}

impl DEMutation {
    pub fn new<P: Problem<Encoding = Vec<f64>> + VectorProblem>(
        y: usize,
        f: f64,
    ) -> Box<dyn Component<P>> {
        assert!((0.0..=2.0).contains(&f));
        assert!([1, 2].contains(&y));
        Box::new(Generator(Self { y, f }))
    }
}

impl<P> Generation<P> for DEMutation
where
    P: Problem<Encoding = Vec<f64>> + VectorProblem,
{
    fn generate_population(
        &self,
        population: &mut Vec<P::Encoding>,
        problem: &P,
        _state: &mut State<P>,
    ) {
        assert_eq!(population.len() % (self.y * 2 + 1), 0);

        let chunks = population.chunks_exact_mut(self.y * 2 + 1);

        // Iterate over chunks of size `y * 2 + 1`, with the first element as base
        for chunk in chunks {
            // Compiler can't guarantee that pattern always matches
            if let [base, remainder @ ..] = chunk {
                let pairs = remainder.chunks_exact(2).map(|chunk| {
                    // Compiler can't guarantee that pattern always matches
                    match chunk {
                        [s1, s2] => (s1, s2),
                        _ => unreachable!(),
                    }
                });

                for (s1, s2) in pairs {
                    for i in 0..problem.dimension() {
                        base[i] += self.f * (s1[i] - s2[i]);
                    }
                }
            } else {
                unreachable!();
            }
        }

        // Keep only the mutated base elements
        let mut index = 0;
        population.retain(|_| {
            index += 1;
            index % (self.y * 2 + 1) == 0
        })
    }
}

#[cfg(test)]
mod de_mutation {
    use crate::problems::bmf::BenchmarkFunction;
    use crate::state::Random;

    use super::*;

    #[test]
    fn all_mutated() {
        let problem = BenchmarkFunction::sphere(3);
        let y = 1;
        let comp = DEMutation { y, f: 1. };
        let mut state = State::new();
        state.insert(Random::testing());
        let mut population = vec![
            vec![0.1, 0.2, 0.4, 0.5, 0.9],
            vec![0.2, 0.3, 0.6, 0.7, 0.8],
            vec![0.1, 0.3, 0.5, 0.7, 0.9],
        ];
        let parents_length = population.len();
        comp.generate_population(&mut population, &problem, &mut state);
        assert_eq!(population.len() * (2 * y + 1), parents_length);
    }
}

/// Applies a mutation only on some uniformly sampled dimensions of the solution.
#[derive(Serialize, derivative::Derivative)]
#[serde(bound = "")]
#[derivative(Clone(bound = ""))]
pub struct UniformPartialMutation<P: Problem> {
    /// Ratio of the solution to be mutated.
    pub ratio: f64,
    /// Mutation to partially apply to the solution.
    pub mutation: Box<dyn Component<P>>,
}

impl<P, D: 'static> UniformPartialMutation<P>
where
    P: Problem<Encoding = Vec<D>>,
    D: Clone,
{
    pub fn new(ratio: f64, mutation: Box<dyn Component<P>>) -> Box<dyn Component<P>> {
        Box::new(Self { ratio, mutation })
    }
}

impl<P, D: 'static> Component<P> for UniformPartialMutation<P>
where
    P: Problem<Encoding = Vec<D>>,
    D: Clone,
{
    fn initialize(&self, problem: &P, state: &mut State<P>) {
        self.mutation.initialize(problem, state);
    }

    fn execute(&self, problem: &P, state: &mut State<P>) {
        let mut partial_population = Vec::new();
        let mut population_indices = Vec::new();

        let mut population = state.populations_mut().pop();

        // Decide which indices/dimensions to mutate,
        // and keep indices and solution from selected indices
        for solution in population.iter_mut() {
            let n = solution.solution().len();
            let amount = (self.ratio * n as f64).floor() as usize;
            let indices = (0..n).choose_multiple(state.random_mut(), amount);
            let partial_solution: Vec<_> = solution
                .solution()
                .iter()
                .enumerate()
                .filter_map(|(i, x)| {
                    if indices.contains(&i) {
                        Some(x.clone())
                    } else {
                        None
                    }
                })
                .collect();

            partial_population.push(Individual::new_unevaluated(partial_solution));
            population_indices.push(indices);
        }

        // Mutate the partial solutions
        state.populations_mut().push(partial_population);
        self.mutation.execute(problem, state);
        let partial_population = state.populations_mut().pop();

        // Insert mutated dimensions into original solutions
        for (indices, solution, partial) in
            izip!(&population_indices, &mut population, partial_population)
        {
            let solution = solution.solution_mut();
            for (i, mutated_x) in izip!(indices, partial.into_solution()) {
                solution[*i] = mutated_x;
            }
        }

        // Push partially mutated population back
        state.populations_mut().push(population);
    }
}
