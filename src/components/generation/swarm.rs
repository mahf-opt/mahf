//! Swarm Operators

use better_any::Tid;
use itertools::multizip;
use rand::Rng;

use derive_more::{Deref, DerefMut};

use crate::{
    components::Component,
    framework::PopulationExtensions,
    problems::SingleObjectiveProblem,
    state::{ParticleSwarm, State},
    CustomState,
};

/// Inertia weight for influence of old velocity.
#[derive(Deref, DerefMut, Tid)]
pub struct Weight(pub f64);

impl CustomState<'_> for Weight {}

/// Applies the PSO specific generation operator.
///
/// Requires [PsoStateUpdate][PsoState::updater].
#[derive(serde::Serialize, Clone)]
pub struct ParticleSwarmGeneration {
    /// Initial inertia weight for influence of old velocity.
    pub weight: f64,
    /// First constant factor for influence of previous best (also called Acceleration coefficient 1)
    pub c_one: f64,
    /// Second constant factor for influence of global best (also called Acceleration coefficient 2)
    pub c_two: f64,
    /// Maximum velocity
    pub v_max: f64,
}

impl ParticleSwarmGeneration {
    pub fn new<P>(weight: f64, c_one: f64, c_two: f64, v_max: f64) -> Box<dyn Component<P>>
    where
        P: SingleObjectiveProblem<Encoding = Vec<f64>>,
    {
        Box::new(Self {
            weight,
            c_one,
            c_two,
            v_max,
        })
    }
}

impl<P> Component<P> for ParticleSwarmGeneration
where
    P: SingleObjectiveProblem<Encoding = Vec<f64>>,
{
    fn initialize(&self, _problem: &P, state: &mut State<P>) {
        state.insert(Weight(self.weight));
    }

    fn require(&self, _problem: &P, state: &State<P>) {
        state.require::<Self, ParticleSwarm<P>>();
    }

    fn execute(&self, _problem: &P, state: &mut State<P>) {
        let &Self {
            c_one,
            c_two,
            v_max,
            ..
        } = self;

        let mut mut_state = state.get_states_mut();
        let population = mut_state.populations_mut().current_mut();
        let rng = mut_state.random_mut();

        let ParticleSwarm {
            velocities: vs,
            bests,
            global_best,
        } = mut_state.get_mut::<ParticleSwarm<P>>();

        let w = mut_state.get_value::<Weight>();

        let mut rand = move || rng.gen::<f64>();

        let xs = population.solutions_mut();
        let xps = bests.solutions();
        let xg = global_best.solution();

        for (x, v, xp) in multizip((xs, vs, xps)) {
            // Update and clamp velocity
            for i in 0..v.len() {
                v[i] = w * v[i] + c_one * rand() * (xp[i] - x[i]) + c_two * rand() * (xg[i] - x[i]);
                v[i] = v[i].clamp(-v_max, v_max);
            }

            // Add velocity to particle position
            for i in 0..x.len() {
                x[i] += v[i];
            }
        }
    }
}
