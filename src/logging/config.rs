use better_any::{Tid, TidAble};
use derivative::Derivative;
use serde::Serialize;

use crate::{
    component::ExecResult,
    logging::{extractor::EntryExtractor, log::Step},
    state::{
        common,
        lens::common::{IdLens, ValueOf},
    },
    Condition, CustomState, Problem, State,
};

#[derive(Tid, Derivative)]
#[derivative(Default(bound = ""), Clone(bound = ""))]
pub struct LogConfig<P: Problem + 'static> {
    triggers: Vec<Box<dyn Condition<P>>>,
    extractors: Vec<Box<dyn EntryExtractor<P>>>,
}

impl<P: Problem> CustomState<'_> for LogConfig<P> {}

impl<P: Problem> LogConfig<P> {
    pub fn new() -> Self {
        Self {
            triggers: Vec::new(),
            extractors: Vec::new(),
        }
    }

    fn push(&mut self, trigger: Box<dyn Condition<P>>, extractor: Box<dyn EntryExtractor<P>>) {
        self.triggers.push(trigger);
        self.extractors.push(extractor);
    }

    pub fn triggers(&self) -> &[Box<dyn Condition<P>>] {
        &self.triggers
    }

    pub fn extractors(&self) -> &[Box<dyn EntryExtractor<P>>] {
        &self.extractors
    }

    pub fn with(
        mut self,
        trigger: Box<dyn Condition<P>>,
        extractor: Box<dyn EntryExtractor<P>>,
    ) -> Self {
        self.push(trigger, extractor);
        self
    }

    pub fn with_many(
        mut self,
        trigger: Box<dyn Condition<P>>,
        extractors: impl IntoIterator<Item = Box<dyn EntryExtractor<P>>>,
    ) -> Self {
        for extractor in extractors {
            self.push(trigger.clone(), extractor);
        }
        self
    }

    pub fn with_auto<T>(mut self, trigger: Box<dyn Condition<P>>) -> Self
    where
        T: for<'a> CustomState<'a> + Clone + Serialize + 'static,
    {
        self.push(trigger, Box::<IdLens<T>>::default());
        self
    }

    pub fn with_common(self, trigger: Box<dyn Condition<P>>) -> Self {
        self.with_auto::<common::Evaluations>(trigger.clone())
            .with_auto::<common::Progress<ValueOf<common::Iterations>>>(trigger)
    }

    pub(crate) fn execute(
        &self,
        problem: &P,
        state: &mut State<P>,
        step: &mut Step,
    ) -> ExecResult<()> {
        for (trigger, extractor) in self.triggers().iter().zip(self.extractors()) {
            if trigger.evaluate(problem, state)? {
                step.push(extractor.extract_entry(state))
            }
        }

        Ok(())
    }
}
