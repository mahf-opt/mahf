//! Postprocess variants
//!

use crate::{
    framework::{
        components::*,
        state::{common::Population, State},
    },
    operators::custom_state::DiversityState,
    problems::{Problem, VectorProblem},
};

/// Postprocess procedure for tracking population diversity
///
/// Currently only for VectorProblem
///
/// Measures can be chosen between dimension-wise (DW), mean of pairwise distance between solutions (PW),
/// average standard deviation of each position (also "true diversity", TD), and distance to average point (DTAP).
/// All measures are normalized with the maximum diversity found so far.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub enum DiversityMeasure {
    DW,
    PW,
    TD,
    DTAP,
}

#[derive(Debug, serde::Serialize)]
pub struct FloatVectorDiversity {
    pub measure: DiversityMeasure,
}

impl<P> Component<P> for FloatVectorDiversity
where
    P: Problem<Encoding = Vec<f64>> + VectorProblem<T = f64>,
{
    fn initialize(&self, _problem: &P, state: &mut State) {
        state.insert(DiversityState {
            diversity: 0.0,
            max_div: 0.0,
        });
    }

    fn execute(&self, problem: &P, state: &mut State) {
        let (population_stack, diversity_state) =
            state.get_multiple_mut::<(Population, DiversityState)>();

        let population = population_stack.current();

        if population.is_empty() {
            diversity_state.diversity = 0.0;
            return;
        }

        let n = population.len() as f64;
        let d = problem.dimension();
        let iter_solutions = || population.iter().map(|i| i.solution::<Vec<f64>>());

        let selected_measure = self.measure;
        match selected_measure {
            DiversityMeasure::DW => {
                diversity_state.diversity = (0..d)
                    .into_iter()
                    .map(|k| {
                        let xk = iter_solutions().map(|s| s[k]).sum::<f64>() / n;
                        iter_solutions().map(|s| (s[k] - xk).abs()).sum::<f64>() / n
                    })
                    .sum::<f64>()
                    / (d as f64)
            }
            DiversityMeasure::PW => {
                let mut sum = 0.0;
                let solutions: Vec<Vec<f64>> = iter_solutions().cloned().collect();
                for i in 1..n as usize {
                    for j in 0..=i - 1 {
                        sum += (0..d)
                            .into_iter()
                            .map(|k| (solutions[i][k] - solutions[j][k]).powi(2))
                            .sum::<f64>();
                        diversity_state.diversity += sum.sqrt();
                    }
                }
                diversity_state.diversity = diversity_state.diversity * 2.0 / (n * (n - 1.0));
            }
            DiversityMeasure::TD => {
                diversity_state.diversity = (0..d)
                    .into_iter()
                    .map(|k| {
                        let xk = iter_solutions().map(|s| s[k]).sum::<f64>() / n;
                        let sum = iter_solutions().map(|i| i[k].powi(2)).sum::<f64>() / n;
                        sum - xk.powi(2)
                    })
                    .sum::<f64>()
                    .sqrt()
                    / (d as f64)
            }
            DiversityMeasure::DTAP => {
                let mut sum = 0.0;
                for i in iter_solutions() {
                    sum += (0..d)
                        .into_iter()
                        .map(|k| {
                            let xk = iter_solutions().map(|s| s[k]).sum::<f64>() / n;
                            (i[k] - xk).powi(2)
                        })
                        .sum::<f64>()
                        .sqrt();
                }
                diversity_state.diversity = sum / n;
            }
        }

        // set new maximum diversity found so far
        if diversity_state.diversity > diversity_state.max_div {
            diversity_state.max_div = diversity_state.diversity
        }

        // normalize by division with maximum diversity
        diversity_state.diversity /= diversity_state.max_div;
    }
}
