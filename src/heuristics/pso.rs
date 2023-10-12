//! Particle Swarm Optimization (PSO).
//!
//! # References
//!
//! \[1\] James Kennedy and Russell Eberhart. 1995.
//! Particle swarm optimization.
//! In Proceedings of ICNN’95 - International Conference on Neural Networks, 1942–1948 vol.4.
//! DOI:<https://doi.org/10/bdc3t3>
//!
//! \[2\] Riccardo Poli, James Kennedy, and Tim Blackwell. 2007.
//! Particle swarm optimization.
//! Swarm Intell 1, 1 (June 2007), 33–57.
//! DOI:<https://doi.org/10/dhnq29>

use eyre::WrapErr;

use crate::{
    component::ExecResult,
    components::{boundary, initialization, mapping, swarm},
    conditions::Condition,
    configuration::Configuration,
    identifier::{Global, Identifier},
    lens::ValueOf,
    logging::Logger,
    problems::{LimitedVectorProblem, SingleObjectiveProblem},
    state::common,
    Component,
};

/// Parameters for [`real_pso`].
pub struct RealProblemParameters {
    pub num_particles: u32,
    pub start_weight: f64,
    pub end_weight: f64,
    pub c_one: f64,
    pub c_two: f64,
    pub v_max: f64,
}

/// An example single-objective PSO operating on a real search space.
///
/// Uses the [`pso`] component internally.
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
        .evaluate()
        .update_best_individual()
        .do_(pso::<P, Global>(
            Parameters {
                particle_init: swarm::pso::ParticleSwarmInit::new(v_max)?,
                particle_update: swarm::pso::ParticleVelocitiesUpdate::new(
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
                    ValueOf::<swarm::pso::InertiaWeight<swarm::pso::ParticleVelocitiesUpdate>>::new(
                    ),
                )),
                state_update: swarm::pso::ParticleSwarmUpdate::new(),
            },
            condition,
        ))
        .build())
}

/// Basic building blocks of [`pso`].
pub struct Parameters<P> {
    pub particle_init: Box<dyn Component<P>>,
    pub particle_update: Box<dyn Component<P>>,
    pub constraints: Box<dyn Component<P>>,
    pub inertia_weight_update: Option<Box<dyn Component<P>>>,
    pub state_update: Box<dyn Component<P>>,
}

/// A generic single-objective Particle Swarm Optimization (PSO) template.
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
                .evaluate_with::<I>()
                .update_best_individual()
                .do_if_some_(inertia_weight_update)
                .do_(state_update)
                .do_(Logger::new())
        })
        .build_component()
}
