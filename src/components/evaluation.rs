use color_eyre::Section;
use std::any::type_name;
use std::marker::PhantomData;

use derivative::Derivative;
use eyre::eyre;
use serde::Serialize;

use crate::{
    component::ExecResult,
    components::Component,
    population::BestIndividual,
    problems::{Evaluator, MultiObjectiveProblem, SingleObjectiveProblem},
    state::{common, StateReq},
    Problem, State,
};

#[derive(Serialize, Derivative)]
#[serde(bound = "")]
#[derivative(Clone(bound = ""))]
pub struct PopulationEvaluator<T: Evaluator>(PhantomData<fn() -> T>);

impl<T: Evaluator> PopulationEvaluator<T> {
    pub fn from_params() -> Self {
        Self(PhantomData)
    }

    pub fn new() -> Box<dyn Component<T::Problem>> {
        Box::new(Self::from_params())
    }
}

impl<P, T> Component<P> for PopulationEvaluator<T>
where
    P: Problem,
    T: Evaluator<Problem = P>,
{
    fn init(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
        state.insert(common::Evaluations(0));

        if !state.has::<T>() {
            state.insert(
                T::try_default()
                    .map_err(|_| eyre!("no default evaluator for this problem available"))
                    .with_suggestion(|| {
                        format!(
                            "either implement TryDefault for {} or insert the evaluator manually into the state beforehand",
                            type_name::<P>()
                        )
                    })?,
            );
        }
        Ok(())
    }

    fn require(&self, _problem: &P, state_req: &StateReq) -> ExecResult<()> {
        state_req.require::<Self, common::Populations<P>>()?;
        state_req.require::<Self, T>()?;
        Ok(())
    }

    fn execute(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        let population = state.populations_mut().try_pop();
        if let Some(mut population) = population {
            state.holding::<T>(|evaluator: &mut T, state| {
                evaluator.evaluate(problem, state, &mut population);
                Ok(())
            })?;
            *state.borrow_value_mut::<common::Evaluations>() += population.len() as u32;
            state.populations_mut().push(population);
        }
        Ok(())
    }
}

#[derive(Clone, Serialize)]
pub struct BestIndividualUpdate;

impl BestIndividualUpdate {
    pub fn from_params() -> Self {
        Self
    }

    pub fn new<P: SingleObjectiveProblem>() -> Box<dyn Component<P>> {
        Box::new(Self)
    }
}

impl<P: SingleObjectiveProblem> Component<P> for BestIndividualUpdate {
    fn init(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
        state.insert(common::BestIndividual::<P>::default());
        Ok(())
    }

    fn execute(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
        let populations = state.populations();
        let population = populations.current();
        let best = population.best_individual();

        if let Some(best) = best {
            state.borrow_mut::<common::BestIndividual<P>>().update(best);
        }
        Ok(())
    }
}

#[derive(Clone, Serialize)]
pub struct ParetoFrontUpdate;

impl ParetoFrontUpdate {
    pub fn from_params() -> Self {
        Self
    }

    pub fn new<P: MultiObjectiveProblem>() -> Box<dyn Component<P>> {
        Box::new(Self)
    }
}

impl<P: MultiObjectiveProblem> Component<P> for ParetoFrontUpdate {
    fn init(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
        state.insert(common::ParetoFront::<P>::default());
        Ok(())
    }

    fn execute(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
        let populations = state.populations();
        let mut front = state.pareto_front_mut();

        for individual in populations.current() {
            front.update(individual);
        }

        Ok(())
    }
}
