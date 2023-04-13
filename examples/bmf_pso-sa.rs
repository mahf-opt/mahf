use mahf::prelude::*;
use problems::bmf::BenchmarkFunction;

fn main() {
    #[derive(derive_more::Deref, derive_more::DerefMut, better_any::Tid)]
    struct PreviousObjectiveValue(framework::SingleObjective);

    impl state::CustomState<'_> for PreviousObjectiveValue {}

    #[derive(Clone, serde::Serialize)]
    pub struct GlobalBestImproved;

    impl GlobalBestImproved {
        #[allow(clippy::new_ret_no_self)]
        pub fn new<P: problems::SingleObjectiveProblem>() -> Box<dyn Condition<P>> {
            Box::new(Self)
        }
    }

    impl<P: problems::SingleObjectiveProblem> Condition<P> for GlobalBestImproved {
        fn initialize(&self, _problem: &P, state: &mut State<P>) {
            state.insert(PreviousObjectiveValue(framework::SingleObjective::default()));
        }

        fn require(&self, _problem: &P, state: &State<P>) {
            state.require::<Self, state::ParticleSwarm<P>>();
        }

        fn evaluate(&self, _problem: &P, state: &mut State<P>) -> bool {
            let mut mut_state = state.get_states_mut();
            let state::ParticleSwarm { global_best, .. } =
                mut_state.get_mut::<state::ParticleSwarm<P>>();
            let previous = mut_state.get_value::<PreviousObjectiveValue>();
            mut_state.set_value::<PreviousObjectiveValue>(*global_best.objective());

            global_best.objective() < &previous
        }
    }

    #[derive(Clone, serde::Serialize)]
    pub struct GlobalBestToPopulation;

    impl GlobalBestToPopulation {
        #[allow(clippy::new_ret_no_self)]
        pub fn new<P: problems::SingleObjectiveProblem>() -> Box<dyn Component<P>> {
            Box::new(Self)
        }
    }

    impl<P: problems::SingleObjectiveProblem> Component<P> for GlobalBestToPopulation {
        fn require(&self, _problem: &P, state: &State<P>) {
            state.require::<Self, state::ParticleSwarm<P>>();
        }

        fn execute(&self, _problem: &P, state: &mut State<P>) {
            let best = state.get::<state::ParticleSwarm<P>>().global_best.clone();
            state.populations_mut().push(vec![best]);
        }
    }

    #[derive(Clone, serde::Serialize)]
    pub struct SingleToGlobalBest;

    impl SingleToGlobalBest {
        #[allow(clippy::new_ret_no_self)]
        pub fn new<P: problems::SingleObjectiveProblem>() -> Box<dyn Component<P>> {
            Box::new(Self)
        }
    }

    impl<P: problems::SingleObjectiveProblem> Component<P> for SingleToGlobalBest {
        fn require(&self, _problem: &P, state: &State<P>) {
            state.require::<Self, state::ParticleSwarm<P>>();
        }

        #[contracts::requires(state.populations().current().len() == 1)]
        fn execute(&self, _problem: &P, state: &mut State<P>) {
            let best = state.populations_mut().pop().into_iter().next().unwrap();
            state.get_mut::<state::ParticleSwarm<P>>().global_best = best;
        }
    }

    let constraints = constraints::Saturation::new();
    let cooling_schedule = mapping::GeometricCooling::new::<_, replacement::Temperature>(0.95);

    // Specify the problem: Sphere function with 10 dimensions.
    let problem: BenchmarkFunction = BenchmarkFunction::rastrigin(/*dim: */ 30);
    // Specify the metaheuristic: Particle Swarm Optimization (pre-implemented in MAHF).
    let config: Configuration<BenchmarkFunction> = Configuration::builder()
        .do_(initialization::RandomSpread::new_init(4 * 30))
        .evaluate()
        .update_best_individual()
        .do_(state::ParticleSwarm::initializer(1.))
        // Outer PSO loop.
        .while_(
            termination::LessThanN::<state::common::Iterations>::new(/*n: */ 10_000)
                & termination::DistanceToOptGreaterThan::new(0.01),
            |builder| {
                builder
                    .do_(generation::swarm::ParticleSwarmGeneration::new(
                        1., 1., 1., 1.,
                    ))
                    .do_(constraints.clone())
                    .evaluate()
                    .update_best_individual()
                    .do_(state::ParticleSwarm::updater())
                    // SA is run if the global best did not change.
                    .if_else_(
                        !GlobalBestImproved::new(),
                        |builder| {
                            builder
                                .do_(GlobalBestToPopulation::new())
                                .do_(sa::sa(
                                    sa::Parameters {
                                        t_0: 1.,
                                        generation: generation::mutation::GaussianMutation::new(
                                            1., 0.1,
                                        ),
                                        cooling_schedule: cooling_schedule.clone(),
                                        constraints,
                                    },
                                    termination::StepsWithoutImprovement::new(1),
                                ))
                                .do_(SingleToGlobalBest::new())
                        },
                        |builder| builder.do_(cooling_schedule.clone()),
                    )
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
