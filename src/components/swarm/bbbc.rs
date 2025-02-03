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


/// Updates the positions of particles according to the cyclic universe mechanism proposed for the
/// Big Bang - Big Crunch (BBBC) algorithm.
#[derive(Clone, Serialize)]
pub struct CyclicUniverseMechanism<I: Identifier = Global> {
    pub new_pop: usize,
    id: PhantomId<I>,
}

impl<I: Identifier> CyclicUniverseMechanism<I> {
    pub fn from_params(new_pop: usize) -> Self {
        Self {
            new_pop,
            id: PhantomId::default(),
        }
    }

    pub fn new_with_id<P>(new_pop: usize) -> Box<dyn Component<P>>
    where
        P: SingleObjectiveProblem + LimitedVectorProblem<Element = f64>,
    {
        Box::new(Self::from_params(new_pop))
    }
}

impl CyclicUniverseMechanism<Global> {
    pub fn new<P>(new_pop: usize) -> Box<dyn Component<P>>
    where
        P: SingleObjectiveProblem + LimitedVectorProblem<Element = f64>,
    {
        Self::new_with_id(new_pop)
    }
}

impl<P, I> Component<P> for CyclicUniverseMechanism<I>
where
    P: SingleObjectiveProblem + LimitedVectorProblem<Element = f64>,
    I: Identifier,
{
    fn init(&self, _problem: &P, _state: &mut State<P>) -> ExecResult<()> {
        Ok(())
    }

    fn execute(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
        // Calculate center of mass

        // Generate new candidate solutions (new_pop specifies how many)

        Ok(())
    }
}