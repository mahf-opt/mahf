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

#[derive(Derivative)]
#[derivative(Clone(bound = ""))]
pub struct ExtractionRule<P: Problem> {
    pub trigger: Box<dyn Condition<P>>,
    pub extractor: Box<dyn EntryExtractor<P>>,
}

#[derive(Tid, Derivative)]
#[derivative(Default(bound = ""), Clone(bound = ""))]
pub struct LogConfig<P: Problem + 'static> {
    rules: Vec<ExtractionRule<P>>,
}

impl<P: Problem> CustomState<'_> for LogConfig<P> {}

impl<P: Problem> LogConfig<P> {
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    fn push(&mut self, trigger: Box<dyn Condition<P>>, extractor: Box<dyn EntryExtractor<P>>) {
        self.rules.push(ExtractionRule { trigger, extractor })
    }

    pub fn triggers(&self) -> impl Iterator<Item = &Box<dyn Condition<P>>> {
        self.rules
            .iter()
            .map(|ExtractionRule { trigger, .. }| trigger)
    }

    pub fn with(
        &mut self,
        trigger: Box<dyn Condition<P>>,
        extractor: Box<dyn EntryExtractor<P>>,
    ) -> &mut Self {
        self.push(trigger, extractor);
        self
    }

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

    pub fn with_auto<T>(&mut self, trigger: Box<dyn Condition<P>>) -> &mut Self
    where
        T: for<'a> CustomState<'a> + Clone + Serialize + 'static,
    {
        self.push(trigger, Box::<IdLens<T>>::default());
        self
    }

    pub fn with_common(&mut self, trigger: Box<dyn Condition<P>>) -> &mut Self {
        self.with_auto::<common::Evaluations>(trigger.clone())
            .with_auto::<common::Progress<ValueOf<common::Iterations>>>(trigger)
    }

    pub(crate) fn execute(
        &self,
        problem: &P,
        state: &mut State<P>,
        step: &mut Step,
    ) -> ExecResult<()> {
        for ExtractionRule { trigger, extractor } in self.rules.iter() {
            if trigger.evaluate(problem, state)? {
                step.push(extractor.extract_entry(problem, state))
            }
        }

        Ok(())
    }

    pub fn clear(&mut self) {
        self.rules.clear()
    }
}
