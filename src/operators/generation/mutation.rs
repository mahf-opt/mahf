//! Mutation-like Operators

use rand::{prelude::SliceRandom, seq::IteratorRandom, Rng};
use rand_distr::Distribution;
use serde::{Deserialize, Serialize};

use crate::{
    framework::{
        common_state::Progress,
        components::*,
        specializations::{Generation, Generator},
        State,
    },
    problems::{LimitedVectorProblem, Problem},
};

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
    fn generate_population(
        &self,
        population: &mut Vec<P::Encoding>,
        _problem: &P,
        state: &mut State,
    ) {
        let distribution = rand_distr::Normal::new(0.0, self.deviation).unwrap();

        for solution in population.iter_mut() {
            for x in solution.iter_mut() {
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
    fn generate_population(
        &self,
        population: &mut Vec<P::Encoding>,
        _problem: &P,
        state: &mut State,
    ) {
        let deviation = self.deviation(state.get_value::<Progress>());
        let distribution = rand_distr::Normal::new(0.0, deviation).unwrap();

        for solution in population.iter_mut() {
            for x in solution.iter_mut() {
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
        state: &mut State,
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
    use crate::random::Random;

    use super::*;

    #[test]
    fn all_mutated() {
        let problem = BenchmarkFunction::sphere(3);
        let comp = UniformMutation { rm: 1.0 };
        let mut state = State::new_root();
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
        state: &mut State,
    ) {
        let distribution = rand_distr::Normal::new(0.0, self.deviation).unwrap();
        let rng = state.random_mut();

        for solution in population.iter_mut() {
            for x in solution.iter_mut() {
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
    use crate::random::Random;

    use super::*;

    #[test]
    fn all_mutated() {
        let problem = BenchmarkFunction::sphere(3);
        let comp = GaussianMutation {
            rm: 1.0,
            deviation: 0.1,
        };
        let mut state = State::new_root();
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
        state: &mut State,
    ) {
        let rng = state.random_mut();

        for solution in population.iter_mut() {
            for x in solution.iter_mut() {
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
        population: &mut Vec<P::Encoding>,
        _problem: &P,
        state: &mut State,
    ) {
        assert!(self.n_swap > 1);
        let rng = state.random_mut();

        for solution in population.iter_mut() {
            if rng.gen_bool(self.pm) {
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
    use crate::random::Random;

    use super::*;

    #[test]
    fn all_mutated() {
        let problem = BenchmarkFunction::sphere(3);
        let comp = SwapMutation { pm: 1.0, n_swap: 2 };
        let mut state = State::new_root();
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
        population: &mut Vec<P::Encoding>,
        _problem: &P,
        state: &mut State,
    ) {
        let rng = state.random_mut();

        for solution in population.iter_mut() {
            if rng.gen_bool(self.pm) {
                solution.shuffle(rng);
            }
        }
    }
}

#[cfg(test)]
mod scramble_mutation {
    use crate::problems::bmf::BenchmarkFunction;
    use crate::random::Random;

    use super::*;

    #[test]
    fn all_mutated() {
        let problem = BenchmarkFunction::sphere(3);
        let comp = ScrambleMutation { pm: 1.0 };
        let mut state = State::new_root();
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
        population: &mut Vec<P::Encoding>,
        _problem: &P,
        state: &mut State,
    ) {
        let rng = state.random_mut();

        for solution in population.iter_mut() {
            if rng.gen_bool(self.pm) {
                let element = solution.remove(rng.gen_range(0..solution.len()));
                solution.insert(rng.gen_range(0..solution.len()), element);
            }
        }
    }
}

#[cfg(test)]
mod insertion_mutation {
    use crate::problems::bmf::BenchmarkFunction;
    use crate::random::Random;

    use super::*;

    #[test]
    fn all_mutated() {
        let problem = BenchmarkFunction::sphere(3);
        let comp = InsertionMutation { pm: 1.0 };
        let mut state = State::new_root();
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
        population: &mut Vec<P::Encoding>,
        _problem: &P,
        state: &mut State,
    ) {
        let rng = state.random_mut();
        for solution in population.iter_mut() {
            if rng.gen_bool(self.pm) {
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
    use crate::random::Random;

    use super::*;

    #[test]
    fn all_mutated() {
        let problem = BenchmarkFunction::sphere(3);
        let comp = InversionMutation { pm: 1.0 };
        let mut state = State::new_root();
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
        population: &mut Vec<P::Encoding>,
        _problem: &P,
        state: &mut State,
    ) {
        let rng = state.random_mut();
        for solution in population.iter_mut() {
            if rng.gen_bool(self.pm) {
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
    use crate::random::Random;

    use super::*;

    #[test]
    fn all_mutated() {
        let problem = BenchmarkFunction::sphere(3);
        let comp = TranslocationMutation { pm: 1.0 };
        let mut state = State::new_root();
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
