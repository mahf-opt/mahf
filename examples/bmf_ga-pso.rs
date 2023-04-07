use itertools::Itertools;
use mahf::prelude::*;
use problems::bmf::BenchmarkFunction;
use state::common::Iterations;
use termination::LessThanN;

fn main() {
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

    // Specify the problem: Sphere function with 10 dimensions.
    let problem: BenchmarkFunction = BenchmarkFunction::rastrigin(/*dim: */ 30);
    // Specify the metaheuristic: Particle Swarm Optimization (pre-implemented in MAHF).
    let config: Configuration<BenchmarkFunction> = Configuration::builder()
        .do_(initialization::RandomSpread::new_init(30))
        .evaluate()
        .update_best_individual()
        .do_(state::ParticleSwarm::initializer(1.))
        .while_(LessThanN::<Iterations>::new(/*n: */ 500), |builder| {
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
        })
        .build();

    // Execute the metaheuristic on the problem with a random seed.
    let state: State<BenchmarkFunction> = config.optimize(&problem);

    // Print the results.
    println!("Found Individual: {:?}", state.best_individual().unwrap());
    println!("This took {} iterations.", state.iterations());
    println!("Global Optimum: {}", problem.known_optimum());
}
