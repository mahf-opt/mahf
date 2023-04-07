use better_any::Tid;
use derive_more::{Deref, DerefMut};
use itertools::{multizip, Itertools};
use rand::Rng;

use crate::{
    components::Component,
    framework::{Individual, SingleObjective},
    problems::{LimitedVectorProblem, Problem, SingleObjectiveProblem},
    state::{CustomState, State},
};

/// State required for PSO.
///
/// For preserving velocities of particles, own best values and global best particle.
#[derive(Tid)]
pub struct ParticleSwarm<P: Problem + 'static> {
    pub velocities: Vec<Vec<f64>>,
    pub bests: Vec<Individual<P>>,
    pub global_best: Individual<P>,
}

impl<P: Problem> CustomState<'_> for ParticleSwarm<P> {}

#[derive(Deref, DerefMut, Tid)]
pub struct ImprovedParticlePercentage(pub f64);

impl CustomState<'_> for ImprovedParticlePercentage {}

#[derive(Deref, DerefMut, Tid)]
struct PreviousObjectiveValues(Vec<SingleObjective>);

impl CustomState<'_> for PreviousObjectiveValues {}

#[derive(Clone, serde::Serialize)]
pub struct UpdateImprovedParticlePercentage {
    pub delta: f64,
}

impl UpdateImprovedParticlePercentage {
    pub fn new<P: SingleObjectiveProblem>(delta: f64) -> Box<dyn Component<P>> {
        Box::new(Self { delta })
    }
}

impl<P: SingleObjectiveProblem> Component<P> for UpdateImprovedParticlePercentage {
    fn initialize(&self, _problem: &P, state: &mut State<P>) {
        state.insert(ImprovedParticlePercentage(0.));
        state.insert(PreviousObjectiveValues(Vec::new()));
    }

    fn execute(&self, _problem: &P, state: &mut State<P>) {
        let mut mut_state = state.get_states_mut();
        let ParticleSwarm { bests, .. } = mut_state.get_mut::<ParticleSwarm<P>>();

        let current = bests.iter().map(|i| *i.objective()).collect_vec();
        let previous = mut_state.get_value_mut::<PreviousObjectiveValues>();

        let n = bests.len() as f64;
        let percentage = current
            .iter()
            .zip(previous.iter_mut())
            .map(|(current, previous)| {
                (previous.value() - current.value() > self.delta) as u8 as f64
            })
            .sum::<f64>()
            / n;

        mut_state.set_value::<ImprovedParticlePercentage>(percentage);
        *previous = current;
    }
}

impl<P: Problem> ParticleSwarm<P>
where
    P: SingleObjectiveProblem<Encoding = Vec<f64>> + LimitedVectorProblem<T = f64>,
{
    /// State initialization for PSO.
    ///
    /// Initializes velocities, best found solutions of particles and global best in [ParticleSwarm].
    pub fn initializer(v_max: f64) -> Box<dyn Component<P>> {
        #[derive(Debug, serde::Serialize, Clone)]
        pub struct ParticleSwarmInitialization {
            v_max: f64,
        }

        impl<P> Component<P> for ParticleSwarmInitialization
        where
            P: SingleObjectiveProblem<Encoding = Vec<f64>> + LimitedVectorProblem<T = f64>,
        {
            fn initialize(&self, _problem: &P, state: &mut State<P>) {
                // Initialize with empty state to satisfy `state.require()` statements
                state.insert(ParticleSwarm {
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

                let global_best = bests.iter().min_by_key(|i| i.objective()).cloned().unwrap();

                state.populations_mut().push(population);

                state.insert(ParticleSwarm {
                    velocities,
                    bests,
                    global_best,
                });
            }
        }

        Box::new(ParticleSwarmInitialization { v_max })
    }

    /// State update for PSO.
    ///
    /// Updates best found solutions of particles and global best in [ParticleSwarm].
    pub fn updater() -> Box<dyn Component<P>> {
        #[derive(Debug, serde::Serialize, Clone)]
        pub struct ParticleSwarmUpdate;

        impl<P> Component<P> for ParticleSwarmUpdate
        where
            P: Problem<Encoding = Vec<f64>> + LimitedVectorProblem<T = f64>,
        {
            fn initialize(&self, _problem: &P, state: &mut State<P>) {
                state.require::<Self, ParticleSwarm<P>>();
            }

            fn execute(&self, _problem: &P, state: &mut State<P>) {
                let population = state.populations_mut().pop();

                let ParticleSwarm {
                    bests, global_best, ..
                } = state.get_mut::<ParticleSwarm<P>>();

                for (individual, best) in multizip((&population, bests)) {
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

        Box::new(ParticleSwarmUpdate)
    }
}
