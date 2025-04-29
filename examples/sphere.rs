use std::ops::Range;

use mahf::{
    conditions::common::{ChangeOf, DeltaEqChecker},
    experiments::par_experiment,
    lens::common::{BestObjectiveValueLens, BestSolutionLens},
    prelude::*,
};

pub struct Sphere {
    pub dim: usize,
}

impl Sphere {
    pub fn new(dim: usize) -> Self {
        Self { dim }
    }
}

impl Problem for Sphere {
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
        vec![-1.0..1.0; self.dim]
    }
}

impl ObjectiveFunction for Sphere {
    fn objective(&self, solution: &Self::Encoding) -> Self::Objective {
        debug_assert_eq!(solution.len(), self.dim);
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
    // Required for pretty error messages and stacktrace.
    color_eyre::install()?;

    // Specify the problem: Sphere function with 10 dimensions.
    let problem = Sphere::new(/* dim: */ 10);

    // Specify the metaheuristic: e.g. Particle Swarm Optimization ...
    let _: Configuration<Sphere> = pso::real_pso(
        /* params: */
        pso::RealProblemParameters {
            num_particles: 120,
            start_weight: 0.9,
            end_weight: 0.4,
            c_one: 1.7,
            c_two: 1.7,
            v_max: 1.0,
        },
        /* condition: */
        conditions::LessThanN::iterations(10_000) & !conditions::OptimumReached::new(0.01)?,
    )?;

    // ... or a Genetic Algorithm.
    let config = ga::real_ga(
        /* params: */
        ga::RealProblemParameters {
            population_size: 120,
            tournament_size: 5,
            pm: 1.0,
            deviation: 0.1,
            pc: 0.8,
        },
        /* condition: */
        conditions::LessThanN::iterations(10_000) & !conditions::OptimumReached::new(0.01)?,
    )?;

    let setup = |state: &mut State<_>| -> ExecResult<()> {
        state.insert_evaluator(evaluate::Sequential::new());
        state.configure_log(|config| {
            config
                .with_common(conditions::EveryN::iterations(50))
                .with(
                    ChangeOf::new(
                        DeltaEqChecker::new(0.001.try_into().unwrap()),
                        BestObjectiveValueLens::new(),
                    ) & conditions::OptimumReached::new(0.05)?,
                    BestObjectiveValueLens::entry(),
                )
                .with(
                    conditions::EveryN::iterations(1_000),
                    BestSolutionLens::entry(),
                );
            Ok(())
        })
    };

    par_experiment(&config, setup, &[problem], 4096, "data/bmf/GA", false)
}
