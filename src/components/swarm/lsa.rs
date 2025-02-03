use rand::distributions::{Distribution, Uniform};
use serde::Serialize;

use crate::{
    component::ExecResult,
    components::Component,
    identifier::{Global, Identifier, PhantomId},
    population::{AsSolutionsMut, BestIndividual},
    problems::LimitedVectorProblem,
    SingleObjectiveProblem, State,
};


/// Updates the positions of particles according to the negatively charged stepped ladder mechanism
/// proposed for the Lightning Search Algorithm (LSA).
#[derive(Clone, Serialize)]
pub struct NegativelyChargedSteppedLadder<I: Identifier = Global> {
    id: PhantomId<I>,
}

impl<I: Identifier> NegativelyChargedSteppedLadder<I> {
    pub fn from_params() -> Self {
        Self {
            id: PhantomId::default(),
        }
    }

    pub fn new_with_id<P>() -> Box<dyn Component<P>>
    where
        P: SingleObjectiveProblem + LimitedVectorProblem<Element = f64>,
    {
        Box::new(Self::from_params())
    }
}

impl NegativelyChargedSteppedLadder<Global> {
    pub fn new<P>() -> Box<dyn Component<P>>
    where
        P: SingleObjectiveProblem + LimitedVectorProblem<Element = f64>,
    {
        Self::new_with_id()
    }
}

impl<P, I> Component<P> for NegativelyChargedSteppedLadder<I>
where
    P: SingleObjectiveProblem + LimitedVectorProblem<Element = f64>,
    I: Identifier,
{
    fn init(&self, _problem: &P, _state: &mut State<P>) -> ExecResult<()> {
        Ok(())
    }

    fn execute(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
        Ok(())
    }
}