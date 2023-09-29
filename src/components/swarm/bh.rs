use rand::{
    distributions::{Distribution, Uniform},
};
use serde::Serialize;

use crate::{component::ExecResult, components::Component, identifier::{Global, Identifier, PhantomId}, problems::{LimitedVectorProblem}, State};
use crate::population::{AsSolutionsMut, BestIndividual};

/// Updates the positions in the black hole algorithm.
///
/// Originally proposed for, and used as operator in [`bh`].
/// The operator is similar to the [`ParticleVelocitiesUpdate`] in [`pso`].
/// Specifically, they are identical when for [`pso`]:
/// inertia weight = 0,
/// c_1 = 0,
/// c_2 = 1,
/// v_max = 1
///
/// [`bh`]: crate::heuristics::bh
#[derive(Clone, Serialize)]
pub struct BlackHoleParticlesUpdate<I: Identifier = Global> {
    id: PhantomId<I>,
}

impl<I: Identifier> BlackHoleParticlesUpdate<I> {
    pub fn from_params() -> Self {
        Self {
            id: PhantomId::default(),
        }
    }

    pub fn new_with_id<P>() -> Box<dyn Component<P>>
        where
            P: LimitedVectorProblem<Element = f64>,
    {
        Box::new(Self::from_params())
    }
}

impl BlackHoleParticlesUpdate<Global> {
    pub fn new<P>() -> Box<dyn Component<P>>
        where
            P: LimitedVectorProblem<Element = f64>,
    {
        Self::new_with_id()
    }
}

impl<P, I> Component<P> for BlackHoleParticlesUpdate<I>
    where
        P: LimitedVectorProblem<Element = f64>,
        I: Identifier,
{
    fn init(&self, _problem: &P, _state: &mut State<P>) -> ExecResult<()> {
        Ok(())
    }

    fn execute(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
        let mut distr = Uniform::new(0.0, 1.0);

        // Get necessary state like global best `xg`
        let best = state.populations().current().best_individual().cloned();
        let xg = best.unwrap().solution();
        let xs = state.populations_mut().current_mut().as_solutions_mut();

        // Perform the update step.
        for x in xs {
            for i in 0..x.len() {
                // Calculate change in position
                let pos = distr.sample(&mut *state.random_mut()) * (xg[i] - x[i]);
                // Add value to particle position
                x[i] += pos;
            }
        }
        Ok(())
    }
}