//! Generation methods

use std::cmp::max;

use rand::seq::IteratorRandom;
use rand::{prelude::SliceRandom, Rng};
use rand_distr::Distribution;
use serde::{Deserialize, Serialize};

use crate::{
    framework::{
        common_state::{Population, Progress},
        components::*,
        specializations::{
            Generation, Generator, Mutation, PositionMutator, Recombination, Recombinator,
            SolutionMutator,
        },
        Individual, State,
    },
    operators::custom_state::PsoState,
    problems::{LimitedVectorProblem, Problem},
    random::Random,
};

// Random Operators without state //
pub use crate::operators::initialization::RandomPermutation;
pub use crate::operators::initialization::RandomSpread;

// Mutation-like Operators //

/// Applies a fixed, component wise delta from a normal distribution.
///
/// Uses a `N(0, deviation)` normal distribution.
/// Currently the same as Gaussian but without mutation rate.
//TODO: maybe change this to generating new value (as in uniform mutation) but with Gaussian distr.
#[derive(Serialize, Deserialize)]
pub struct FixedDeviationDelta {
    /// Standard Deviation for the mutation.
    pub deviation: f64,
}
impl FixedDeviationDelta {
    pub fn new<P>(deviation: f64) -> Box<dyn Component<P>>
    where
        P: Problem<Encoding = Vec<f64>>,
    {
        Box::new(Generator(PositionMutator(Self { deviation })))
    }
}
impl<P> Mutation<P, f64> for FixedDeviationDelta
where
    P: Problem<Encoding = Vec<f64>>,
{
    fn mutation_func<'p>(
        &self,
        _problem: &P,
        state: &'p mut State,
    ) -> Box<dyn FnMut(&mut f64) + 'p> {
        let distribution = rand_distr::Normal::new(0.0, self.deviation).unwrap();

        Box::new(move |x| *x += distribution.sample(state.random_mut()))
    }
}

/// Applies an adaptive, component wise delta from a normal distribution as was proposed for IWO.
///
/// The actual deviation gets computed as follows:
/// ```math
/// final_deviation + (1 - progress)^modulation * (initial_deviation - final_deviation)
/// ```
#[derive(Serialize, Deserialize)]
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
        Box::new(Generator(PositionMutator(Self {
            initial_deviation,
            final_deviation,
            modulation_index,
        })))
    }

    fn deviation(&self, progress: f64) -> f64 {
        self.final_deviation
            + (1.0 - progress).powi(self.modulation_index as i32)
                * (self.initial_deviation - self.final_deviation)
    }
}
impl<P> Mutation<P, f64> for IWOAdaptiveDeviationDelta
where
    P: Problem<Encoding = Vec<f64>>,
{
    fn mutation_func<'p>(
        &self,
        _problem: &P,
        state: &'p mut State,
    ) -> Box<dyn FnMut(&mut f64) + 'p> {
        let deviation = self.deviation(state.get_value::<Progress>());
        let distribution = rand_distr::Normal::new(0.0, deviation).unwrap();

        Box::new(move |x| *x += distribution.sample(state.random_mut()))
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
#[derive(Serialize, Deserialize)]
pub struct UniformMutation {
    /// Probability of mutating one position.
    pub rm: f64,
}
impl UniformMutation {
    pub fn new<P>(rm: f64) -> Box<dyn Component<P>>
    where
        P: Problem<Encoding = Vec<f64>> + LimitedVectorProblem<T = f64>,
    {
        Box::new(Generator(PositionMutator(Self { rm })))
    }
}
impl<P> Mutation<P, f64> for UniformMutation
where
    P: Problem<Encoding = Vec<f64>> + LimitedVectorProblem<T = f64>,
{
    fn mutation_func<'p>(
        &self,
        problem: &'p P,
        state: &'p mut State,
    ) -> Box<dyn FnMut(&mut f64) + 'p> {
        let rng = state.random_mut();
        let &Self { rm } = self;

        Box::new(move |x| {
            if rng.gen_bool(rm) {
                *x = rng.gen_range(problem.range(problem.dimension()));
            }
        })
    }
}

#[cfg(test)]
mod uniform_mutation {
    use crate::problems::bmf::BenchmarkFunction;

    use super::*;

    #[test]
    fn all_mutated() {
        // let problem = BenchmarkFunction::sphere(3);
        // let comp = UniformMutation { rm: 1.0 };
        // let mut state = State::new_root();
        // let mut rng = Random::testing();
        // let mut parents = vec![vec![0.1, 0.2, 0.4], vec![0.2, 0.3, 0.6]];
        // let parents_length = parents.len();
        // let solution_length = vec![parents[0].len(), parents[1].len()];
        // let mut offspring = Vec::new();
        // comp.generate(&mut state, &problem, &mut rng, &mut parents, &mut offspring);
        // assert_eq!(offspring.len(), parents_length);
        // assert_eq!(
        //     vec![offspring[0].len(), offspring[1].len()],
        //     solution_length
        // );
    }
}

/// Applies a gaussian mutation to each position depending on mutation rate.
/// The distribution is centered around 0 and the resulting value is added to the value of the solution.
///
/// Uses a Gaussian distribution.
///
/// If rm = 1, all positions of solution are mutated.
#[derive(Serialize, Deserialize)]
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
        Box::new(Generator(PositionMutator(Self { rm, deviation })))
    }
}
impl<P> Mutation<P, f64> for GaussianMutation
where
    P: Problem<Encoding = Vec<f64>>,
{
    fn mutation_func<'p>(
        &self,
        _problem: &'p P,
        state: &'p mut State,
    ) -> Box<dyn FnMut(&mut f64) + 'p> {
        let &Self { rm, deviation } = self;
        let distribution = rand_distr::Normal::new(0.0, deviation).unwrap();
        let rng = state.random_mut();

        Box::new(move |x| {
            if rng.gen_bool(rm) {
                *x += distribution.sample(rng);
            }
        })
    }
}

#[cfg(test)]
mod gaussian_mutation {
    use crate::problems::bmf::BenchmarkFunction;

    use super::*;

    #[test]
    fn all_mutated() {
        // let problem = BenchmarkFunction::sphere(3);
        // let comp = GaussianMutation {
        //     rm: 1.0,
        //     deviation: 0.1,
        // };
        // let mut state = State::new_root();
        // let mut rng = Random::testing();
        // let mut parents = vec![vec![0.1, 0.2, 0.4], vec![0.2, 0.3, 0.6]];
        // let parents_length = parents.len();
        // let solution_length = vec![parents[0].len(), parents[1].len()];
        // let mut offspring = Vec::new();
        // comp.generate(&mut state, &problem, &mut rng, &mut parents, &mut offspring);
        // assert_eq!(offspring.len(), parents_length);
        // assert_eq!(
        //     vec![offspring[0].len(), offspring[1].len()],
        //     solution_length
        // );
    }
}

/// Applies a bitflip mutation to each position depending on mutation rate.
///
/// Only for binary encodings!
///
/// If rm = 1, all positions of solution are mutated.
#[derive(Serialize, Deserialize)]
pub struct BitflipMutation {
    /// Probability of mutating one position.
    pub rm: f64,
}
impl BitflipMutation {
    pub fn new<P>(rm: f64) -> Box<dyn Component<P>>
    where
        P: Problem<Encoding = Vec<bool>>,
    {
        Box::new(Generator(PositionMutator(Self { rm })))
    }
}
impl<P> Mutation<P, bool> for BitflipMutation
where
    P: Problem<Encoding = Vec<bool>>,
{
    fn mutation_func<'p>(
        &self,
        _problem: &'p P,
        state: &'p mut State,
    ) -> Box<dyn FnMut(&mut bool) + 'p> {
        let &Self { rm } = self;

        Box::new(move |x| {
            if state.random_mut().gen_bool(rm) {
                *x = !*x;
            }
        })
    }
}

/// Applies a swap mutation to n_swap elements depending on mutation probability.
///
/// For more than two elements: swap is performed circular.
///
/// If pm = 1, all positions of solution are mutated.
#[derive(Serialize, Deserialize)]
pub struct SwapMutation {
    /// Probability of mutation.
    pub pm: f64,
    /// Number of swaps.
    pub n_swap: usize,
}
impl SwapMutation {
    pub fn new<P, D: 'static>(pm: f64, n_swap: usize) -> Box<dyn Component<P>>
    where
        P: Problem<Encoding = Vec<D>>,
    {
        Box::new(Generator(Self { pm, n_swap }))
    }
}
impl<P, D: 'static> Generation<P> for SwapMutation
where
    P: Problem<Encoding = Vec<D>>,
{
    fn generate_population(
        &self,
        population: &mut Vec<Individual>,
        _problem: &P,
        state: &mut State,
    ) {
        assert!(self.n_swap > 1);
        let rng = state.random_mut();
        for solution in population.iter_mut() {
            if rng.gen_bool(self.pm) {
                let solution = solution.solution_mut::<P::Encoding>();
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
}

#[cfg(test)]
mod swap_mutation {
    use crate::problems::bmf::BenchmarkFunction;

    use super::*;

    #[test]
    fn all_mutated() {
        // let problem = BenchmarkFunction::sphere(3);
        // let comp = SwapMutation { pm: 1.0, n_swap: 2 };
        // let mut state = State::new_root();
        // let mut rng = Random::testing();
        // let mut parents = vec![vec![0.1, 0.2, 0.4, 0.5, 0.9], vec![0.2, 0.3, 0.6, 0.7, 0.8]];
        // let parents_length = parents.len();
        // let solution_length = vec![parents[0].len(), parents[1].len()];
        // let mut offspring = Vec::new();
        // comp.generate(&mut state, &problem, &mut rng, &mut parents, &mut offspring);
        // assert_eq!(offspring.len(), parents_length);
        // assert_eq!(
        //     vec![offspring[0].len(), offspring[1].len()],
        //     solution_length
        // );
    }
}

/// Applies a scramble mutation to the solution depending on mutation probability.
///
/// Shuffles the solution.
///
/// If pm = 1, the solution is mutated.
#[derive(Serialize, Deserialize)]
pub struct ScrambleMutation {
    /// Probability of mutating the solution.
    pub pm: f64,
}
impl ScrambleMutation {
    pub fn new<P, D: 'static>(pm: f64) -> Box<dyn Component<P>>
    where
        P: Problem<Encoding = Vec<D>>,
    {
        Box::new(Generator(Self { pm }))
    }
}
impl<P, D: 'static> Generation<P> for ScrambleMutation
where
    P: Problem<Encoding = Vec<D>>,
{
    fn generate_population(
        &self,
        population: &mut Vec<Individual>,
        _problem: &P,
        state: &mut State,
    ) {
        let rng = state.random_mut();
        for solution in population.iter_mut() {
            if rng.gen_bool(self.pm) {
                solution.solution_mut::<P::Encoding>().shuffle(rng);
            }
        }
    }
}

#[cfg(test)]
mod scramble_mutation {
    use crate::problems::bmf::BenchmarkFunction;

    use super::*;

    #[test]
    fn all_mutated() {
        // let problem = BenchmarkFunction::sphere(3);
        // let comp = ScrambleMutation { pm: 1.0 };
        // let mut state = State::new_root();
        // let mut rng = Random::testing();
        // let mut parents = vec![vec![0.1, 0.2, 0.4, 0.5, 0.9], vec![0.2, 0.3, 0.6, 0.7, 0.8]];
        // let parents_length = parents.len();
        // let solution_length = vec![parents[0].len(), parents[1].len()];
        // let mut offspring = Vec::new();
        // comp.generate(&mut state, &problem, &mut rng, &mut parents, &mut offspring);
        // assert_eq!(offspring.len(), parents_length);
        // assert_eq!(
        //     vec![offspring[0].len(), offspring[1].len()],
        //     solution_length
        // );
    }
}

/// Applies a insertion mutation to the solution depending on mutation probability.
///
/// Removes one random element of the solution and inserts it on a random position.
///
/// If pm = 1, the solution is mutated.
#[derive(Serialize, Deserialize)]
pub struct InsertionMutation {
    /// Probability of mutating the solution.
    pub pm: f64,
}
impl InsertionMutation {
    pub fn new<P, D: 'static>(pm: f64) -> Box<dyn Component<P>>
    where
        P: Problem<Encoding = Vec<D>>,
    {
        Box::new(Generator(Self { pm }))
    }
}
impl<P, D: 'static> Generation<P> for InsertionMutation
where
    P: Problem<Encoding = Vec<D>>,
{
    fn generate_population(
        &self,
        population: &mut Vec<Individual>,
        _problem: &P,
        state: &mut State,
    ) {
        let rng = state.random_mut();
        for solution in population.iter_mut() {
            if rng.gen_bool(self.pm) {
                let solution = solution.solution_mut::<P::Encoding>();
                let element = solution.remove(rng.gen_range(0..solution.len()));
                solution.insert(rng.gen_range(0..solution.len()), element);
            }
        }
    }
}

#[cfg(test)]
mod insertion_mutation {
    use crate::problems::bmf::BenchmarkFunction;

    use super::*;

    #[test]
    fn all_mutated() {
        // let problem = BenchmarkFunction::sphere(3);
        // let comp = InsertionMutation { pm: 1.0 };
        // let mut state = State::new_root();
        // let mut rng = Random::testing();
        // let mut parents = vec![vec![0.1, 0.2, 0.4, 0.5, 0.9], vec![0.2, 0.3, 0.6, 0.7, 0.8]];
        // let parents_length = parents.len();
        // let solution_length = vec![parents[0].len(), parents[1].len()];
        // let mut offspring = Vec::new();
        // comp.generate(&mut state, &problem, &mut rng, &mut parents, &mut offspring);
        // assert_eq!(offspring.len(), parents_length);
        // assert_eq!(
        //     vec![offspring[0].len(), offspring[1].len()],
        //     solution_length
        // );
    }
}

/// Applies a inversion mutation to the solution depending on mutation probability.
///
/// Takes a random slice of the solution and inverts it.
///
/// If pm = 1, the solution is mutated.
#[derive(Serialize, Deserialize)]
pub struct InversionMutation {
    /// Probability of mutating the solution.
    pub pm: f64,
}
impl InversionMutation {
    pub fn new<P, D: 'static>(pm: f64) -> Box<dyn Component<P>>
    where
        P: Problem<Encoding = Vec<D>>,
    {
        Box::new(Generator(Self { pm }))
    }
}
impl<P, D: 'static> Generation<P> for InversionMutation
where
    P: Problem<Encoding = Vec<D>>,
{
    fn generate_population(
        &self,
        population: &mut Vec<Individual>,
        _problem: &P,
        state: &mut State,
    ) {
        let rng = state.random_mut();
        for solution in population.iter_mut() {
            if rng.gen_bool(self.pm) {
                let solution = solution.solution_mut::<P::Encoding>();
                let dim = solution.len();
                let mut pos: Vec<usize> = (0..dim).collect();
                pos.shuffle(rng);
                pos.resize(2, 0);
                pos.sort_unstable();
                solution[pos[0]..pos[1] + 1].reverse();
            }
        }
    }
}

#[cfg(test)]
mod inversion_mutation {
    use crate::problems::bmf::BenchmarkFunction;

    use super::*;

    #[test]
    fn all_mutated() {
        // let problem = BenchmarkFunction::sphere(3);
        // let comp = InversionMutation { pm: 1.0 };
        // let mut state = State::new_root();
        // let mut rng = Random::testing();
        // let mut parents = vec![vec![0.1, 0.2, 0.4, 0.5, 0.9], vec![0.2, 0.3, 0.6, 0.7, 0.8]];
        // let parents_length = parents.len();
        // let solution_length = vec![parents[0].len(), parents[1].len()];
        // let mut offspring = Vec::new();
        // comp.generate(&mut state, &problem, &mut rng, &mut parents, &mut offspring);
        // assert_eq!(offspring.len(), parents_length);
        // assert_eq!(
        //     vec![offspring[0].len(), offspring[1].len()],
        //     solution_length
        // );
    }
}

/// Applies a translocation mutation to the solution depending on mutation probability.
///
/// Takes a random slice of the solution and inserts it at a new position.
///
/// If pm = 1, the solution is mutated.
#[derive(Serialize, Deserialize)]
pub struct TranslocationMutation {
    /// Probability of mutating the solution.
    pub pm: f64,
}
impl TranslocationMutation {
    pub fn new<P, D: 'static>(pm: f64) -> Box<dyn Component<P>>
    where
        P: Problem<Encoding = Vec<D>>,
        D: Clone,
    {
        Box::new(Generator(Self { pm }))
    }
}
impl<P, D: 'static> Generation<P> for TranslocationMutation
where
    P: Problem<Encoding = Vec<D>>,
    D: Clone,
{
    fn generate_population(
        &self,
        population: &mut Vec<Individual>,
        _problem: &P,
        state: &mut State,
    ) {
        let rng = state.random_mut();
        for solution in population.iter_mut() {
            if rng.gen_bool(self.pm) {
                let solution = solution.solution_mut::<P::Encoding>();
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
}

#[cfg(test)]
mod translocation_mutation {
    use crate::problems::bmf::BenchmarkFunction;

    use super::*;

    #[test]
    fn all_mutated() {
        // let problem = BenchmarkFunction::sphere(3);
        // let comp = TranslocationMutation { pm: 1.0 };
        // let mut state = State::new_root();
        // let mut rng = Random::testing();
        // let mut parents = vec![vec![0.1, 0.2, 0.4, 0.5, 0.9], vec![0.2, 0.3, 0.6, 0.7, 0.8]];
        // let parents_length = parents.len();
        // let solution_length = vec![parents[0].len(), parents[1].len()];
        // let mut offspring = Vec::new();
        // comp.generate(&mut state, &problem, &mut rng, &mut parents, &mut offspring);
        // assert_eq!(offspring.len(), parents_length);
        // assert_eq!(
        //     vec![offspring[0].len(), offspring[1].len()],
        //     solution_length
        // );
    }
}

/// Applies the PSO specific generation operator.
///
/// Requires PsoPostInitialization and PsoPostReplacement!
#[derive(serde::Serialize)]
pub struct PsoGeneration {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub v_max: f64,
}
impl PsoGeneration {
    pub fn new<P>(a: f64, b: f64, c: f64, v_max: f64) -> Box<dyn Component<P>>
    where
        P: Problem<Encoding = Vec<f64>>,
    {
        Box::new(Self { a, b, c, v_max })
    }
}
impl<P> Component<P> for PsoGeneration
where
    P: Problem<Encoding = Vec<f64>>,
{
    fn initialize(&self, _problem: &P, state: &mut State) {
        state.require::<PsoState>();
    }

    fn execute(&self, _problem: &P, state: &mut State) {
        let &Self { a, b, c, v_max } = self;

        let mut offspring = Vec::new();
        let mut parents = state.population_stack_mut().pop();

        let rng = state.random_mut();
        // let pso_state = pso_state.deref_mut();

        let rs = rng.gen_range(0.0..=1.0);
        let rt = rng.gen_range(0.0..=1.0);

        let pso_state = state.get_mut::<PsoState>();

        for (i, x) in parents.drain(..).enumerate() {
            let mut x = x.into_solution::<Vec<f64>>();
            let v = &mut pso_state.velocities[i];
            let xl = pso_state.bests[i].solution::<Vec<f64>>();
            let xg = pso_state.global_best.solution::<Vec<f64>>();

            for i in 0..v.len() {
                v[i] = a * v[i] + b * rs * (xg[i] - x[i]) + c * rt * (xl[i] - x[i]);
                v[i] = v[i].clamp(-v_max, v_max);
            }

            for i in 0..x.len() {
                x[i] = (x[i] + v[i]).clamp(-1.0, 1.0);
            }

            offspring.push(Individual::new_unevaluated(x));
        }

        state.population_stack_mut().push(offspring);
    }
}

// Recombination Operators //

/// Applies a n-point crossover to two parent solutions depending on crossover probability.
///
/// If pc = 1, the solutions are recombined.
#[derive(Serialize, Deserialize)]
pub struct NPointCrossover {
    /// Probability of recombining the solutions.
    pub pc: f64,
    pub points: usize,
}
impl NPointCrossover {
    pub fn new<P, D>(pc: f64, points: usize) -> Box<dyn Component<P>>
    where
        P: Problem<Encoding = Vec<D>>,
        D: Clone + PartialEq + 'static,
    {
        Box::new(Recombinator(Self { pc, points }))
    }
}
impl<P, D> Recombination<P, Vec<D>> for NPointCrossover
where
    P: Problem<Encoding = Vec<D>>,
    D: Clone,
{
    fn recombine_solutions(
        &self,
        parents: Vec<Vec<D>>,
        offspring: &mut Vec<Vec<D>>,
        state: &mut State,
    ) {
        let dim: usize = parents
            .iter()
            .min_by(|x, &y| (x.len()).cmp(&y.len()))
            .unwrap()
            .len();
        assert!(self.points < dim);
        let rng = state.random_mut();
        for pairs in parents.chunks(2) {
            if pairs.len() > 1 {
                let mut child1 = pairs[0].to_owned();
                let mut child2 = pairs[1].to_owned();
                if rng.gen::<f64>() <= self.pc {
                    let mut pos = (0..dim).choose_multiple(rng, self.points);
                    pos.sort_unstable();
                    for (i, &pt) in pos.iter().enumerate() {
                        if pairs[0].len() != pairs[1].len() {
                            if i < self.points - 1 {
                                child2[..pt].swap_with_slice(&mut child1[..pt]);
                            } else {
                                child1.truncate(pt);
                                child1.extend_from_slice(&pairs[1][pt..]);
                                child2.truncate(pt);
                                child2.extend_from_slice(&pairs[0][pt..]);
                            }
                        } else {
                            child2[pt..].swap_with_slice(&mut child1[pt..]);
                        }
                    }
                }
                offspring.push(child1);
                offspring.push(child2);
            } else {
                let child1 = pairs[0].to_owned();
                offspring.push(child1);
            }
        }
    }
}

#[cfg(test)]
mod npoint_crossover {
    use crate::problems::bmf::BenchmarkFunction;

    use super::*;

    #[test]
    fn all_recombined() {
        // let problem = BenchmarkFunction::sphere(3);
        // let comp = NPointCrossover { pc: 1.0, points: 3 };
        // let mut state = State::new_root();
        // let mut rng = Random::testing();
        // let mut parents = vec![
        //     vec![0.1, 0.2, 0.4, 0.5, 0.9],
        //     vec![0.2, 0.3, 0.6, 0.7, 0.8],
        //     vec![0.11, 0.21, 0.41, 0.51, 0.91],
        // ];
        // let parents_length = parents.len();
        // let mut offspring = Vec::new();
        // comp.generate(&mut state, &problem, &mut rng, &mut parents, &mut offspring);
        // assert_eq!(offspring.len(), parents_length);
    }
}

/// Applies a uniform crossover to two parent solutions depending on crossover probability.
///
/// If pc = 1, the solutions are recombined.
#[derive(Serialize, Deserialize)]
pub struct UniformCrossover {
    /// Probability of recombining the solutions.
    pub pc: f64,
}
impl UniformCrossover {
    pub fn new<P, D>(pc: f64) -> Box<dyn Component<P>>
    where
        P: Problem<Encoding = Vec<D>>,
        D: Clone + PartialEq + 'static,
    {
        Box::new(Recombinator(Self { pc }))
    }
}
impl<P, D> Recombination<P, Vec<D>> for UniformCrossover
where
    P: Problem<Encoding = Vec<D>>,
    D: Clone,
{
    fn recombine_solutions(
        &self,
        parents: Vec<Vec<D>>,
        offspring: &mut Vec<Vec<D>>,
        state: &mut State,
    ) {
        for pairs in parents.chunks(2) {
            if pairs.len() == 1 {
                let child1 = pairs[0].to_owned();
                offspring.push(child1);
                continue;
            }
            let mut child1 = Vec::new();
            let mut child2 = Vec::new();
            let rng = state.random_mut();
            if rng.gen::<f64>() <= self.pc {
                for i in 0..max(pairs[0].len(), pairs[1].len()) {
                    if i < pairs[0].len() && i < pairs[1].len() {
                        let (a, b) = if rng.gen_bool(0.5) { (0, 1) } else { (1, 0) };
                        child1.push(pairs[a][i].clone());
                        child2.push(pairs[b][i].clone());
                    } else if i >= pairs[0].len() {
                        child2.push(pairs[1][i].clone());
                    } else if i >= pairs[1].len() {
                        child1.push(pairs[0][i].clone());
                    }
                }
            } else {
                child1 = pairs[0].to_owned();
                child2 = pairs[1].to_owned();
            }
            offspring.push(child1);
            offspring.push(child2);
        }
    }
}

#[cfg(test)]
mod uniform_crossover {
    use crate::problems::bmf::BenchmarkFunction;

    use super::*;

    #[test]
    fn all_recombined() {
        // let problem = BenchmarkFunction::sphere(3);
        // let comp = UniformCrossover { pc: 1.0 };
        // let mut state = State::new_root();
        // let mut rng = Random::testing();
        // let mut parents = vec![
        //     vec![0.1, 0.2, 0.4, 0.5, 0.9],
        //     vec![0.2, 0.3, 0.6, 0.7, 0.8],
        //     vec![0.11, 0.21, 0.41, 0.51, 0.91],
        // ];
        // let parents_length = parents.len();
        // let mut offspring = Vec::new();
        // comp.generate(&mut state, &problem, &mut rng, &mut parents, &mut offspring);
        // assert_eq!(offspring.len(), parents_length);
    }
}

/// Applies a cycle crossover to two parent solutions depending on crossover probability.
///
/// Usually exclusive to combinatorial problems.
///
/// If pc = 1, the solutions are recombined.
#[derive(Serialize, Deserialize)]
pub struct CycleCrossover {
    /// Probability of recombining the solutions.
    pub pc: f64,
}
impl CycleCrossover {
    pub fn new<P, D>(pc: f64) -> Box<dyn Component<P>>
    where
        P: Problem<Encoding = Vec<D>>,
        D: Clone + PartialEq + 'static,
    {
        Box::new(Recombinator(Self { pc }))
    }
}
impl<P, D: Clone> Recombination<P, Vec<D>> for CycleCrossover
where
    P: Problem<Encoding = Vec<D>>,
    D: Clone + PartialEq,
{
    fn recombine_solutions(
        &self,
        parents: Vec<Vec<D>>,
        offspring: &mut Vec<Vec<D>>,
        state: &mut State,
    ) {
        for pairs in parents.chunks(2) {
            if pairs.len() == 1 {
                let child1 = pairs[0].to_owned();
                offspring.push(child1);
                continue;
            }

            let mut child1 = Vec::new();
            let mut child2 = Vec::new();
            let rng = state.random_mut();
            if rng.gen::<f64>() <= self.pc {
                let mut cycles = vec![-1; pairs[0].len()];
                let mut cycle_number = 1;
                let cycle_start: Vec<usize> = (0..cycles.len()).collect();

                for mut pos in cycle_start {
                    while cycles[pos] < 0 {
                        cycles[pos] = cycle_number;
                        pos = pairs[0].iter().position(|r| r == &pairs[1][pos]).unwrap();
                    }

                    cycle_number += 1;
                }

                for (i, n) in cycles.iter().enumerate() {
                    if n % 2 == 0 {
                        child1.push(pairs[0][i].clone());
                        child2.push(pairs[1][i].clone());
                    } else {
                        child1.push(pairs[1][i].clone());
                        child2.push(pairs[0][i].clone());
                    }
                }
            } else {
                child1 = pairs[0].to_owned();
                child2 = pairs[1].to_owned();
            }
            offspring.push(child1);
            offspring.push(child2);
        }
    }
}

#[cfg(test)]
mod cycle_crossover {
    use crate::problems::bmf::BenchmarkFunction;

    use super::*;

    #[test]
    fn all_recombined() {
        // let problem = BenchmarkFunction::sphere(3);
        // let comp = CycleCrossover { pc: 1.0 };
        // let mut state = State::new_root();
        // let mut rng = Random::testing();
        // let mut parents = vec![
        //     vec![8.0, 4.0, 7.0, 3.0, 6.0, 2.0, 5.0, 1.0, 9.0, 0.0],
        //     vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0],
        // ];
        // let parents_length = parents.len();
        // let mut offspring = Vec::new();
        // comp.generate(&mut state, &problem, &mut rng, &mut parents, &mut offspring);
        // assert_eq!(offspring.len(), parents_length);
    }
}
