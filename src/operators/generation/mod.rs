//! Generation methods

// Random Operators without state //
pub use crate::operators::initialization::RandomPermutation;
pub use crate::operators::initialization::RandomSpread;

pub mod mutation;
pub mod recombination;

pub mod swarm {
    use rand::distributions::Uniform;
    use rand::Rng;

    use crate::random::Random;
    use crate::{
        framework::{components::*, Individual, State},
        operators::custom_state::PsoState,
        problems::Problem,
    };

    /// Applies the PSO specific generation operator.
    ///
    /// Requires [PsoStateUpdate][crate::heuristics::pso::pso_ops::PsoStateUpdate].
    #[derive(serde::Serialize)]
    pub struct PsoGeneration {
        pub a: f64,
        pub b: f64,
        pub c: f64,
        pub v_max: f64,
    }
    impl PsoGeneration {
        pub fn new<P>(a: f64, b: f64, c: f64, v_max: f64) -> Box<dyn Component<P>>
        where
            P: Problem<Encoding = Vec<f64>>,
        {
            Box::new(Self { a, b, c, v_max })
        }
    }
    impl<P> Component<P> for PsoGeneration
    where
        P: Problem<Encoding = Vec<f64>>,
    {
        fn initialize(&self, _problem: &P, state: &mut State) {
            state.require::<PsoState>();
        }

        fn execute(&self, _problem: &P, state: &mut State) {
            let &Self { a, b, c, v_max } = self;

            let mut offspring = Vec::new();
            let mut parents = state.population_stack_mut().pop();

            let rng = state.random_mut();
            let rng_iter = |rng: &mut Random| {
                rng.sample_iter(Uniform::new(0., 1.))
                    .take(parents.len())
                    .collect::<Vec<_>>()
            };

            let rs = rng_iter(rng);
            let rt = rng_iter(rng);

            let pso_state = state.get_mut::<PsoState>();

            for (i, x) in parents.drain(..).enumerate() {
                let mut x = x.into_solution::<Vec<f64>>();
                let v = &mut pso_state.velocities[i];
                let xl = pso_state.bests[i].solution::<Vec<f64>>();
                let xg = pso_state.global_best.solution::<Vec<f64>>();

                for i in 0..v.len() {
                    v[i] = a * v[i] + b * rs[i] * (xg[i] - x[i]) + c * rt[i] * (xl[i] - x[i]);
                    v[i] = v[i].clamp(-v_max, v_max);
                }

                for i in 0..x.len() {
                    x[i] = (x[i] + v[i]).clamp(-1.0, 1.0);
                }

                offspring.push(Individual::new_unevaluated(x));
            }

            state.population_stack_mut().push(offspring);
        }
    }
}
