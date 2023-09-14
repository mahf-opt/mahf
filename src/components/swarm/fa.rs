use std::marker::PhantomData;

use better_any::{Tid, TidAble};
use derive_more::{Deref, DerefMut};
use eyre::{ensure, ContextCompat, WrapErr};
use itertools::multizip;
use rand::Rng;
use serde::Serialize;

use crate::{
    component::{AnyComponent, ExecResult},
    components::{Block, Component},
    identifier::{Global, Identifier, PhantomId},
    population::{AsSolutions, AsSolutionsMut, BestIndividual},
    problems::{LimitedVectorProblem, SingleObjectiveProblem},
    state::StateReq,
    CustomState, Individual, Problem, State,
};

/// Updates the and firefly positions.
///
/// Originally proposed for, and used as operator in [`fa`].
///
/// Uses the [`RandomizationParameter`].
///
/// [`fa`]: crate::heuristics::fa
#[derive(Clone, Serialize)]
pub struct FireflyPositionsUpdate<I: Identifier = Global> {
    pub alpha: f64,
    pub beta: f64,
    pub gamma: f64,
    id: PhantomId<I>,
}

impl<I: Identifier> FireflyPositionsUpdate<I> {
    pub fn from_params(alpha: f64, beta: f64, gamma: f64) -> Self {
        Self {
            alpha,
            beta,
            gamma,
            id: PhantomId::default(),
        }
    }

    pub fn new_with_id<P>(
        alpha: f64,
        beta: f64,
        gamma: f64,
    ) -> Box<dyn Component<P>>
        where
            P: LimitedVectorProblem<Element = f64>,
    {
        Box::new(Self::from_params(alpha, beta, gamma))
    }
}

impl FireflyPositionsUpdate<Global> {
    pub fn new<P>(alpha: f64, beta: f64, gamma: f64) -> Box<dyn Component<P>>
        where
            P: LimitedVectorProblem<Element = f64>,
    {
        Self::new_with_id(alpha, beta, gamma)
    }
}

impl<P, I> Component<P> for FireflyPositionsUpdate<I>
    where
        P: LimitedVectorProblem<Element = f64>,
        I: Identifier,
{
    fn init(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
        state.insert(RandomizationParameter(self.alpha));
        Ok(())
    }

    fn execute(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        let mut populations = state.populations_mut();
        let mut rng = state.random_mut();

        // Prepare parameters
        let &Self {
            beta, gamma, ..
        } = self;
        let a = state.get_value::<RandomizationParameter>();

        // Get necessary state
        let mut jj: Vec<Individual<P>> = populations.current().into_iter().cloned().collect();
        let mut ii: Vec<Individual<P>> = populations.current().into_iter().cloned().collect();
        let mut x: Vec<&mut Vec<f64>> = populations.current_mut().as_solutions_mut();

        // scale for adapting to problem domain; at the moment only if domain in each dim is the same
        let scale = (problem.domain()[0].end - problem.domain()[0].start).abs();

        // Perform the update step.
        // compare all individuals
        for (u, i) in  ii.iter_mut().enumerate() {
            for j in jj.iter_mut() {
                // if individual j is "more attractive" (i.e. has lower fitness), move towards j
                if i.get_objective() > j.get_objective() {
                    // draw random values from uniform distribution between 0 and 1
                    // according to paper: also possible to use normal distribution, depending on problem
                    let rand: Vec<f64> = (0..problem.dimension()).map(|_| rng.gen_range(0.0..1.0)).collect();
                    // calculate distance between i and j; without .sqrt() as it has to be squared again in the next step
                    let r = i.solution_mut()
                        .into_iter()
                        .zip(j.solution_mut())
                        .map(|(p, q)| (p.clone() - q.clone()).powf(2.0))
                        .sum::<f64>();
                    // calculate "attractiveness"
                    let b = beta * (- gamma * r).exp();
                    // calculate difference of solutions j and i
                    let diff = i.solution_mut()
                        .into_iter()
                        .zip(j.solution_mut())
                        .map(|(p, q)| q.clone() - p.clone())
                        .collect::<Vec<f64>>();
                    // calculate values that should be added to current position
                    let pos = diff
                        .into_iter()
                        .zip(rand)
                        .map(|(p, q)| b * p + a * (q - 0.5) * scale)
                        .collect::<Vec<f64>>();
                    // Add values to firefly position
                    for s in 0..i.solution_mut().len() {
                        x[u][s] += pos[s];
                    }
                }
            }
        }
        Ok(())
    }
}

/// The randomization parameter used to update the firefly positions.
#[derive(Deref, DerefMut, Tid)]
pub struct RandomizationParameter(
    #[deref]
    #[deref_mut]
    pub f64,
);

impl CustomState<'_> for RandomizationParameter {}