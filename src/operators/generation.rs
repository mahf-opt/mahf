//! Generation methods

use crate::{
    heuristic::{components::*, State},
    problem::Problem,
    random::Random,
};
use rand_distr::Distribution;
use serde::{Deserialize, Serialize};

/// Applies a fixed, component wise delta from a normal distribution.
///
/// Uses a `N(0, deviation)` normal distribution.
#[derive(Serialize, Deserialize)]
pub struct Fixed {
    /// Standard Deviation for the mutation.
    pub deviation: f64,
}
impl<P> Generation<P> for Fixed
where
    P: Problem<Encoding = Vec<f64>>,
{
    fn generate(
        &self,
        _state: &mut State,
        _problem: &P,
        rng: &mut Random,
        parents: &mut Vec<&Vec<f64>>,
        offspring: &mut Vec<Vec<f64>>,
    ) {
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
#[derive(Serialize, Deserialize)]
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
impl Adaptive {
    fn deviation(&self, progress: f64) -> f64 {
        self.final_deviation
            + (1.0 - progress).powi(self.modulation_index as i32)
                * (self.initial_deviation - self.final_deviation)
    }
}
impl<P> Generation<P> for Adaptive
where
    P: Problem<Encoding = Vec<f64>>,
{
    fn generate(
        &self,
        state: &mut State,
        _problem: &P,
        rng: &mut Random,
        parents: &mut Vec<&Vec<f64>>,
        offspring: &mut Vec<Vec<f64>>,
    ) {
        let deviation = self.deviation(state.progress);
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
#[cfg(test)]
mod adaptive {
    use super::*;

    #[test]
    fn deviation_is_falling() {
        let comp = Adaptive {
            initial_deviation: 10.0,
            final_deviation: 1.0,
            modulation_index: 1,
        };
        float_eq::assert_float_eq!(comp.deviation(0.0), 10.0, ulps <= 1);
        float_eq::assert_float_eq!(comp.deviation(0.5), 5.5, ulps <= 1);
        float_eq::assert_float_eq!(comp.deviation(1.0), 1.0, ulps <= 1);
    }
}
