//! Particle Swarm Optimization

use crate::{
    framework::Configuration,
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
    Configuration::new_extended(
        initialization::RandomSpread {
            initial_population_size: num_particles,
        },
        selection::All,
        generation::PsoGeneration { a, b, c, v_max },
        replacement::Generational {
            max_population_size: num_particles,
        },
        Some(archive::None),
        Some(postprocess::PsoPostprocess { v_max }),
        termination::FixedIterations { max_iterations },
    )
}
