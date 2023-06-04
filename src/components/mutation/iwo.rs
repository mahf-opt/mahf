use std::ops::Range;

use derivative::Derivative;
use eyre::ensure;
use serde::Serialize;

use crate::{
    component::ExecResult,
    components::{
        mapping::common::Polynomial,
        mutation::common::{MutationStrength, NormalMutation},
        Component,
    },
    identifier::{Global, Identifier},
    problems::VectorProblem,
    state::{common::Progress, extract::common::ValueOf, StateReq},
    State,
};

#[derive(Serialize, Derivative)]
#[serde(bound = "")]
#[derivative(Clone(bound = ""))]
pub struct IWOAdaptiveDeviation<T: 'static, I: Identifier = Global> {
    mutation: NormalMutation<I>,
    adaptation: Polynomial<ValueOf<Progress<T>>, ValueOf<MutationStrength<NormalMutation<I>>>>,
}

impl<T, I> IWOAdaptiveDeviation<T, I>
where
    T: 'static,
    I: Identifier,
{
    pub fn from_params(std_dev_range: Range<f64>, modulation_index: u32) -> ExecResult<Self> {
        ensure!(
            !std_dev_range.is_empty(),
            "the std_dev range must not be empty for this operator"
        );
        Ok(Self {
            mutation: <NormalMutation<I>>::from_params(std_dev_range.start, 1.),
            adaptation: Polynomial::from_params(
                std_dev_range.start,
                std_dev_range.end,
                modulation_index as f64,
            ),
        })
    }

    pub fn new<P>(
        std_dev_range: Range<f64>,
        modulation_index: u32,
    ) -> ExecResult<Box<dyn Component<P>>>
    where
        P: VectorProblem<Element = f64>,
    {
        Ok(Box::new(Self::from_params(
            std_dev_range,
            modulation_index,
        )?))
    }
}

impl<P, T, I> Component<P> for IWOAdaptiveDeviation<T, I>
where
    P: VectorProblem<Element = f64>,
    I: Identifier,
{
    fn init(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        self.mutation.init(problem, state)?;
        self.adaptation.init(problem, state)?;
        Ok(())
    }

    fn require(&self, problem: &P, state_req: &StateReq) -> ExecResult<()> {
        self.mutation.require(problem, state_req)?;
        self.adaptation.require(problem, state_req)?;
        Ok(())
    }

    fn execute(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        self.mutation.execute(problem, state)?;
        self.adaptation.execute(problem, state)?;
        Ok(())
    }
}
