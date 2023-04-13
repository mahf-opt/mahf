use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::sync::Arc;

use anyhow::Context;
use itertools::Itertools;
use rayon::prelude::*;

use mahf::prelude::*;
use mahf::tracking::files;
use problems::bmf::BenchmarkFunction;
use state::common::Iterations;
use termination::{DistanceToOptGreaterThan, LessThanN};

fn main() -> anyhow::Result<()> {
    #[derive(Clone, serde::Serialize)]
    pub struct SplitFittest;

    impl SplitFittest {
        #[allow(clippy::new_ret_no_self)]
        pub fn new<P: problems::SingleObjectiveProblem>() -> Box<dyn Component<P>> {
            Box::new(Self)
        }
    }

    impl<P: problems::SingleObjectiveProblem> Component<P> for SplitFittest {
        #[contracts::requires(state.populations().current().len() % 2 == 0)]
        fn execute(&self, _problem: &P, state: &mut State<P>) {
            let mut population = state.populations_mut().pop();
            population.sort_unstable_by_key(|i| *i.objective());
            let n = population.len();
            let (lower, upper) = population
                .into_iter()
                .chunks(n / 2)
                .into_iter()
                .map(|c| c.collect_vec())
                .collect_tuple::<(_, _)>()
                .unwrap();
            state.populations_mut().push(upper);
            state.populations_mut().push(lower);
        }
    }

    #[derive(Clone, serde::Serialize)]
    pub struct SwapPopulations;

    impl SwapPopulations {
        #[allow(clippy::new_ret_no_self)]
        pub fn new<P: problems::Problem>() -> Box<dyn Component<P>> {
            Box::new(Self)
        }
    }

    impl<P: problems::Problem> Component<P> for SwapPopulations {
        #[contracts::requires(state.populations().len() >= 2)]
        fn execute(&self, _problem: &P, state: &mut State<P>) {
            let p1 = state.populations_mut().pop();
            let p2 = state.populations_mut().pop();
            state.populations_mut().push(p1);
            state.populations_mut().push(p2);
        }
    }

    let problems = [
        BenchmarkFunction::rastrigin(/*dim: */ 30),
        BenchmarkFunction::sphere(/*dim: */ 30),
        BenchmarkFunction::ackley(/*dim: */ 30),
    ];

    let config: Configuration<BenchmarkFunction> = Configuration::builder()
        .do_(initialization::RandomSpread::new_init(30))
        .evaluate()
        .update_best_individual()
        .do_(state::ParticleSwarm::initializer(1.))
        .while_(
            LessThanN::<Iterations>::new(/*n: */ 1000) & DistanceToOptGreaterThan::new(0.01),
            |builder| {
                builder
                    .do_(SplitFittest::new())
                    .do_(generation::recombination::ArithmeticCrossover::new_both(1.))
                    .do_(generation::mutation::GaussianMutation::new(0.2, 0.1))
                    .do_(SwapPopulations::new())
                    .do_(generation::swarm::ParticleSwarmGeneration::new(
                        1., 1., 1., 1.,
                    ))
                    .do_(replacement::Merge::new())
                    .do_(constraints::Saturation::new())
                    .evaluate()
                    .update_best_individual()
                    .do_(state::ParticleSwarm::updater())
                    .do_many_([
                        state::diversity::TrueDiversity::new(),
                        state::diversity::DimensionWiseDiversity::new(),
                        state::diversity::DistanceToAveragePointDiversity::new(),
                        state::diversity::PairwiseDistanceDiversity::new(),
                    ])
                    .do_(tracking::Logger::new())
            },
        )
        .build();

    experiment(config, logging, &problems, 50, "data/bmf/GA_PSO")
}

fn logging<P>(state: &mut State<P>)
where
    P: problems::SingleObjectiveProblem,
    <P as problems::Problem>::Encoding: serde::Serialize,
{
    use tracking::extractor::normalized_diversity;
    state.insert(
        tracking::LogSet::<P>::new()
            .with_many(
                tracking::trigger::Iteration::new(1),
                [
                    normalized_diversity::<_, state::diversity::TrueDiversity>.into(),
                    normalized_diversity::<_, state::diversity::DimensionWiseDiversity>.into(),
                    normalized_diversity::<_, state::diversity::DistanceToAveragePointDiversity>
                        .into(),
                    normalized_diversity::<_, state::diversity::PairwiseDistanceDiversity>.into(),
                ],
            )
            .with(
                tracking::trigger::Iteration::new(5),
                tracking::extractor::best_objective_value,
            ),
    )
}

fn experiment<P>(
    config: Configuration<P>,
    setup: impl Fn(&mut State<P>) + Send + Sync,
    problems: &[P],
    runs: u32,
    folder: impl AsRef<Path>,
) -> anyhow::Result<()>
where
    P: problems::SingleObjectiveProblem + problems::HasKnownOptimum + Send + Sync,
    <P as problems::Problem>::Encoding: std::fmt::Debug,
{
    // Create data dir
    let data_dir = Arc::new(folder.as_ref());
    fs::create_dir_all(data_dir.as_ref())?;

    // Write configuration RON file
    let config_log_file = data_dir.join("configuration.ron");
    ron::ser::to_writer_pretty(
        std::io::BufWriter::new(
            File::create(config_log_file).context("failed to create configuration file")?,
        ),
        config.heuristic(),
        ron::ser::PrettyConfig::default().struct_names(true),
    )
    .context("failed to serialize configuration")?;

    (0..runs)
        .into_par_iter()
        .map(|run| {
            for problem in problems {
                let log_file = data_dir.join(format!("{}_{run}.log", problem.name()));
                let state = config.optimize_with(problem, |state| {
                    state.insert(Random::seeded(run as u64));
                    setup(state);
                });
                files::write_log_file(log_file, state.log())?;
                println!("Finished run {run}.");
                println!("Found Individual: {:?}", state.best_individual().unwrap());
                println!("This took {} iterations.", state.iterations());
                println!("Global Optimum: {:?}", problem.known_optimum());
                std::io::stdout().flush().unwrap();
            }
            Ok(())
        })
        .collect::<anyhow::Result<()>>()?;

    println!("All runs finished.");
    Ok(())
}
