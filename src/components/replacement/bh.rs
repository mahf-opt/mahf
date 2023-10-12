//! Replacement components for the Black Hole algorithm (BH).

use crate::population::BestIndividual;
use crate::problems::LimitedVectorProblem;
use crate::utils::squared_euclidean;
use crate::{Component, ExecResult, Individual, SingleObjectiveProblem, State};
use rand::Rng;
use serde::{Deserialize, Serialize};

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
        let offspring = populations.pop();

        let f_bh = state.best_objective_value().unwrap().value();

        let fitness_sum = offspring.iter().map(|x| x.objective().value()).sum::<f64>();
        let radius = f_bh / fitness_sum;

        let best_ind = offspring.best_individual().cloned();
        let best = best_ind.unwrap().solution().clone();
        let distances = offspring
            .iter()
            .map(|o| squared_euclidean(o.solution(), &best).sqrt())
            .collect::<Vec<f64>>();

        let mut new_offspring: Vec<Individual<P>> = vec![];
        for (u, i) in offspring.iter().enumerate() {
            // do not replace best individual
            if distances[u] < radius && distances[u] != 0.0 {
                let rand: Vec<f64> = (0..problem.dimension())
                    .map(|_| state.random_mut().gen_range(problem.domain()[0].clone()))
                    .collect();
                let j = Individual::new_unevaluated(rand);
                //println!("{:?}, {:?}", u, &j);
                new_offspring.push(j);
            } else {
                new_offspring.push(i.clone());
            }
        }
        populations.push(new_offspring);
        Ok(())
    }
}
