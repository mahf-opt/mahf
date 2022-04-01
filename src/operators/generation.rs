//! Generation methods

use crate::operators::custom_state::PsoState;
use crate::{
    framework::{components::*, State},
    problems::{LimitedVectorProblem, Problem, VectorProblem},
    random::Random,
};
use rand::{prelude::SliceRandom, Rng};
use rand_distr::{uniform::SampleUniform, Distribution};
use serde::{Deserialize, Serialize};
use std::cmp::max;

#[derive(Serialize)]
pub struct Noop;
impl Noop {
    pub fn new<P: Problem>() -> Box<dyn Component<P>> {
        Box::new(Generator(Self))
    }
}
impl<P: Problem> Generation<P> for Noop {
    fn generate(
        &self,
        _state: &mut State,
        _problem: &P,
        _rng: &mut Random,
        _parents: &mut Vec<<P as Problem>::Encoding>,
        _offspring: &mut Vec<<P as Problem>::Encoding>,
    ) {
    }
}

// Mutation-like Operators //

/// Generates new random solutions in the search space.
#[derive(Serialize)]
pub struct RandomSpread;
impl RandomSpread {
    pub fn new<P, D>() -> Box<dyn Component<P>>
    where
        D: SampleUniform + PartialOrd,
        P: Problem<Encoding = Vec<D>> + LimitedVectorProblem<T = D>,
    {
        Box::new(Generator(Self))
    }
}
impl<P, D> Generation<P> for RandomSpread
where
    D: SampleUniform + PartialOrd,
    P: Problem<Encoding = Vec<D>> + LimitedVectorProblem<T = D>,
{
    fn generate(
        &self,
        _state: &mut State,
        problem: &P,
        rng: &mut Random,
        _parents: &mut Vec<Vec<D>>,
        offspring: &mut Vec<Vec<D>>,
    ) {
        let solution = (0..problem.dimension())
            .map(|d| rng.gen_range(problem.range(d)))
            .collect::<Vec<D>>();
        offspring.push(solution);
    }
}

/// Generates new random permutations.
#[derive(Serialize)]
pub struct RandomPermutation;
impl RandomPermutation {
    pub fn new<P>() -> Box<dyn Component<P>>
    where
        P: Problem<Encoding = Vec<usize>> + VectorProblem<T = usize>,
    {
        Box::new(Generator(Self))
    }
}
impl<P> Generation<P> for RandomPermutation
where
    P: Problem<Encoding = Vec<usize>> + VectorProblem<T = usize>,
{
    fn generate(
        &self,
        _state: &mut State,
        problem: &P,
        rng: &mut Random,
        _parents: &mut Vec<Vec<usize>>,
        offspring: &mut Vec<Vec<usize>>,
    ) {
        let mut solution = (0..problem.dimension()).collect::<Vec<usize>>();
        solution.shuffle(rng);
        offspring.push(solution);
    }
}

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
        Box::new(Generator(Self { deviation }))
    }
}
impl<P> Generation<P> for FixedDeviationDelta
where
    P: Problem<Encoding = Vec<f64>>,
{
    fn generate(
        &self,
        _state: &mut State,
        _problem: &P,
        rng: &mut Random,
        parents: &mut Vec<Vec<f64>>,
        offspring: &mut Vec<Vec<f64>>,
    ) {
        let distribution = rand_distr::Normal::new(0.0, self.deviation).unwrap();

        for solution in parents.iter_mut() {
            for x in solution {
                *x += distribution.sample(rng)
            }
        }

        offspring.append(parents)
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
    fn generate(
        &self,
        state: &mut State,
        _problem: &P,
        rng: &mut Random,
        parents: &mut Vec<Vec<f64>>,
        offspring: &mut Vec<Vec<f64>>,
    ) {
        let deviation = self.deviation(state.progress);
        let distribution = rand_distr::Normal::new(0.0, deviation).unwrap();

        for solution in parents.iter_mut() {
            for x in solution {
                *x += distribution.sample(rng)
            }
        }

        offspring.append(parents)
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
impl<P> Generation<P> for UniformMutation
where
    P: Problem<Encoding = Vec<f64>> + LimitedVectorProblem<T = f64>,
{
    fn generate(
        &self,
        _state: &mut State,
        problem: &P,
        rng: &mut Random,
        parents: &mut Vec<Vec<f64>>,
        offspring: &mut Vec<Vec<f64>>,
    ) {
        let dim = problem.dimension();
        for solution in parents.iter_mut() {
            for position in solution {
                if rng.gen::<f64>() <= self.rm {
                    *position = rng.gen_range(problem.range(dim));
                }
            }
        }
        offspring.append(parents);
    }
}

#[cfg(test)]
mod uniform_mutation {
    use super::*;
    use crate::problems::bmf::BenchmarkFunction;

    #[test]
    fn all_mutated() {
        let problem = BenchmarkFunction::sphere(3);
        let comp = UniformMutation { rm: 1.0 };
        let mut state = State::new();
        let mut rng = Random::testing();
        let mut parents = vec![vec![0.1, 0.2, 0.4], vec![0.2, 0.3, 0.6]];
        let parents_length = parents.len();
        let solution_length = vec![parents[0].len(), parents[1].len()];
        let mut offspring = Vec::new();
        comp.generate(&mut state, &problem, &mut rng, &mut parents, &mut offspring);
        assert_eq!(offspring.len(), parents_length);
        assert_eq!(
            vec![offspring[0].len(), offspring[1].len()],
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
#[derive(Serialize, Deserialize)]
pub struct GaussianMutation {
    /// Probability of mutating one position.
    pub rm: f64,
    /// Standard Deviation for the mutation.
    pub deviation: f64,
}
impl<P> Generation<P> for GaussianMutation
where
    P: Problem<Encoding = Vec<f64>>,
{
    fn generate(
        &self,
        _state: &mut State,
        _problem: &P,
        rng: &mut Random,
        parents: &mut Vec<Vec<f64>>,
        offspring: &mut Vec<Vec<f64>>,
    ) {
        let distribution = rand_distr::Normal::new(0.0, self.deviation).unwrap();

        for solution in parents.iter_mut() {
            for position in solution {
                if rng.gen::<f64>() <= self.rm {
                    *position += distribution.sample(rng);
                }
            }
        }
        offspring.append(parents);
    }
}

#[cfg(test)]
mod gaussian_mutation {
    use super::*;
    use crate::problems::bmf::BenchmarkFunction;

    #[test]
    fn all_mutated() {
        let problem = BenchmarkFunction::sphere(3);
        let comp = GaussianMutation {
            rm: 1.0,
            deviation: 0.1,
        };
        let mut state = State::new();
        let mut rng = Random::testing();
        let mut parents = vec![vec![0.1, 0.2, 0.4], vec![0.2, 0.3, 0.6]];
        let parents_length = parents.len();
        let solution_length = vec![parents[0].len(), parents[1].len()];
        let mut offspring = Vec::new();
        comp.generate(&mut state, &problem, &mut rng, &mut parents, &mut offspring);
        assert_eq!(offspring.len(), parents_length);
        assert_eq!(
            vec![offspring[0].len(), offspring[1].len()],
            solution_length
        );
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
impl<P> Generation<P> for BitflipMutation
where
    P: Problem<Encoding = Vec<bool>>,
{
    fn generate(
        &self,
        _state: &mut State,
        _problem: &P,
        rng: &mut Random,
        parents: &mut Vec<Vec<bool>>,
        offspring: &mut Vec<Vec<bool>>,
    ) {
        for solution in parents.iter_mut() {
            for position in solution {
                if rng.gen::<f64>() <= self.rm {
                    *position = !*position;
                }
            }
        }
        offspring.append(parents);
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
impl<P, D> Generation<P> for SwapMutation
where
    P: Problem<Encoding = Vec<D>>,
{
    fn generate(
        &self,
        _state: &mut State,
        _problem: &P,
        rng: &mut Random,
        parents: &mut Vec<Vec<D>>,
        offspring: &mut Vec<Vec<D>>,
    ) {
        assert!(self.n_swap > 1);
        for solution in parents.iter_mut() {
            if rng.gen::<f64>() <= self.pm {
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
        offspring.append(parents);
    }
}

#[cfg(test)]
mod swap_mutation {
    use super::*;
    use crate::problems::bmf::BenchmarkFunction;

    #[test]
    fn all_mutated() {
        let problem = BenchmarkFunction::sphere(3);
        let comp = SwapMutation { pm: 1.0, n_swap: 2 };
        let mut state = State::new();
        let mut rng = Random::testing();
        let mut parents = vec![vec![0.1, 0.2, 0.4, 0.5, 0.9], vec![0.2, 0.3, 0.6, 0.7, 0.8]];
        let parents_length = parents.len();
        let solution_length = vec![parents[0].len(), parents[1].len()];
        let mut offspring = Vec::new();
        comp.generate(&mut state, &problem, &mut rng, &mut parents, &mut offspring);
        assert_eq!(offspring.len(), parents_length);
        assert_eq!(
            vec![offspring[0].len(), offspring[1].len()],
            solution_length
        );
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
impl<P, D> Generation<P> for ScrambleMutation
where
    P: Problem<Encoding = Vec<D>>,
{
    fn generate(
        &self,
        _state: &mut State,
        _problem: &P,
        rng: &mut Random,
        parents: &mut Vec<Vec<D>>,
        offspring: &mut Vec<Vec<D>>,
    ) {
        if rng.gen::<f64>() <= self.pm {
            for solution in parents.iter_mut() {
                solution.shuffle(rng);
            }
        }
        offspring.append(parents);
    }
}

#[cfg(test)]
mod scramble_mutation {
    use super::*;
    use crate::problems::bmf::BenchmarkFunction;

    #[test]
    fn all_mutated() {
        let problem = BenchmarkFunction::sphere(3);
        let comp = ScrambleMutation { pm: 1.0 };
        let mut state = State::new();
        let mut rng = Random::testing();
        let mut parents = vec![vec![0.1, 0.2, 0.4, 0.5, 0.9], vec![0.2, 0.3, 0.6, 0.7, 0.8]];
        let parents_length = parents.len();
        let solution_length = vec![parents[0].len(), parents[1].len()];
        let mut offspring = Vec::new();
        comp.generate(&mut state, &problem, &mut rng, &mut parents, &mut offspring);
        assert_eq!(offspring.len(), parents_length);
        assert_eq!(
            vec![offspring[0].len(), offspring[1].len()],
            solution_length
        );
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
impl<P, D> Generation<P> for InsertionMutation
where
    P: Problem<Encoding = Vec<D>>,
{
    fn generate(
        &self,
        _state: &mut State,
        _problem: &P,
        rng: &mut Random,
        parents: &mut Vec<Vec<D>>,
        offspring: &mut Vec<Vec<D>>,
    ) {
        if rng.gen::<f64>() <= self.pm {
            for solution in parents.iter_mut() {
                let element = solution.remove(rng.gen_range(0..solution.len()));
                solution.insert(rng.gen_range(0..solution.len()), element);
            }
        }
        offspring.append(parents);
    }
}

#[cfg(test)]
mod insertion_mutation {
    use super::*;
    use crate::problems::bmf::BenchmarkFunction;

    #[test]
    fn all_mutated() {
        let problem = BenchmarkFunction::sphere(3);
        let comp = InsertionMutation { pm: 1.0 };
        let mut state = State::new();
        let mut rng = Random::testing();
        let mut parents = vec![vec![0.1, 0.2, 0.4, 0.5, 0.9], vec![0.2, 0.3, 0.6, 0.7, 0.8]];
        let parents_length = parents.len();
        let solution_length = vec![parents[0].len(), parents[1].len()];
        let mut offspring = Vec::new();
        comp.generate(&mut state, &problem, &mut rng, &mut parents, &mut offspring);
        assert_eq!(offspring.len(), parents_length);
        assert_eq!(
            vec![offspring[0].len(), offspring[1].len()],
            solution_length
        );
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
impl<P, D> Generation<P> for InversionMutation
where
    P: Problem<Encoding = Vec<D>>,
{
    fn generate(
        &self,
        _state: &mut State,
        _problem: &P,
        rng: &mut Random,
        parents: &mut Vec<Vec<D>>,
        offspring: &mut Vec<Vec<D>>,
    ) {
        if rng.gen::<f64>() <= self.pm {
            for solution in parents.iter_mut() {
                let dim = solution.len();
                let mut pos: Vec<usize> = (0..dim).collect();
                pos.shuffle(rng);
                pos.resize(2, 0);
                pos.sort_unstable();
                solution[pos[0]..pos[1] + 1].reverse();
            }
        }
        offspring.append(parents);
    }
}

#[cfg(test)]
mod inversion_mutation {
    use super::*;
    use crate::problems::bmf::BenchmarkFunction;

    #[test]
    fn all_mutated() {
        let problem = BenchmarkFunction::sphere(3);
        let comp = InversionMutation { pm: 1.0 };
        let mut state = State::new();
        let mut rng = Random::testing();
        let mut parents = vec![vec![0.1, 0.2, 0.4, 0.5, 0.9], vec![0.2, 0.3, 0.6, 0.7, 0.8]];
        let parents_length = parents.len();
        let solution_length = vec![parents[0].len(), parents[1].len()];
        let mut offspring = Vec::new();
        comp.generate(&mut state, &problem, &mut rng, &mut parents, &mut offspring);
        assert_eq!(offspring.len(), parents_length);
        assert_eq!(
            vec![offspring[0].len(), offspring[1].len()],
            solution_length
        );
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
impl<P, D> Generation<P> for TranslocationMutation
where
    P: Problem<Encoding = Vec<D>>,
    D: std::clone::Clone,
{
    fn generate(
        &self,
        _state: &mut State,
        _problem: &P,
        rng: &mut Random,
        parents: &mut Vec<Vec<D>>,
        offspring: &mut Vec<Vec<D>>,
    ) {
        if rng.gen::<f64>() <= self.pm {
            for solution in parents.iter_mut() {
                let dim = solution.len();
                let mut pos: Vec<usize> = (0..dim).collect();
                pos.shuffle(rng);
                pos.resize(2, 0);
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
                offspring.push(start);
            }
        }
    }
}

#[cfg(test)]
mod translocation_mutation {
    use super::*;
    use crate::problems::bmf::BenchmarkFunction;

    #[test]
    fn all_mutated() {
        let problem = BenchmarkFunction::sphere(3);
        let comp = TranslocationMutation { pm: 1.0 };
        let mut state = State::new();
        let mut rng = Random::testing();
        let mut parents = vec![vec![0.1, 0.2, 0.4, 0.5, 0.9], vec![0.2, 0.3, 0.6, 0.7, 0.8]];
        let parents_length = parents.len();
        let solution_length = vec![parents[0].len(), parents[1].len()];
        let mut offspring = Vec::new();
        comp.generate(&mut state, &problem, &mut rng, &mut parents, &mut offspring);
        assert_eq!(offspring.len(), parents_length);
        assert_eq!(
            vec![offspring[0].len(), offspring[1].len()],
            solution_length
        );
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
        Box::new(Generator(Self { a, b, c, v_max }))
    }
}
impl<P> Generation<P> for PsoGeneration
where
    P: Problem<Encoding = Vec<f64>>,
{
    fn generate(
        &self,
        state: &mut State,
        _problem: &P,
        rng: &mut Random,
        parents: &mut Vec<P::Encoding>,
        offspring: &mut Vec<P::Encoding>,
    ) {
        let &PsoGeneration { a, b, c, v_max } = self;
        let pso_state = state.custom.get_mut::<PsoState>();
        let rs = rng.gen_range(0.0..=1.0);
        let rt = rng.gen_range(0.0..=1.0);

        for (i, mut x) in parents.drain(..).enumerate() {
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

            offspring.push(x);
        }
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
impl<P, D> Generation<P> for NPointCrossover
where
    P: Problem<Encoding = Vec<D>>,
    D: std::clone::Clone,
{
    fn generate(
        &self,
        _state: &mut State,
        _problem: &P,
        rng: &mut Random,
        parents: &mut Vec<Vec<D>>,
        offspring: &mut Vec<Vec<D>>,
    ) {
        let dim: usize = parents
            .iter()
            .min_by(|x, &y| (x.len()).cmp(&y.len()))
            .unwrap()
            .len();
        assert!(self.points < dim);
        for pairs in parents.chunks(2) {
            if pairs.len() > 1 {
                let mut child1 = pairs[0].to_owned();
                let mut child2 = pairs[1].to_owned();
                if rng.gen::<f64>() <= self.pc {
                    let mut pos: Vec<usize> = (0..dim).collect();
                    pos.shuffle(rng);
                    pos.resize(self.points, 0);
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
    use super::*;
    use crate::problems::bmf::BenchmarkFunction;

    #[test]
    fn all_recombined() {
        let problem = BenchmarkFunction::sphere(3);
        let comp = NPointCrossover { pc: 1.0, points: 3 };
        let mut state = State::new();
        let mut rng = Random::testing();
        let mut parents = vec![
            vec![0.1, 0.2, 0.4, 0.5, 0.9],
            vec![0.2, 0.3, 0.6, 0.7, 0.8],
            vec![0.11, 0.21, 0.41, 0.51, 0.91],
        ];
        let parents_length = parents.len();
        let mut offspring = Vec::new();
        comp.generate(&mut state, &problem, &mut rng, &mut parents, &mut offspring);
        assert_eq!(offspring.len(), parents_length);
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
        D: std::clone::Clone,
    {
        Box::new(Generator(Self { pc }))
    }
}
impl<P, D> Generation<P> for UniformCrossover
where
    P: Problem<Encoding = Vec<D>>,
    D: std::clone::Clone,
{
    fn generate(
        &self,
        _state: &mut State,
        _problem: &P,
        rng: &mut Random,
        parents: &mut Vec<Vec<D>>,
        offspring: &mut Vec<Vec<D>>,
    ) {
        for pairs in parents.chunks(2) {
            if pairs.len() == 1 {
                let child1 = pairs[0].to_owned();
                offspring.push(child1);
                continue;
            }

            let mut child1 = Vec::new();
            let mut child2 = Vec::new();
            if rng.gen::<f64>() <= self.pc {
                for i in 0..max(pairs[0].len(), pairs[1].len()) {
                    if i < pairs[0].len() && i < pairs[1].len() {
                        let r = rng.gen::<f64>() < 0.5;
                        let (a, b) = if r { (0, 1) } else { (1, 0) };
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
    use super::*;
    use crate::problems::bmf::BenchmarkFunction;

    #[test]
    fn all_recombined() {
        let problem = BenchmarkFunction::sphere(3);
        let comp = UniformCrossover { pc: 1.0 };
        let mut state = State::new();
        let mut rng = Random::testing();
        let mut parents = vec![
            vec![0.1, 0.2, 0.4, 0.5, 0.9],
            vec![0.2, 0.3, 0.6, 0.7, 0.8],
            vec![0.11, 0.21, 0.41, 0.51, 0.91],
        ];
        let parents_length = parents.len();
        let mut offspring = Vec::new();
        comp.generate(&mut state, &problem, &mut rng, &mut parents, &mut offspring);
        assert_eq!(offspring.len(), parents_length);
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
impl<P, D: Clone> Generation<P> for CycleCrossover
where
    P: Problem<Encoding = Vec<D>>,
    D: std::clone::Clone + std::cmp::PartialEq,
{
    fn generate(
        &self,
        _state: &mut State,
        _problem: &P,
        rng: &mut Random,
        parents: &mut Vec<Vec<D>>,
        offspring: &mut Vec<Vec<D>>,
    ) {
        for pairs in parents.chunks(2) {
            if pairs.len() == 1 {
                let child1 = pairs[0].to_owned();
                offspring.push(child1);
                continue;
            }

            let mut child1 = Vec::new();
            let mut child2 = Vec::new();

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
    use super::*;
    use crate::problems::bmf::BenchmarkFunction;

    #[test]
    fn all_recombined() {
        let problem = BenchmarkFunction::sphere(3);
        let comp = CycleCrossover { pc: 1.0 };
        let mut state = State::new();
        let mut rng = Random::testing();
        let mut parents = vec![
            vec![8.0, 4.0, 7.0, 3.0, 6.0, 2.0, 5.0, 1.0, 9.0, 0.0],
            vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0],
        ];
        let parents_length = parents.len();
        let mut offspring = Vec::new();
        comp.generate(&mut state, &problem, &mut rng, &mut parents, &mut offspring);
        assert_eq!(offspring.len(), parents_length);
    }
}
