//! Generation methods

use crate::{
    heuristic::{components::*, State},
    problem::Problem,
};
use rand_distr::Distribution;
use serde::Serialize;

/// Applies a fixed, component wise delta from a normal distribution.
///
/// Uses a `N(0, deviation)` normal distribution.
#[derive(Serialize)]
pub struct Fixed {
    /// Standard Deviation for the mutation.
    pub deviation: f64,
}
impl<P> Generation<P> for Fixed
where
    P: Problem<Encoding = Vec<f64>>,
{
    fn generate(
        &mut self,
        _state: &mut State,
        _problem: &P,
        parents: &mut Vec<&Vec<f64>>,
        offspring: &mut Vec<Vec<f64>>,
    ) {
        let rng = &mut rand::thread_rng();
        let distribution = rand_distr::Normal::new(0.0, self.deviation).unwrap();

        for parent in parents {
            let solution = parent
                .iter()
                .map(|x| x + distribution.sample(rng))
                // TODO: How should clamping work?
                //.map(|x| x.clamp(*problem.range.start(), *problem.range.end()))
                .collect::<Vec<f64>>();
            offspring.push(solution);
        }
    }
}

/// Applies an adaptive, component wise delta from a normal distribution.
///
/// The actual deviation gets computed as follows:
/// ```math
/// final_deviation + (1 - progress)^modulation * (initial_deviation - final_deviation)
/// ```
#[derive(Serialize)]
pub struct Adaptive {
    /// Initial standard deviation for the mutation
    pub initial_deviation: f64,
    /// Final standard deviation for the mutation
    ///
    /// Must not be larger than `initial_deviation`.
    pub final_deviation: f64,
    /// Modulation index for the standard deviation.
    pub modulation_index: u32,
}
impl<P> Generation<P> for Adaptive
where
    P: Problem<Encoding = Vec<f64>>,
{
    fn generate(
        &mut self,
        state: &mut State,
        _problem: &P,
        parents: &mut Vec<&Vec<f64>>,
        offspring: &mut Vec<Vec<f64>>,
    ) {
        let rng = &mut rand::thread_rng();

        let deviation = self.final_deviation
            + (1.0 - state.progress).powi(self.modulation_index as i32)
                * (self.initial_deviation - self.final_deviation);
        let distribution = rand_distr::Normal::new(0.0, deviation).unwrap();

        for parent in parents {
            let solution = parent
                .iter()
                .map(|x| x + distribution.sample(rng))
                // TODO: Clamping
                //.map(|x| x.clamp(*problem.range.start(), *problem.range.end()))
                .collect::<Vec<f64>>();
            offspring.push(solution);
        }
    }
}
