//! Particle Swarm Optimization (PSO).

use eyre::WrapErr;

use crate::{
    component::ExecResult,
    components::*,
    conditions::Condition,
    configuration::Configuration,
    identifier::{Global, Identifier},
    lens::ValueOf,
    logging::Logger,
    problems::{LimitedVectorProblem, SingleObjectiveProblem},
    state::common,
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
    condition: Box<dyn Condition<P>>,
) -> ExecResult<Configuration<P>>
where
    P: SingleObjectiveProblem + LimitedVectorProblem<Element = f64>,
{
    let RealProblemParameters {
        num_particles,
        start_weight,
        end_weight,
        c_one,
        c_two,
        v_max,
    } = params;

    Ok(Configuration::builder()
        .do_(initialization::RandomSpread::new(num_particles))
        .evaluate::<Global>()
        .update_best_individual()
        .do_(pso::<P, Global>(
            Parameters {
                particle_init: Block::new([
                    <swarm::ParticleVelocitiesInit>::new(v_max)
                        .wrap_err("failed to construct particle velocities init")?,
                    <swarm::PersonalBestParticlesInit>::new(),
                    <swarm::GlobalBestParticleUpdate>::new(),
                ]),
                particle_update: <swarm::ParticleVelocitiesUpdate>::new(
                    start_weight,
                    c_one,
                    c_two,
                    v_max,
                )
                .wrap_err("failed to construct particle velocities update")?,
                constraints: boundary::Saturation::new(),
                inertia_weight_update: Some(mapping::Linear::new(
                    start_weight,
                    end_weight,
                    ValueOf::<common::Progress<ValueOf<common::Iterations>>>::new(),
                    ValueOf::<swarm::InertiaWeight<swarm::ParticleVelocitiesUpdate>>::new(),
                )),
                state_update: Block::new([
                    <swarm::PersonalBestParticlesUpdate>::new(),
                    <swarm::GlobalBestParticleUpdate>::new(),
                ]),
            },
            condition,
        ))
        .build())
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
pub fn pso<P, I>(params: Parameters<P>, condition: Box<dyn Condition<P>>) -> Box<dyn Component<P>>
where
    P: SingleObjectiveProblem,
    I: Identifier,
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
        .while_(condition, |builder| {
            builder
                .do_(particle_update)
                .do_(constraints)
                .evaluate::<I>()
                .update_best_individual()
                .do_if_some_(inertia_weight_update)
                .do_(state_update)
                .do_(Logger::new())
        })
        .build_component()
}
