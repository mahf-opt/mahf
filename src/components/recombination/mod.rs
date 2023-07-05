//! Recombine multiple solutions (also called crossover).

use crate::{
    component::{AnyComponent, ExecResult},
    population::{IntoIndividuals, IntoSolutions},
    state::random::Random,
    Problem, State,
};

pub mod common;
pub mod de;
pub mod functional;

pub use common::{ArithmeticCrossover, CycleCrossover, NPointCrossover, UniformCrossover};

pub enum OptionalPair<T> {
    None,
    Single(T),
    Both([T; 2]),
}

impl<T> OptionalPair<T> {
    pub fn from_pair(ts: [T; 2], both: bool) -> Self {
        if both {
            Self::Both(ts)
        } else {
            let [t, _] = ts;
            Self::Single(t)
        }
    }
}

pub trait Recombination<P: Problem>: AnyComponent {
    fn recombine(
        &self,
        parent1: &P::Encoding,
        parent2: &P::Encoding,
        rng: &mut Random,
    ) -> OptionalPair<P::Encoding>;
}

erased_serde::serialize_trait_object!(<P: Problem> Recombination<P>);
dyn_clone::clone_trait_object!(<P: Problem> Recombination<P>);

pub fn recombination<P, T>(component: &T, _problem: &P, state: &mut State<P>) -> ExecResult<()>
where
    P: Problem,
    T: Recombination<P>,
{
    let mut populations = state.populations_mut();
    let mut rng = state.random_mut();

    let solutions = populations.pop().into_solutions();
    let mut population = Vec::new();

    for chunk in solutions.chunks(2) {
        match chunk {
            [parent1, parent2] => {
                let children = component.recombine(parent1, parent2, &mut rng);
                match children {
                    OptionalPair::None => {
                        population.push(parent1.clone());
                        population.push(parent2.clone());
                    }
                    OptionalPair::Single(child) => {
                        population.push(child);
                    }
                    OptionalPair::Both([child1, child2]) => {
                        population.push(child1);
                        population.push(child2);
                    }
                }
            }
            [remainder] => population.push(remainder.clone()),
            _ => unreachable!(),
        }
    }

    populations.push(population.into_individuals());
    Ok(())
}
