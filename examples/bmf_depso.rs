use mahf::prelude::*;
use problems::bmf::BenchmarkFunction;

fn main() {
    let y = 2;
    let f = 1.;

    let problem: BenchmarkFunction = BenchmarkFunction::rastrigin(/*dim: */ 30);

    let config: Configuration<BenchmarkFunction> = Configuration::builder()
        .do_(initialization::RandomSpread::new_init(4 * 30))
        .evaluate()
        .update_best_individual()
        .do_(state::ParticleSwarm::initializer(1.))
        .while_(
            termination::LessThanN::<state::common::Iterations>::new(/*n: */ 500)
                & termination::DistanceToOptGreaterThan::new(0.01),
            |builder| {
                builder
                    .if_else_(
                        branching::RandomChance::new(0.8),
                        |builder| {
                            builder
                                .do_(selection::DEBest::new(y))
                                .do_(generation::mutation::DEMutation::new(y, f))
                                .do_(generation::recombination::DEBinomialCrossover::new(0.8))
                        },
                        |builder| {
                            builder.do_(selection::All::new()).do_(
                                generation::swarm::ParticleSwarmGeneration::new(1., 1., 1., 1.),
                            )
                        },
                    )
                    .do_(constraints::Saturation::new())
                    .evaluate()
                    .update_best_individual()
                    .do_(replacement::KeepBetterAtIndex::new())
                    .do_(state::ParticleSwarm::updater())
            },
        )
        .build();

    // Execute the metaheuristic on the problem with a random seed.
    let state: State<BenchmarkFunction> = config.optimize(&problem);

    // Print the results.
    println!("Found Individual: {:?}", state.best_individual().unwrap());
    println!("This took {} iterations.", state.iterations());
    println!("Global Optimum: {}", problem.known_optimum());
}
