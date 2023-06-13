use std::ops::Range;

use mahf::{
    conditions::common::{ChangeOf, DeltaEqChecker},
    experiments::par_experiment,
    logging::config::LogConfig,
    prelude::*,
    state::extract::common::{ExBestObjectiveValue, ExBestSolution},
    State,
};

pub struct Sphere {
    pub dim: usize,
}

impl Sphere {
    pub fn new(dim: usize) -> Self {
        Self { dim }
    }
}

impl problems::Problem for Sphere {
    type Encoding = Vec<f64>;
    type Objective = SingleObjective;

    fn name(&self) -> &str {
        "Sphere"
    }
}

impl problems::VectorProblem for Sphere {
    type Element = f64;

    fn dimension(&self) -> usize {
        self.dim
    }
}

impl problems::LimitedVectorProblem for Sphere {
    fn domain(&self) -> Vec<Range<Self::Element>> {
        std::iter::repeat(-1.0..1.0).take(self.dim).collect()
    }
}

impl problems::ObjectiveFunction for Sphere {
    fn objective(solution: &Self::Encoding) -> Self::Objective {
        solution
            .iter()
            .map(|x| x.powi(2))
            .sum::<f64>()
            .try_into()
            .unwrap()
    }
}

impl problems::KnownOptimumProblem for Sphere {
    fn known_optimum(&self) -> SingleObjective {
        0.0.try_into().unwrap()
    }
}

fn main() -> ExecResult<()> {
    color_eyre::install()?;

    // Specify the problem: Sphere function with 10 dimensions.
    let problem = Sphere::new(30);
    // Specify the metaheuristic: Particle Swarm Optimization (pre-implemented in MAHF).
    let config = pso::real_pso::<_, problems::evaluate::Sequential<_>>(
        /*params: */
        pso::RealProblemParameters {
            num_particles: 120,
            start_weight: 0.9,
            end_weight: 0.4,
            c_one: 1.7,
            c_two: 1.7,
            v_max: 1.0,
        },
        /*termination: */
        conditions::LessThan::<ValueOf<common::Iterations>>::new(/*n: */ 10_000)
            & conditions::DistanceToOptimumGreaterThan::new(0.01)?,
    )?;

    let setup = |state: &mut State<Sphere>| -> ExecResult<()> {
        state.insert(
            LogConfig::<Sphere>::new()
                .with_common(conditions::EveryN::<ValueOf<common::Iterations>>::new(50))
                .with(
                    ChangeOf::<ExBestObjectiveValue<Sphere>>::new(DeltaEqChecker::new(
                        0.001.try_into().unwrap(),
                    )) & !conditions::DistanceToOptimumGreaterThan::new(0.05)?,
                    ExBestObjectiveValue::entry(),
                )
                .with(
                    conditions::EveryN::<ValueOf<common::Iterations>>::new(1_000),
                    ExBestSolution::entry(),
                ),
        );
        Ok(())
    };

    par_experiment(&config, setup, &[problem], 4096, "data/bmf/PSO", false)
}
