use std::f64::consts::PI;
use eyre::Context;
use rand::distributions::{Distribution};
use rand::Rng;
use rand_distr::Normal;
use serde::Serialize;

use crate::{
    component::ExecResult,
    components::Component,
    identifier::{Global, Identifier, PhantomId},
    problems::LimitedVectorProblem,
    SingleObjectiveProblem, State,
};
use crate::components::initialization::functional::random_spread;
use crate::population::IntoIndividuals;

/// Updates the positions of particles according to the mine explosion dynamics proposed for the
/// Mine Blast Algorithm (MBA).
#[derive(Clone, Serialize)]
pub struct MineExplosionDynamics<I: Identifier = Global> {
    /// Number of new individuals to generate.
    pub num_pieces: u32,
    /// Solution used as center.
    pub center: String,
    id: PhantomId<I>,
}

impl<I: Identifier> MineExplosionDynamics<I> {
    pub fn from_params(num_pieces: u32, center: String) -> Self {
        Self {
            num_pieces,
            center,
            id: PhantomId::default(),
        }
    }

    pub fn new_with_id<P>(num_pieces: u32, center: String) -> Box<dyn Component<P>>
    where
        P: SingleObjectiveProblem + LimitedVectorProblem<Element = f64>,
    {
        Box::new(Self::from_params(num_pieces, center))
    }
}

impl MineExplosionDynamics<Global> {
    pub fn new<P>(num_pieces: u32, center: String) -> Box<dyn Component<P>>
    where
        P: SingleObjectiveProblem + LimitedVectorProblem<Element = f64>,
    {
        Self::new_with_id(num_pieces, center)
    }
}

impl<P, I> Component<P> for MineExplosionDynamics<I>
where
    P: SingleObjectiveProblem + LimitedVectorProblem<Element = f64>,
    I: Identifier,
{
    fn init(&self, _problem: &P, _state: &mut State<P>) -> ExecResult<()> {
        Ok(())
    }

    fn execute(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        let mut rng = state.random_mut();
        let distribution: Normal<f64> = Normal::new(0., 1.0).wrap_err("invalid distribution")?;

        // Get population from state
        let xs = state.populations_mut().pop();
        
        // Set center solution
        let mut center_solution = Vec::new();
        if self.center.as_str() == "random_new" {
            center_solution = random_spread(&problem.domain(), 1, &mut *rng)[0].clone();
        } else if self.center.as_str() == "best" { 
            center_solution = state.best_individual().unwrap().solution().clone();
        } else if self.center.as_str() == "random_solution" { 
            let random_index = &mut rng.gen_range(0..xs.len());
            center_solution = xs[*random_index].solution().clone();
        } else {
            println!("Invalid center solution");
        }

        // prepare parameters
        let &Self {
            num_pieces, ..
        } = self;

        // get distances as difference between upper and lower bound in every dimension
        let mut distances = problem
            .domain()
            .iter()
            .map(|p| (p.end - p.start).abs())
            .collect::<Vec<f64>>();

        // calculate vector of exploding particle positions by adding distance to center solution
        let mut exploding_particles = Vec::new();
        for _p in 0..num_pieces {
            let mine_distance = distances
                .iter()
                .map(|d| d * distribution.sample(&mut *rng))
                .collect::<Vec<f64>>();
            let xe = mine_distance
                .iter()
                .zip(center_solution.iter())
                .map(|(d, c)| c + d * (2.0 * PI / num_pieces as f64))
                .collect::<Vec<f64>>();
            exploding_particles.push(xe);
        }

        state.populations_mut().push(exploding_particles.into_individuals());
        Ok(())
    }
}