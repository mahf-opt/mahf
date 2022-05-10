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
        initialization: initialization::RandomSpread::new(num_particles),
        selection: selection::All::new(),
        generation: vec![generation::PsoGeneration::new(a, b, c, v_max)],
        replacement: replacement::Generational::new(num_particles),
        post_replacement: postprocess::PsoPostprocess::new(v_max),
        termination: termination::FixedIterations::new(max_iterations),
        ..Default::default()
    }
}
