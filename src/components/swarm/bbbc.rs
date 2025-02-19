use std::f64::EPSILON;
use rand::distributions::{Distribution, Uniform};
use rand_distr::Normal;
use serde::Serialize;

use crate::{
    component::ExecResult,
    components::Component,
    identifier::{Global, Identifier, PhantomId},
    population::{AsSolutionsMut, BestIndividual},
    problems::LimitedVectorProblem,
    SingleObjectiveProblem, State,
};
use crate::components::selection;
use crate::population::IntoIndividuals;
use crate::prelude::selection::selection;

/// Updates the positions of particles according to the cyclic universe mechanism proposed for the
/// Big Bang - Big Crunch (BBBC) algorithm.
#[derive(Clone, Serialize)]
pub struct CyclicUniverseMechanism<I: Identifier = Global> {
    /// Number of new individuals to generate.
    pub new_pop: u32,
    id: PhantomId<I>,
}

impl<I: Identifier> CyclicUniverseMechanism<I> {
    pub fn from_params(new_pop: u32) -> Self {
        Self {
            new_pop,
            id: PhantomId::default(),
        }
    }

    pub fn new_with_id<P>(new_pop: u32) -> Box<dyn Component<P>>
    where
        P: SingleObjectiveProblem + LimitedVectorProblem<Element = f64>,
    {
        Box::new(Self::from_params(new_pop))
    }
}

impl CyclicUniverseMechanism<Global> {
    pub fn new<P>(new_pop: u32) -> Box<dyn Component<P>>
    where
        P: SingleObjectiveProblem + LimitedVectorProblem<Element = f64>,
    {
        Self::new_with_id(new_pop)
    }
}

impl<P, I> Component<P> for CyclicUniverseMechanism<I>
where
    P: SingleObjectiveProblem + LimitedVectorProblem<Element = f64>,
    I: Identifier,
{
    fn init(&self, _problem: &P, _state: &mut State<P>) -> ExecResult<()> {
        Ok(())
    }

    fn execute(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        let mut rng = state.random_mut();
        let distribution = Uniform::new(0., 1.0);

        // Get population from state
        let xs = state.populations_mut().pop();
        
        // Shift fitness values to always be > 0
        let (_max, min) = selection::functional::objective_bounds(&*xs).unwrap();
        let objective_values: Vec<_> = if min > 0.0 {
            xs.iter().map(|i| i.objective().value()).collect()
        } else {
            xs.iter().map(|i| i.objective().value() + min.abs() + f64::EPSILON).collect()
        };

        // prepare parameters
        let &Self {
            new_pop, ..
        } = self;

        // Calculate center of mass
        let inverse_fitness_sum = objective_values
            .iter()
            .map(|o| 1.0 / o)
            .sum::<f64>();
        
        let mut positions = Vec::new();
        for (o, i) in xs.iter().enumerate() {
            let weighted_position = i.solution().iter().map(|x| (1.0 / objective_values[o]) * x).collect::<Vec<f64>>();
            positions.push(weighted_position);
        }
        let sum_positions = positions.iter()
            .map(|v| v.iter()) // Convert each vector into an iterator
            .fold(None, |acc: Option<Vec<f64>>, v_iter| {
                Some(match acc {
                    None => v_iter.cloned().collect(), // First vector initializes the result
                    Some(mut acc_vec) => {
                        acc_vec.iter_mut().zip(v_iter).for_each(|(a, &b)| *a += b);
                        acc_vec
                    }
                })
            }).unwrap_or_default();
        
        let center = sum_positions.iter().map(|p| p / inverse_fitness_sum).collect::<Vec<f64>>();

        // Generate new candidate solutions (new_pop specifies how many)
        let mut new_solutions = Vec::new();
        for _ in 0..new_pop {
            let new_ind = center
                .iter()
                .zip(problem.domain())
                .map(|(c, p)| c + (p.end * distribution.sample(&mut *rng)) / state.iterations() as f64)
                .collect::<Vec<f64>>();
            new_solutions.push(new_ind);
        }

        state.populations_mut().push(new_solutions.into_individuals());
        Ok(())
    }
}