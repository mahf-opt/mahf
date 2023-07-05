//! Mutation components for Differential Evolution (DE).

use color_eyre::Section;
use eyre::{ensure, eyre};
use itertools::{multizip, Itertools};
use serde::{Deserialize, Serialize};

use crate::{
    component::ExecResult, components::Component, population::AsSolutionsMut,
    problems::VectorProblem, utils::with_index, State,
};

#[derive(Clone, Serialize, Deserialize)]
pub struct DEMutation {
    y: u32,
    f: f64,
}

impl DEMutation {
    pub fn from_params(y: u32, f: f64) -> ExecResult<Self> {
        ensure!(
            [1, 2].contains(&y),
            "`y` needs to be one of {{1, 2}}, but was {}",
            y
        );
        ensure!(
            (0.0..=2.0).contains(&f),
            "`f` must be in [0, 2], but was {}",
            f
        );
        Ok(Self { y, f })
    }

    pub fn new<P>(y: u32, f: f64) -> ExecResult<Box<dyn Component<P>>>
    where
        P: VectorProblem<Element = f64>,
    {
        Ok(Box::new(Self::from_params(y, f)?))
    }
}

impl<P> Component<P> for DEMutation
where
    P: VectorProblem<Element = f64>,
{
    fn execute(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
        let mut populations = state.populations_mut();
        let population = populations.current_mut();

        let size = (self.y * 2 + 1) as usize;

        if !population.len() % size == 0 {
            return Err(eyre!("the population must be in the format [`y` * 2 + 1]*, where the first individual is the base of the mutation"))
                .suggestion("try to use an appropriate selection method for this mutation");
        }

        let mut solutions = population.as_solutions_mut();

        for chunk in solutions.chunks_exact_mut(size) {
            match chunk {
                [ref mut base, remainder @ ..] => {
                    let pairs: Vec<[_; 2]> = remainder
                        .iter()
                        .chunks(2)
                        .into_iter()
                        .map(|pair| pair.collect_vec().try_into().unwrap())
                        .collect();

                    for [solution1, solution2] in pairs {
                        for (x, s1, s2) in
                            multizip((base.iter_mut(), solution1.iter(), solution2.iter()))
                        {
                            *x += self.f * (s1 - s2);
                        }
                    }
                }
                _ => unreachable!(),
            }
        }

        population.retain(with_index(|i, _| i % size == 0));

        Ok(())
    }
}
