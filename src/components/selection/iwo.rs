use eyre::{ensure, ContextCompat};
use serde::{Deserialize, Serialize};

use crate::{
    component::ExecResult,
    components::{
        selection::{functional, selection, Selection},
        Component,
    },
    problems::SingleObjectiveProblem,
    state::random::Random,
    Individual, State,
};

#[derive(Clone, Serialize, Deserialize)]
pub struct DeterministicFitnessProportional {
    pub min_selected: u32,
    pub max_selected: u32,
}

impl DeterministicFitnessProportional {
    pub fn from_params(min_selected: u32, max_selected: u32) -> Self {
        Self {
            min_selected,
            max_selected,
        }
    }

    pub fn new<P: SingleObjectiveProblem>(
        min_selected: u32,
        max_selected: u32,
    ) -> Box<dyn Component<P>> {
        Box::new(Self::from_params(min_selected, max_selected))
    }
}

impl<P: SingleObjectiveProblem> Selection<P> for DeterministicFitnessProportional {
    fn select<'a>(
        &self,
        population: &'a [Individual<P>],
        _rng: &mut Random,
    ) -> ExecResult<Vec<&'a Individual<P>>> {
        let (worst, best) =
            functional::objective_bounds(population).wrap_err("population is empty")?;
        ensure!(
            worst.is_finite(),
            "this selection operator does not work with Inf fitness values"
        );
        let mut selection = Vec::new();
        for ind in population.iter() {
            let bonus = (ind.objective().value() - worst) / (best - worst);
            let bonus_offspring = (self.max_selected - self.min_selected) as f64;
            let num_offspring = self.min_selected
                + if bonus.is_nan() {
                    // best ≈ worst, thus we picked a generic value
                    (0.5 * bonus_offspring).floor() as u32
                } else {
                    (bonus * bonus_offspring).floor() as u32
                };

            for _ in 0..num_offspring {
                selection.push(ind);
            }
        }
        Ok(selection)
    }
}

impl<P: SingleObjectiveProblem> Component<P> for DeterministicFitnessProportional {
    fn execute(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        selection(self, problem, state)
    }
}
