use std::f64::consts::PI;
use itertools::{izip, multizip};
use rand::distributions::{Distribution, Uniform};
use rand::Rng;
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
use crate::components::mutation::MutationStrength;

/// Updates the positions of particles according to the mine explosion dynamics proposed for the
/// Mine Blast Algorithm (MBA).
#[derive(Clone, Serialize)]
pub struct MineExplosionDynamics<I: Identifier = Global> {
    pub num_pieces: usize,
    id: PhantomId<I>,
}

impl<I: Identifier> MineExplosionDynamics<I> {
    pub fn from_params(num_pieces: usize) -> Self {
        Self {
            num_pieces,
            id: PhantomId::default(),
        }
    }

    pub fn new_with_id<P>(num_pieces: usize) -> Box<dyn Component<P>>
    where
        P: SingleObjectiveProblem + LimitedVectorProblem<Element = f64>,
    {
        Box::new(Self::from_params(num_pieces))
    }
}

impl MineExplosionDynamics<Global> {
    pub fn new<P>(num_pieces: usize) -> Box<dyn Component<P>>
    where
        P: SingleObjectiveProblem + LimitedVectorProblem<Element = f64>,
    {
        Self::new_with_id(num_pieces)
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
        let mut distribution = Normal::new(0., 1.0);

        // Get population from state
        let mut population = state.populations_mut();
        let xs = population.current_mut().as_solutions_mut();

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

        // calculate vector of exploding particle positions
        let mut exploding_particles = Vec::new();
        for _p in num_pieces{
            let mine_distance = distances
                .iter()
                .map(|d| d * (distribution.sample(&mut *rng)).powi(2))
                .collect::<Vec<f64>>();
            let Xe = mine_distance
                .iter()
                .map(|d| d * (2 * PI / num_pieces as f64))
                .collect::<Vec<f64>>();
            exploding_particles.push(Xe);

            // I'm missing m(n+1) here (and d(n+1) is calculated through mine_distances), because its formula is not clear
        }

        for (x_old, x_e) in multizip((xs, exploding_particles)) {
            for i in 0..x_old.len() {
                x_old[i] = x_old[i] + x_e[i];         // the exponential term is still missing here
            }
        }

        Ok(())
    }
}