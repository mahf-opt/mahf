//! Particle Swarm Optimization

use crate::{
    framework::{components::Component, conditions::Condition, Configuration},
    operators::*,
    problems::{LimitedVectorProblem, SingleObjectiveProblem},
};

/// Parameters for [real_pso].
pub struct RealProblemParameters {
    pub num_particles: u32,
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub v_max: f64,
}

/// An example single-objective Particle Swarm Optimization operating on a real search space.
/// Uses the [pso] component internally.
pub fn real_pso<P>(
    params: RealProblemParameters,
    termination: Box<dyn Condition<P>>,
    logger: Box<dyn Component<P>>,
) -> Configuration<P>
where
    P: SingleObjectiveProblem<Encoding = Vec<f64>> + LimitedVectorProblem<T = f64> + 'static,
{
    let RealProblemParameters {
        num_particles,
        a,
        b,
        c,
        v_max,
    } = params;

    Configuration::builder()
        .do_(initialization::RandomSpread::new_init(num_particles))
        .evaluate_sequential()
        .update_best_individual()
        .do_(pso(
            Parameters {
                particle_init: pso_ops::PsoStateInitialization::new(v_max),
                particle_update: generation::swarm::PsoGeneration::new(a, b, c, v_max),
                state_update: pso_ops::PsoStateUpdate::new(),
            },
            termination,
            logger,
        ))
        .build()
}

/// Basic building blocks of Particle Swarm Optimization.
pub struct Parameters<P> {
    particle_init: Box<dyn Component<P>>,
    particle_update: Box<dyn Component<P>>,
    state_update: Box<dyn Component<P>>,
}

/// A generic single-objective Particle Swarm Optimization template.
pub fn pso<P>(
    params: Parameters<P>,
    termination: Box<dyn Condition<P>>,
    logger: Box<dyn Component<P>>,
) -> Box<dyn Component<P>>
where
    P: SingleObjectiveProblem,
{
    let Parameters {
        particle_init,
        particle_update,
        state_update,
    } = params;

    Configuration::builder()
        .do_(particle_init)
        .while_(termination, |builder| {
            builder
                .do_(particle_update)
                .evaluate_sequential()
                .update_best_individual()
                .do_(state_update)
                .do_(logger)
        })
        .build_component()
}

#[allow(clippy::new_ret_no_self)]
mod pso_ops {
    use crate::problems::SingleObjectiveProblem;
    use crate::{
        framework::{components::*, state::State, Individual},
        operators::custom_state::PsoState,
        problems::{LimitedVectorProblem, Problem},
    };
    use rand::Rng;

    #[derive(Debug, serde::Serialize)]
    pub struct PsoStateInitialization {
        v_max: f64,
    }
    impl PsoStateInitialization {
        pub fn new<P: Problem>(v_max: f64) -> Box<dyn Component<P>>
        where
            P: SingleObjectiveProblem<Encoding = Vec<f64>> + LimitedVectorProblem<T = f64>,
        {
            Box::new(Self { v_max })
        }
    }
    impl<P> Component<P> for PsoStateInitialization
    where
        P: SingleObjectiveProblem<Encoding = Vec<f64>> + LimitedVectorProblem<T = f64>,
    {
        fn initialize(&self, _problem: &P, state: &mut State) {
            // Initialize with empty state to satisfy `state.require()` statements
            state.insert(PsoState {
                velocities: vec![],
                bests: vec![],
                global_best: Individual::<P>::new_unevaluated(Vec::new()),
            })
        }

        fn execute(&self, problem: &P, state: &mut State) {
            let population = state.population_stack_mut::<P>().pop();
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
                .min_by_key(|i| Individual::objective(i))
                .cloned()
                .unwrap();

            state.population_stack_mut().push(population);

            state.insert(PsoState {
                velocities,
                bests,
                global_best,
            });
        }
    }

    /// State update for PSO.
    ///
    /// Updates best found solutions of particles and global best in [PsoState].
    #[derive(Debug, serde::Serialize)]
    pub struct PsoStateUpdate;
    impl PsoStateUpdate {
        pub fn new<P: Problem>() -> Box<dyn Component<P>>
        where
            P: Problem<Encoding = Vec<f64>> + LimitedVectorProblem<T = f64>,
        {
            Box::new(Self)
        }
    }
    impl<P> Component<P> for PsoStateUpdate
    where
        P: Problem<Encoding = Vec<f64>> + LimitedVectorProblem<T = f64>,
    {
        fn initialize(&self, _problem: &P, state: &mut State) {
            state.require::<PsoState<P>>();
        }

        fn execute(&self, _problem: &P, state: &mut State) {
            let population = state.population_stack_mut().pop();
            let mut pso_state = state.get_mut::<PsoState<P>>();

            for (i, individual) in population.iter().enumerate() {
                if pso_state.bests[i].objective() > individual.objective() {
                    pso_state.bests[i] = individual.clone();

                    if pso_state.global_best.objective() > individual.objective() {
                        pso_state.global_best = individual.clone();
                    }
                }
            }

            state.population_stack_mut().push(population);
        }
    }
}
