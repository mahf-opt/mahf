use crate::{
    framework::{components::*, Individual},
    problems::{LimitedVectorProblem, Problem, SingleObjectiveProblem},
    state::{CustomState, State},
};
use rand::Rng;

/// State required for PSO.
///
/// For preserving velocities of particles, own best values and global best particle.
pub struct PsoState<P: Problem> {
    pub velocities: Vec<Vec<f64>>,
    pub bests: Vec<Individual<P>>,
    pub global_best: Individual<P>,
}

impl<P: Problem> CustomState for PsoState<P> {}

impl<P: Problem> PsoState<P>
where
    P: SingleObjectiveProblem<Encoding = Vec<f64>> + LimitedVectorProblem<T = f64>,
{
    /// State initialization for PSO.
    ///
    /// Initializes velocities, best found solutions of particles and global best in [PsoState].
    pub fn intializer(v_max: f64) -> Box<dyn Component<P>> {
        #[derive(Debug, serde::Serialize)]
        pub struct PsoStateInitialization {
            v_max: f64,
        }

        impl<P> Component<P> for PsoStateInitialization
        where
            P: SingleObjectiveProblem<Encoding = Vec<f64>> + LimitedVectorProblem<T = f64>,
        {
            fn initialize(&self, _problem: &P, state: &mut State) {
                // Initialize with empty state to satisfy `state.require()` statements
                state.insert(PsoState {
                    velocities: vec![],
                    bests: vec![],
                    global_best: Individual::<P>::new_unevaluated(Vec::new()),
                })
            }

            fn execute(&self, problem: &P, state: &mut State) {
                let population = state.population_stack_mut::<P>().pop();
                let rng = state.random_mut();

                let velocities = population
                    .iter()
                    .map(|_| {
                        (0..problem.dimension())
                            .into_iter()
                            .map(|_| rng.gen_range(-self.v_max..=self.v_max))
                            .collect::<Vec<f64>>()
                    })
                    .collect::<Vec<Vec<f64>>>();

                let bests = population.to_vec();

                let global_best = bests
                    .iter()
                    .min_by_key(|i| Individual::objective(i))
                    .cloned()
                    .unwrap();

                state.population_stack_mut().push(population);

                state.insert(PsoState {
                    velocities,
                    bests,
                    global_best,
                });
            }
        }

        Box::new(PsoStateInitialization { v_max })
    }

    /// State update for PSO.
    ///
    /// Updates best found solutions of particles and global best in [PsoState].
    pub fn updater() -> Box<dyn Component<P>> {
        #[derive(Debug, serde::Serialize)]
        pub struct PsoStateUpdate;

        impl<P> Component<P> for PsoStateUpdate
        where
            P: Problem<Encoding = Vec<f64>> + LimitedVectorProblem<T = f64>,
        {
            fn initialize(&self, _problem: &P, state: &mut State) {
                state.require::<PsoState<P>>();
            }

            fn execute(&self, _problem: &P, state: &mut State) {
                let population = state.population_stack_mut().pop();
                let mut pso_state = state.get_mut::<PsoState<P>>();

                for (i, individual) in population.iter().enumerate() {
                    if pso_state.bests[i].objective() > individual.objective() {
                        pso_state.bests[i] = individual.clone();

                        if pso_state.global_best.objective() > individual.objective() {
                            pso_state.global_best = individual.clone();
                        }
                    }
                }

                state.population_stack_mut().push(population);
            }
        }

        Box::new(PsoStateUpdate)
    }
}
