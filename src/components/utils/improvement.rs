use better_any::{Tid, TidAble};
use derive_more::{Deref, DerefMut};
use serde::Serialize;

use crate::{
    component::ExecResult, components::Component, problems::SingleObjectiveProblem, CustomState,
    SingleObjective, State,
};

/// The number of steps performed without any improvement in the [`BestIndividual`].
///
/// [`BestIndividual`]: crate::state::common::BestIndividual
#[derive(Clone, Default, Deref, DerefMut, Serialize, Tid)]
pub struct StepsWithoutImprovement(pub u32);

impl CustomState<'_> for StepsWithoutImprovement {}

/// The objective value in the previous step.
#[derive(Clone, Default, Deref, DerefMut, Serialize, Tid)]
struct PreviousObjectiveValue(pub SingleObjective);

impl CustomState<'_> for PreviousObjectiveValue {}

/// Updates the [`StepsWithoutImprovement`].
#[derive(Clone, Serialize)]
pub struct StepsWithoutImprovementUpdate;

impl StepsWithoutImprovementUpdate {
    pub fn from_params() -> Self {
        Self
    }

    pub fn new<P: SingleObjectiveProblem>() -> Box<dyn Component<P>> {
        Box::new(Self)
    }
}

impl<P: SingleObjectiveProblem> Component<P> for StepsWithoutImprovementUpdate {
    fn init(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
        state.insert(StepsWithoutImprovement(0));
        state.insert(PreviousObjectiveValue::default());
        Ok(())
    }

    fn execute(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
        let previous = state.try_get_value::<PreviousObjectiveValue>()?;
        if let Some(current) = state.best_objective_value() {
            if current < previous {
                state.set_value::<StepsWithoutImprovement>(0);
            } else {
                *state.borrow_value_mut::<StepsWithoutImprovement>() += 1;
            }

            state.set_value::<PreviousObjectiveValue>(current);
        }

        Ok(())
    }
}
