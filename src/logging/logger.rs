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

/// A configurable logging component.
///
/// The `Logger` is configured using the [`LogConfig`] stored in the [`State`].
///
/// The logged state is stored in the [`Log`].
///
/// # Position
///
/// Note that what a `Logger ` logs depends on its position in a [`Configuration`] (as the position decides
/// at which time it is executed).
/// There is no restriction on multiple `Logger`s (even within the same loop), so it can be used
/// to track changes of state even within a single iteration.
///
/// [`Configuration`]: crate::Configuration
///
/// # Examples
///
/// Placing the logger at the end of the main loop:
///
/// ```
/// use mahf::prelude::*;
/// # fn condition<P: Problem>() -> Box<dyn Condition<P>> { unimplemented!() }
/// # fn component1<P: Problem>() -> Box<dyn Component<P>> { unimplemented!() }
/// # fn component2<P: Problem>() -> Box<dyn Component<P>> { unimplemented!() }
/// # fn component3<P: Problem>() -> Box<dyn Component<P>> { unimplemented!() }
/// # fn example<P: Problem>() -> Configuration<P> {
/// Configuration::builder()
///     .do_(component1())
///     .while_(condition(), |builder| {
///         builder
///             .do_(component2())
///             .do_(component3())
///             .do_(logging::Logger::new())
///     })
///     .build()
/// # }
/// ```
#[derive(Default, Clone, Serialize)]
pub struct Logger;

impl Logger {
    /// Creates a new `Logger`.
    #[allow(clippy::new_ret_no_self)]
    pub fn new<P: Problem>() -> Box<dyn Component<P>> {
        Box::new(Self)
    }
}

impl<P: Problem> Component<P> for Logger {
    fn init(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        if state.contains::<LogConfig<P>>() {
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
        if state.contains::<LogConfig<P>>() {
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
