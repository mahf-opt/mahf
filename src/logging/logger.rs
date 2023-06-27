//! Logging arbitrary data during a (meta)heuristic run.

use serde::Serialize;

use crate::{
    component::ExecResult,
    components::Component,
    logging::{
        config::LogConfig,
        log::{Log, Step},
    },
    Problem, State,
};

#[derive(Default, Clone, Serialize)]
pub struct Logger;

impl Logger {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<P: Problem>() -> Box<dyn Component<P>> {
        Box::new(Self)
    }
}

impl<P: Problem> Component<P> for Logger {
    fn init(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        if state.has::<LogConfig<P>>() {
            state.holding::<LogConfig<P>>(|config, state| {
                for trigger in config.triggers() {
                    trigger.init(problem, state)?;
                }
                Ok(())
            })?;
        }
        Ok(())
    }

    fn execute(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        if state.has::<LogConfig<P>>() {
            state.holding::<LogConfig<P>>(|config, state| {
                let mut step = Step::default();
                config.execute(problem, state, &mut step)?;
                if !step.entries().is_empty() {
                    step.push_iteration(state);
                    state.borrow_mut::<Log>().push(step);
                }
                Ok(())
            })?;
        }
        Ok(())
    }
}
