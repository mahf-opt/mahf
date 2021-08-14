//! Generation methods

use crate::{
    heuristic::{components::*, State},
    problem::{LimitedVectorProblem, Problem, VectorProblem},
    random::Random,
};
use rand::{prelude::SliceRandom, Rng};
use rand_distr::{uniform::SampleUniform, Distribution};
use serde::{Deserialize, Serialize};

/// Generates new random solutions in the search space.
#[derive(Serialize)]
pub struct RandomSpread;
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
#[derive(Serialize, Deserialize)]
pub struct FixedDeviationDelta {
    /// Standard Deviation for the mutation.
    pub deviation: f64,
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

/// Applies an adaptive, component wise delta from a normal distribution.
///
/// The actual deviation gets computed as follows:
/// ```math
/// final_deviation + (1 - progress)^modulation * (initial_deviation - final_deviation)
/// ```
#[derive(Serialize, Deserialize)]
pub struct AdaptiveDeviationDelta {
    /// Initial standard deviation for the mutation
    pub initial_deviation: f64,
    /// Final standard deviation for the mutation
    ///
    /// Must not be larger than `initial_deviation`.
    pub final_deviation: f64,
    /// Modulation index for the standard deviation.
    pub modulation_index: u32,
}
impl AdaptiveDeviationDelta {
    fn deviation(&self, progress: f64) -> f64 {
        self.final_deviation
            + (1.0 - progress).powi(self.modulation_index as i32)
                * (self.initial_deviation - self.final_deviation)
    }
}
impl<P> Generation<P> for AdaptiveDeviationDelta
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
        let comp = AdaptiveDeviationDelta {
            initial_deviation: 10.0,
            final_deviation: 1.0,
            modulation_index: 1,
        };
        float_eq::assert_float_eq!(comp.deviation(0.0), 10.0, ulps <= 1);
        float_eq::assert_float_eq!(comp.deviation(0.5), 5.5, ulps <= 1);
        float_eq::assert_float_eq!(comp.deviation(1.0), 1.0, ulps <= 1);
    }
}
