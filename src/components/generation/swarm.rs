//! Swarm Operators

use crate::state::PsoState;
use rand::distributions::Uniform;
use rand::Rng;

use crate::{
    framework::{components::*, Individual, Random},
    problems::SingleObjectiveProblem,
    state::State,
};

/// Applies the PSO specific generation operator.
///
/// Requires [PsoStateUpdate][crate::heuristics::pso::pso_ops::PsoStateUpdate].
#[derive(serde::Serialize, Clone)]
pub struct PsoGeneration {
    /// Inertia weight for influence of old velocity
    pub weight: f64,
    /// First constant factor for influence of previous best (also called Acceleration coefficient 1)
    pub c_one: f64,
    /// Second constant factor for influence of global best (also called Acceleration coefficient 2)
    pub c_two: f64,
    /// Maximum velocity
    pub v_max: f64,
}
impl PsoGeneration {
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
impl<P> Component<P> for PsoGeneration
where
    P: SingleObjectiveProblem<Encoding = Vec<f64>>,
{
    fn initialize(&self, _problem: &P, state: &mut State) {
        state.require::<PsoState<P>>();
    }

    fn execute(&self, _problem: &P, state: &mut State) {
        let &Self {
            weight,
            c_one,
            c_two,
            v_max,
        } = self;

        let mut offspring = Vec::new();
        let mut parents = state.population_stack_mut::<P>().pop();

        let rng = state.random_mut();
        let rng_iter = |rng: &mut Random| {
            rng.sample_iter(Uniform::new(0., 1.))
                .take(parents.len())
                .collect::<Vec<_>>()
        };

        // it might be debatable if one should use a vector of different random numbers or of the same
        // both versions exist in the literature
        let r_one = rng_iter(rng);
        let r_two = rng_iter(rng);

        let pso_state = state.get_mut::<PsoState<P>>();

        for (i, x) in parents.drain(..).enumerate() {
            let mut x = x.into_solution();
            let v = &mut pso_state.velocities[i];
            let xp = pso_state.bests[i].solution();
            let xg = pso_state.global_best.solution();

            for i in 0..v.len() {
                v[i] = weight * v[i]
                    + c_one * r_one[i] * (xp[i] - x[i])
                    + c_two * r_two[i] * (xg[i] - x[i]);
                v[i] = v[i].clamp(-v_max, v_max);
            }

            for i in 0..x.len() {
                //TODO we will need constraint handling here
                x[i] = (x[i] + v[i]).clamp(-1.0, 1.0);
            }

            offspring.push(Individual::<P>::new_unevaluated(x));
        }

        state.population_stack_mut().push(offspring);
    }
}
