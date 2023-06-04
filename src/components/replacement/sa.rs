use better_any::{Tid, TidAble};
use derive_more::{Deref, DerefMut};
use eyre::ContextCompat;
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::{
    component::ExecResult, components::Component, problems::SingleObjectiveProblem, CustomState,
    State,
};

#[derive(Default, Tid, Deref, DerefMut)]
pub struct Temperature(pub f64);

impl CustomState<'_> for Temperature {}

#[derive(Clone, Serialize, Deserialize)]
pub struct ExponentialAnnealingAcceptance {
    t_0: f64,
}

impl ExponentialAnnealingAcceptance {
    pub fn new<P: SingleObjectiveProblem>(t_0: f64) -> Box<dyn Component<P>> {
        Box::new(Self { t_0 })
    }
}

impl<P: SingleObjectiveProblem> Component<P> for ExponentialAnnealingAcceptance {
    fn init(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
        state.insert(Temperature(self.t_0));
        Ok(())
    }

    #[contracts::invariant(state.populations().current().len() == 1, "population before and after should contain a single individual")]
    #[contracts::requires(state.populations().peek(1).len() == 1, "parent population should contain a single individual")]
    #[contracts::ensures(state.populations().len() == old(state.populations().len()) - 1)]
    fn execute(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
        let mut populations = state.populations_mut();

        let o_current = populations
            .peek(0)
            .first()
            .wrap_err("current solution is missing")?
            .objective();
        let o_candidate = populations
            .peek(1)
            .first()
            .wrap_err("candidate solution is missing")?
            .objective();

        let t = state.get_value::<Temperature>();
        let p = ((o_current.value() - o_candidate.value()) / t).exp();

        if o_candidate < o_current || state.random_mut().gen::<f64>() < p {
            let candidate = populations.pop();
            populations.pop();
            populations.push(candidate);
        } else {
            populations.pop();
        }

        Ok(())
    }
}
