//! Black Hole Algorithm (BH).
//!
//! # References
//!
//! \[1\] Abdolreza Hatamlou. 2013.
//! Black hole: A new heuristic optimization approach for data clustering.
//! In Information Sciences 222, 175–184.
//! DOI:<http://dx.doi.org/10.1016/j.ins.2012.08.023>

use crate::{
    component::ExecResult,
    components::{boundary, initialization, replacement, swarm},
    conditions::Condition,
    configuration::Configuration,
    identifier::{Global, Identifier},
    logging::Logger,
    problems::{LimitedVectorProblem, SingleObjectiveProblem},
    Component,
};

/// Parameters for [`real_bh`].
pub struct RealProblemParameters {
    pub num_particles: u32,
}

/// An example single-objective BH algorithm operating on a real search space.
///
/// Uses the [`bh`] component internally.
pub fn real_bh<P>(
    params: RealProblemParameters,
    condition: Box<dyn Condition<P>>,
) -> ExecResult<Configuration<P>>
where
    P: SingleObjectiveProblem + LimitedVectorProblem<Element = f64>,
{
    let RealProblemParameters { num_particles } = params;

    Ok(Configuration::builder()
        .do_(initialization::RandomSpread::new(num_particles))
        .evaluate()
        .update_best_individual()
        .do_(bh::<P, Global>(
            Parameters {
                particle_update: swarm::bh::BlackHoleParticlesUpdate::new(),
                constraints: boundary::Saturation::new(),
                replacement: replacement::bh::EventHorizon::new(),
            },
            condition,
        ))
        .build())
}

/// Basic building blocks of [`bh`].
pub struct Parameters<P> {
    pub particle_update: Box<dyn Component<P>>,
    pub constraints: Box<dyn Component<P>>,
    pub replacement: Box<dyn Component<P>>,
}

/// A generic single-objective Black Hole algorithm (BH) template.
pub fn bh<P, I>(params: Parameters<P>, condition: Box<dyn Condition<P>>) -> Box<dyn Component<P>>
where
    P: SingleObjectiveProblem,
    I: Identifier,
{
    let Parameters {
        particle_update,
        constraints,
        replacement,
    } = params;

    Configuration::builder()
        .while_(condition, |builder| {
            builder
                .do_(particle_update)
                .do_(constraints)
                .evaluate_with::<I>()
                .update_best_individual()
                .do_(replacement)
                .evaluate_with::<I>()
                .update_best_individual()
                .do_(Logger::new())
        })
        .build_component()
}
