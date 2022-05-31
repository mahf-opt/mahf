//! Particle Swarm Optimization

use crate::{
    framework::legacy::Configuration,
    operators::*,
    problems::{LimitedVectorProblem, Problem},
};

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
    Configuration {
        initialization: initialization::RandomSpread::new_init(num_particles),
        selection: selection::All::new(),
        generation: vec![generation::swarm::PsoGeneration::new(a, b, c, v_max)],
        replacement: replacement::Generational::new(num_particles),
        post_replacement: pso_ops::PsoStateUpdate::new(v_max),
        termination: termination::FixedIterations::new(max_iterations),
        ..Default::default()
    }
}

#[allow(clippy::new_ret_no_self)]
mod pso_ops {
    use crate::{
        framework::{components::*, Individual, State},
        operators::custom_state::PsoState,
        problems::{LimitedVectorProblem, Problem},
    };
    use rand::Rng;

    /// State update for PSO.
    ///
    /// Updates best found solutions of particles and global best in [PsoState].
    #[derive(Debug, serde::Serialize)]
    pub struct PsoStateUpdate {
        pub v_max: f64,
    }
    impl PsoStateUpdate {
        pub fn new<P: Problem>(v_max: f64) -> Box<dyn Component<P>>
        where
            P: Problem<Encoding = Vec<f64>> + LimitedVectorProblem<T = f64>,
        {
            Box::new(Self { v_max })
        }
    }
    impl<P> Component<P> for PsoStateUpdate
    where
        P: Problem<Encoding = Vec<f64>> + LimitedVectorProblem<T = f64>,
    {
        fn initialize(&self, problem: &P, state: &mut State) {
            let population = state.population_stack_mut().pop();
            let rng = state.random_mut();

            let velocities = population
                .iter()
                .map(|_| {
                    (0..problem.dimension())
                        .into_iter()
                        .map(|_| rng.gen_range(-self.v_max..=self.v_max))
                        .collect::<Vec<f64>>()
                })
                .collect::<Vec<Vec<f64>>>();

            let bests = population.to_vec();

            let global_best = bests
                .iter()
                .min_by_key(|i| Individual::fitness(i))
                .cloned()
                .unwrap();

            state.population_stack_mut().push(population);

            state.insert(PsoState {
                velocities,
                bests,
                global_best,
            });
        }

        fn execute(&self, _problem: &P, state: &mut State) {
            let population = state.population_stack_mut().pop();
            let mut pso_state = state.get_mut::<PsoState>();

            for (i, individual) in population.iter().enumerate() {
                if pso_state.bests[i].fitness() > individual.fitness() {
                    pso_state.bests[i] = individual.clone();

                    if pso_state.global_best.fitness() > individual.fitness() {
                        pso_state.global_best = individual.clone();
                    }
                }
            }

            state.population_stack_mut().push(population);
        }
    }
}
