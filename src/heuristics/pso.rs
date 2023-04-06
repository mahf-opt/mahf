//! Particle Swarm Optimization

use crate::{
    components::*,
    conditions::Condition,
    framework::Configuration,
    problems::{LimitedVectorProblem, SingleObjectiveProblem},
    state,
    tracking::Logger,
};

/// Parameters for [real_pso].
pub struct RealProblemParameters {
    pub num_particles: u32,
    pub start_weight: f64,
    pub end_weight: f64,
    pub c_one: f64,
    pub c_two: f64,
    pub v_max: f64,
}

/// An example single-objective Particle Swarm Optimization operating on a real search space.
/// Uses the [pso] component internally.
pub fn real_pso<P>(
    params: RealProblemParameters,
    termination: Box<dyn Condition<P>>,
) -> Configuration<P>
where
    P: SingleObjectiveProblem<Encoding = Vec<f64>> + LimitedVectorProblem<T = f64> + 'static,
{
    let RealProblemParameters {
        num_particles,
        start_weight,
        end_weight,
        c_one,
        c_two,
        v_max,
    } = params;

    Configuration::builder()
        .do_(initialization::RandomSpread::new_init(num_particles))
        .evaluate()
        .update_best_individual()
        .do_(pso(
            Parameters {
                particle_init: state::ParticleSwarm::initializer(v_max),
                particle_update: generation::swarm::ParticleSwarmGeneration::new(
                    start_weight,
                    c_one,
                    c_two,
                    v_max,
                ),
                constraints: constraints::Saturation::new(),
                inertia_weight_update: Some(mapping::Linear::new::<
                    _,
                    state::common::Progress,
                    generation::swarm::Weight,
                >(start_weight, end_weight)),
                state_update: state::ParticleSwarm::updater(),
            },
            termination,
        ))
        .build()
}

/// Basic building blocks of Particle Swarm Optimization.
pub struct Parameters<P> {
    pub particle_init: Box<dyn Component<P>>,
    pub particle_update: Box<dyn Component<P>>,
    pub constraints: Box<dyn Component<P>>,
    pub inertia_weight_update: Option<Box<dyn Component<P>>>,
    pub state_update: Box<dyn Component<P>>,
}

/// A generic single-objective Particle Swarm Optimization template.
pub fn pso<P>(params: Parameters<P>, termination: Box<dyn Condition<P>>) -> Box<dyn Component<P>>
where
    P: SingleObjectiveProblem,
{
    let Parameters {
        particle_init,
        particle_update,
        constraints,
        inertia_weight_update,
        state_update,
    } = params;

    Configuration::builder()
        .do_(particle_init)
        .while_(termination, |builder| {
            builder
                .do_(particle_update)
                .do_(constraints)
                .evaluate()
                .update_best_individual()
                .do_if_some_(inertia_weight_update)
                .do_(state_update)
                .do_(Logger::new())
        })
        .build_component()
}
