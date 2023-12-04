//! Replacement components for the Black Hole algorithm (BH).

use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::{
    problems::LimitedVectorProblem, utils::squared_euclidean,
    Component, ExecResult, SingleObjectiveProblem, State,
};

#[derive(Clone, Serialize, Deserialize)]
pub struct EventHorizon;

impl EventHorizon {
    pub fn new<P>() -> Box<dyn Component<P>>
    where
        P: LimitedVectorProblem<Element = f64>,
        P: SingleObjectiveProblem,
    {
        Box::new(Self)
    }
}

impl<P> Component<P> for EventHorizon
where
    P: LimitedVectorProblem<Element = f64>,
    P: SingleObjectiveProblem,
{
    fn execute(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        let mut populations = state.populations_mut();
        let mut offspring = populations.pop();

        let f_bh = state.best_objective_value().unwrap().value();

        let fitness_sum = offspring.iter().map(|x| x.objective().value()).sum::<f64>();
        let radius = f_bh / fitness_sum;

        let (index, best) = offspring
            .iter()
            .enumerate()
            .min_by_key(|(_u, i)| i.objective())
            .unwrap();
        let distances = offspring
            .iter()
            .map(|o| squared_euclidean(o.solution(), best.solution()).sqrt())
            .collect::<Vec<f64>>();

        for (u, i) in offspring.iter_mut().enumerate() {
            // do not replace best individual
            if distances[u] < radius && u != index {
                let rand: Vec<f64> = problem.domain().iter()
                    .map(|d| state.random_mut().gen_range(d.clone()))
                    .collect();
                *i.solution_mut() = rand;
            }
        }
        populations.push(offspring);
        Ok(())
    }
}
