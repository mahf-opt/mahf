use crate::{
    heuristic::{
        components::{Generation, Postprocess},
        Configuration, CustomState, Individual, State,
    },
    operators::*,
    problem::{LimitedVectorProblem, Problem},
    random::Random,
};
use rand::Rng;

pub fn pso<P>(
    num_particles: u32,
    a: f64,
    b: f64,
    c: f64,
    v_max: f64,
    max_iterations: u32,
) -> Configuration<P>
where
    P: Problem<Encoding = Vec<f64>> + LimitedVectorProblem<T = f64>,
{
    Configuration::new_extended(
        initialization::RandomSpread {
            initial_population_size: num_particles,
        },
        Some(PsoPostprocess { v_max }),
        selection::All,
        PsoGeneration { a, b, c, v_max },
        replacement::Generational {
            max_population_size: num_particles,
        },
        Some(PsoPostprocess { v_max }),
        termination::FixedIterations { max_iterations },
    )
}

pub struct PsoState {
    velocities: Vec<Vec<f64>>,
    bests: Vec<Individual>,
    global_best: Individual,
}
impl CustomState for PsoState {}

#[derive(Debug, serde::Serialize)]
pub struct PsoPostprocess {
    pub v_max: f64,
}

impl<P> Postprocess<P> for PsoPostprocess
where
    P: Problem<Encoding = Vec<f64>> + LimitedVectorProblem<T = f64>,
{
    fn post_initialize(
        &self,
        state: &mut State,
        problem: &P,
        rng: &mut Random,
        population: &Vec<Individual>,
    ) {
        if !state.custom.has::<PsoState>() {
            let velocities = population
                .iter()
                .map(|_| {
                    (0..problem.dimension())
                        .into_iter()
                        .map(|_| rng.gen_range(-self.v_max..=self.v_max))
                        .collect::<Vec<f64>>()
                })
                .collect::<Vec<Vec<f64>>>();

            let bests = population
                .iter()
                .map(Individual::clone::<Vec<f64>>)
                .collect::<Vec<Individual>>();

            let global_best = bests
                .iter()
                .min_by_key(|i| Individual::fitness(i))
                .map(Individual::clone::<Vec<f64>>)
                .unwrap();

            state.custom.insert(PsoState {
                velocities,
                bests,
                global_best,
            });
        } else {
            let pso_state = state.custom.get_mut::<PsoState>();

            for (i, individual) in population.iter().enumerate() {
                if pso_state.bests[i].fitness() > individual.fitness() {
                    pso_state.bests[i] = individual.clone::<Vec<f64>>();

                    if pso_state.global_best.fitness() > individual.fitness() {
                        pso_state.global_best = individual.clone::<Vec<f64>>();
                    }
                }
            }
        }
    }
}

#[derive(serde::Serialize)]
pub struct PsoGeneration {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub v_max: f64,
}

impl<P> Generation<P> for PsoGeneration
where
    P: Problem<Encoding = Vec<f64>>,
{
    fn generate(
        &self,
        state: &mut State,
        _problem: &P,
        rng: &mut Random,
        parents: &mut Vec<&P::Encoding>,
        offspring: &mut Vec<P::Encoding>,
    ) {
        let &PsoGeneration { a, b, c, v_max } = self;
        let pso_state = state.custom.get_mut::<PsoState>();
        let rs = rng.gen_range(0.0..=1.0);
        let rt = rng.gen_range(0.0..=1.0);

        for (i, x) in parents.iter().enumerate() {
            let v = &mut pso_state.velocities[i];
            let xl = &pso_state.bests[i].solution::<Vec<f64>>();
            let xg = &pso_state.global_best.solution::<Vec<f64>>();

            for i in 0..v.len() {
                v[i] = a * v[i] + b * rs * (xg[i] - x[i]) + c * rt * (xl[i] - x[i]);
                v[i] = v[i].clamp(-v_max, v_max);
            }

            let xn = x
                .iter()
                .zip(v.iter())
                .map(|(xi, vi)| xi + vi)
                .map(|xi| xi.clamp(-1.0, 1.0))
                .collect::<Vec<f64>>();

            offspring.push(xn);
        }
    }
}
