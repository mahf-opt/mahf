use std::array::from_mut;
use std::ops::Deref;

use better_any::{Tid, TidAble};
use derive_more::{Deref, DerefMut};
use itertools::{izip};
use rand::Rng;
use serde::Serialize;

use crate::{component::ExecResult, components::Component, identifier::{Global, Identifier, PhantomId}, problems::{LimitedVectorProblem}, CustomState, State};
use crate::state::common;
use crate::state::common::Evaluator;

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
        let mut individuals = populations.current_mut();
        let mut evaluator = state.borrow::<Evaluator<P>>();

        // scale for adapting to problem domain
        let scales = problem.domain()
            .iter()
            .map(|p| (p.end - p.start).abs())
            .collect::<Vec<f64>>();


        // Perform the update step.
        // compare all individuals
        for i in  0..individuals.len() {
            for j in 0..individuals.len() {
                // if individual j is "more attractive" (i.e. has lower fitness), move towards j
                if individuals[i].objective() > individuals[j].objective() {
                    // draw random values from uniform distribution between 0 and 1
                    // according to paper: also possible to use normal distribution, depending on problem
                    let rands: Vec<f64> = (0..problem.dimension()).map(|_| rng.gen_range(0.0..1.0)).collect();
                    let mut current = individuals[i].clone();
                    izip!(current.solution_mut(), individuals[j].solution(), &scales, rands)
                        .map(|(xi, xj, scale, rand)| {
                            // calculate "attractiveness"
                            //let b = ;
                            // calculate value that should be added to current position
                            let pos = beta * (-gamma * (*xi - xj).powf(2.0)).exp() * (xj - *xi) + a * (rand - 0.5) * scale;
                            (xi, pos) })
                        .for_each(|(xi, pos)| *xi += pos);
                    individuals[i] = current;

                    evaluator.evaluate(problem, state, from_mut(&mut individuals[i]));
                    *state.borrow_value_mut::<common::Evaluations>() += 1;
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