//! Replacement components for the Black Hole algorithm (BH).

use rand::Rng;
use serde::{Deserialize, Serialize};
use crate::{Component, ExecResult, Individual, SingleObjectiveProblem, State};
use crate::problems::LimitedVectorProblem;
use crate::utils::squared_euclidean;

#[derive(Clone, Serialize, Deserialize)]
pub struct EventHorizon;

impl EventHorizon {
    pub fn new<P: SingleObjectiveProblem + LimitedVectorProblem>() -> Box<dyn Component<P>>
        where
            P: LimitedVectorProblem<Element = f64>, {
        Box::new(Self)
    }
}

impl<P> Component<P> for EventHorizon
    where
        P: LimitedVectorProblem<Element = f64>,
{

    fn execute(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        let mut populations = state.populations_mut();
        let mut offspring = populations.pop();

        let f_bh = state.best_objective_value().unwrap().value();

        let fitness_sum = offspring.iter().map(|x| *x.objective().value()).sum::<f64>();
        let radius = f_bh / fitness_sum;

        let rng = &mut state.random_mut();

        let best = state.best_individual().solution();
        let distances = offspring.iter().map(|o| squared_euclidean(*o.as_solution(), best).sqrt()).collect::<Vec<f64>>();

        for (u, mut i) in offspring.iter().enumerate() {
            if distances[u] < radius {
                let rand: Vec<f64> = (0..problem.dimension()).map(|_| rng.gen_range(problem.domain()[0].clone())).collect();
                let j = Individual::new_unevaluated(rand);
                i = &j;
            }
        }
        populations.push(offspring);
        Ok(())
    }
}