use better_any::{Tid, TidAble};
use rand::Rng;

use crate::{
    components::Component,
    framework::Individual,
    problems::{LimitedVectorProblem, Problem, SingleObjectiveProblem},
    state::{CustomState, State},
};

/// State required for PSO.
///
/// For preserving velocities of particles, own best values and global best particle.
#[derive(Tid)]
pub struct PsoState<P: Problem + 'static> {
    pub velocities: Vec<Vec<f64>>,
    pub bests: Vec<Individual<P>>,
    pub global_best: Individual<P>,
}

impl<P: Problem> CustomState<'_> for PsoState<P> {}

impl<P: Problem> PsoState<P>
where
    P: SingleObjectiveProblem<Encoding = Vec<f64>> + LimitedVectorProblem<Element = f64>,
{
    /// State initialization for PSO.
    ///
    /// Initializes velocities, best found solutions of particles and global best in [PsoState].
    pub fn initializer(v_max: f64) -> Box<dyn Component<P>> {
        #[derive(Debug, serde::Serialize, Clone)]
        pub struct PsoStateInitialization {
            v_max: f64,
        }

        impl<P> Component<P> for PsoStateInitialization
        where
            P: SingleObjectiveProblem<Encoding = Vec<f64>> + LimitedVectorProblem<Element = f64>,
        {
            fn initialize(&self, _problem: &P, state: &mut State<P>) {
                // Initialize with empty state to satisfy `state.require()` statements
                state.insert(PsoState {
                    velocities: vec![],
                    bests: vec![],
                    global_best: Individual::<P>::new_unevaluated(Vec::new()),
                })
            }

            fn execute(&self, problem: &P, state: &mut State<P>) {
                let population = state.populations_mut().pop();
                let rng = state.random_mut();

                let velocities = std::iter::repeat_with(|| {
                    std::iter::repeat_with(|| rng.gen_range(-self.v_max..=self.v_max))
                        .take(problem.dimension())
                        .collect::<Vec<_>>()
                })
                .take(population.len())
                .collect::<Vec<_>>();

                let bests = population.to_vec();

                let global_best = bests
                    .iter()
                    .min_by_key(|i| Individual::objective(i))
                    .cloned()
                    .unwrap();

                state.populations_mut().push(population);

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
        #[derive(Debug, serde::Serialize, Clone)]
        pub struct PsoStateUpdate;

        impl<P> Component<P> for PsoStateUpdate
        where
            P: Problem<Encoding = Vec<f64>> + LimitedVectorProblem<Element = f64>,
        {
            fn initialize(&self, _problem: &P, state: &mut State<P>) {
                state.require::<Self, PsoState<P>>();
            }

            fn execute(&self, _problem: &P, state: &mut State<P>) {
                let population = state.populations_mut().pop();

                let PsoState {
                    bests, global_best, ..
                } = &mut state.get_mut::<PsoState<P>>();

                for (individual, best) in population.iter().zip(bests.iter_mut()) {
                    if best.objective() > individual.objective() {
                        *best = individual.clone();

                        if global_best.objective() > individual.objective() {
                            *global_best = individual.clone();
                        }
                    }
                }

                state.populations_mut().push(population);
            }
        }

        Box::new(PsoStateUpdate)
    }
}
