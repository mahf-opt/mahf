//! Recombination components for Differential Evolution (DE).

use itertools::multizip;
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::{
    component::ExecResult,
    components::Component,
    population::{AsSolutions, AsSolutionsMut},
    problems::VectorProblem,
    Problem, State,
};

#[derive(Clone, Serialize, Deserialize)]
pub struct DEBinomialCrossover {
    pc: f64,
}

impl DEBinomialCrossover {
    pub fn from_params(pc: f64) -> Self {
        Self { pc }
    }

    pub fn new<P>(pc: f64) -> Box<dyn Component<P>>
    where
        P: Problem + VectorProblem<Element = f64>,
    {
        Box::new(Self::from_params(pc))
    }
}

impl<P> Component<P> for DEBinomialCrossover
where
    P: Problem + VectorProblem<Element = f64>,
{
    fn execute(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        let mut populations = state.populations_mut();
        let mut rng = state.random_mut();

        let mut mutations = populations.pop();
        let bases = populations.current();

        for (mutation, base) in multizip((mutations.as_solutions_mut(), bases.as_solutions())) {
            let index = rng.gen_range(0..problem.dimension());

            for i in 0..problem.dimension() {
                if rng.gen::<f64>() <= self.pc || i == index {
                    mutation[i] = base[i];
                }
            }
        }

        populations.push(mutations);
        Ok(())
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DEExponentialCrossover {
    pc: f64,
}

impl DEExponentialCrossover {
    pub fn from_params(pc: f64) -> Self {
        Self { pc }
    }

    pub fn new<P>(pc: f64) -> Box<dyn Component<P>>
    where
        P: Problem + VectorProblem<Element = f64>,
    {
        Box::new(Self::from_params(pc))
    }
}

impl<P> Component<P> for DEExponentialCrossover
where
    P: Problem + VectorProblem<Element = f64>,
{
    fn execute(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        let mut populations = state.populations_mut();
        let mut rng = state.random_mut();

        let mut mutations = populations.pop();
        let bases = populations.current();

        for (mutation, base) in multizip((mutations.as_solutions_mut(), bases.as_solutions())) {
            let index = rng.gen_range(0..problem.dimension());
            let mut i = index;

            loop {
                mutation[i] = base[i];
                i = (i + 1) % problem.dimension();

                if rng.gen::<f64>() > self.pc || i == index {
                    break;
                }
            }
        }

        populations.push(mutations);
        Ok(())
    }
}
