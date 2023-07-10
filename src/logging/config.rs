//! Logging configuration.

use better_any::{Tid, TidAble};
use derivative::Derivative;
use serde::Serialize;

use crate::{
    component::ExecResult,
    lens::common::{IdLens, ValueOf},
    logging::{extractor::EntryExtractor, log::Step},
    state::common,
    Condition, CustomState, Problem, State,
};

/// A container for an [`EntryExtractor`] with a corresponding trigger [`Condition`].
#[derive(Derivative)]
#[derivative(Clone(bound = ""))]
pub(crate) struct ExtractionRule<P: Problem> {
    pub trigger: Box<dyn Condition<P>>,
    pub extractor: Box<dyn EntryExtractor<P>>,
}

impl<P: Problem> ExtractionRule<P> {
    /// Adds the value extracted by `extractor` to the `Step` if the `trigger` evaluates to `true`.
    pub(crate) fn execute(
        &self,
        problem: &P,
        state: &mut State<P>,
        step: &mut Step,
    ) -> ExecResult<()> {
        if self.trigger.evaluate(problem, state)? {
            step.push(self.extractor.extract_entry(problem, state))
        }
        Ok(())
    }
}

/// A logging configuration for a [`Logger`].
///
/// It is most commonly used through the [`State::configure_log`] method.
///
/// # Usage
///
/// The `LogConfig` contains trigger [`Condition`]s, which are evaluated by the [`Logger`],
/// which then conditionally writes the an entry created by the [`EntryExtractor`] into the [`Log`].
///
/// [`Logger`]: crate::logging::Logger
/// [`Log`]: crate::logging::Log
///
/// # Examples
///
/// Configuring the [`Logger`] to log the best objective value every 10 iterations:
///
/// ```
/// # use mahf::{ExecResult, SingleObjectiveProblem, State};
/// use mahf::{conditions::EveryN, lens::common::BestObjectiveValueLens};
///
/// # fn example<P: SingleObjectiveProblem>(state: &mut State<P>) -> ExecResult<()> {
/// state.configure_log(|config| {
///     config.with(EveryN::iterations(10), BestObjectiveValueLens::entry());
///     Ok(())
/// })?;
/// # Ok(())
/// # }
/// ```
#[derive(Tid, Derivative)]
#[derivative(Default(bound = ""), Clone(bound = ""))]
pub struct LogConfig<P: Problem + 'static> {
    rules: Vec<ExtractionRule<P>>,
}

impl<P: Problem> CustomState<'_> for LogConfig<P> {}

impl<P: Problem> LogConfig<P> {
    /// Creates a new `LogConfig`.
    ///
    /// Note that [`State::configure_log`] is preferred over inserting the config manually.
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    fn push(&mut self, trigger: Box<dyn Condition<P>>, extractor: Box<dyn EntryExtractor<P>>) {
        self.rules.push(ExtractionRule { trigger, extractor })
    }

    /// Returns the trigger [`Condition`]s.
    pub fn triggers(&self) -> impl Iterator<Item = &Box<dyn Condition<P>>> {
        self.rules
            .iter()
            .map(|ExtractionRule { trigger, .. }| trigger)
    }

    /// Logs the value returned by `extractor` if the `trigger` evaluates to `true`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mahf::{ExecResult, SingleObjectiveProblem, State};
    /// use mahf::{conditions::EveryN, lens::common::BestObjectiveValueLens};
    ///
    /// # fn example<P: SingleObjectiveProblem>(state: &mut State<P>) -> ExecResult<()> {
    /// state.configure_log(|config| {
    ///     config.with(EveryN::iterations(10), BestObjectiveValueLens::entry());
    ///     Ok(())
    /// })?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn with(
        &mut self,
        trigger: Box<dyn Condition<P>>,
        extractor: Box<dyn EntryExtractor<P>>,
    ) -> &mut Self {
        self.push(trigger, extractor);
        self
    }

    /// Logs the values returned by the `extractors` if the `trigger` evaluates to `true`.
    ///
    /// This is equivalent to calling [`LogConfig::with`] with each `extractor` and the `condition`
    /// separately.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mahf::{ExecResult, SingleObjectiveProblem, State};
    /// use mahf::{
    ///     conditions::EveryN,
    ///     lens::{common::BestObjectiveValueLens, ValueOf},
    ///     state::common,
    /// };
    ///
    /// # fn example<P: SingleObjectiveProblem>(state: &mut State<P>) -> ExecResult<()> {
    /// state.configure_log(|config| {
    ///     config.with_many(
    ///         EveryN::iterations(10),
    ///         [
    ///             ValueOf::<common::Evaluations>::entry(),
    ///             BestObjectiveValueLens::entry(),
    ///         ],
    ///     );
    ///     Ok(())
    /// })?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn with_many(
        &mut self,
        trigger: Box<dyn Condition<P>>,
        extractors: impl IntoIterator<Item = Box<dyn EntryExtractor<P>>>,
    ) -> &mut Self {
        for extractor in extractors {
            self.push(trigger.clone(), extractor);
        }
        self
    }

    /// Logs the state `T` if the `trigger` evaluates to `true`.
    ///
    /// The state `T` is extracted using [`IdLens`].
    pub fn with_auto<T>(&mut self, trigger: Box<dyn Condition<P>>) -> &mut Self
    where
        T: for<'a> CustomState<'a> + Clone + Serialize + 'static,
    {
        self.push(trigger, Box::<IdLens<T>>::default());
        self
    }

    /// Logs common values if the `trigger` evaluates to `true`.
    ///
    /// Common values currently include:
    /// - The number of [`Evaluations`]
    /// - The [`Progress`] of the number of [`Iterations`]
    ///
    /// [`Evaluations`]: common::Evaluations
    /// [`Progress`]: common::Progress
    /// [`Iterations`]: common::Iterations
    pub fn with_common(&mut self, trigger: Box<dyn Condition<P>>) -> &mut Self {
        self.with_auto::<common::Evaluations>(trigger.clone())
            .with_auto::<common::Progress<ValueOf<common::Iterations>>>(trigger)
    }

    /// Executes all [`ExtractionRule`]s.
    pub(crate) fn execute(
        &self,
        problem: &P,
        state: &mut State<P>,
        step: &mut Step,
    ) -> ExecResult<()> {
        for rule in &self.rules {
            rule.execute(problem, state, step)?;
        }
        Ok(())
    }

    /// Removes all triggers and extractors.
    pub fn clear(&mut self) {
        self.rules.clear()
    }
}
