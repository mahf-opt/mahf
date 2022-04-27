#![allow(unused_variables)]
#![allow(clippy::new_ret_no_self)]

//! Framework components.

use crate::{
    framework::{
        common_state::{BestFitness, Evaluations, Iterations, Population, Progress},
        state::State,
        CustomState, Fitness,
    },
    problems::Problem,
    tracking::{
        log::{LogEntry, LoggerFunction, LoggerSet},
        trigger::LoggerCriteria,
        Log,
    },
};
use serde::Serialize;
use std::any::Any;
use trait_set::trait_set;

trait_set! {
    pub trait AnyComponent = erased_serde::Serialize + Any + Send + Sync;
}

/// Defines the traits required by any component.
///
/// This will be implemented automatically for all structs satisfying the requirements.
///
/// # Any
/// All components must allow downcasting and thus require [Any].
///
/// # Serialize
/// [DynSerialize] allows serializing dynamic components for the purpose of logging.
///
/// # Send
/// Most of the time, execution should be multi threaded and having
/// components implement [Send] makes this much easier.
///
pub trait Component<P>: AnyComponent {
    #[allow(unused_variables)]
    fn initialize(&self, problem: &P, state: &mut State) {}
    fn execute(&self, problem: &P, state: &mut State);
}
erased_serde::serialize_trait_object!(<P: Problem> Component<P>);

pub trait Condition<P>: AnyComponent {
    #[allow(unused_variables)]
    fn initialize(&self, problem: &P, state: &mut State) {}
    fn evaluate(&self, problem: &P, state: &mut State) -> bool;
}
erased_serde::serialize_trait_object!(<P: Problem> Condition<P>);

pub type Configuration<P> = Box<dyn Component<P>>;

#[derive(Serialize)]
#[serde(bound = "")]
pub struct Scope<P: Problem> {
    body: Box<dyn Component<P>>,

    #[serde(skip)]
    init: fn(&mut State),
}

impl<P> Component<P> for Scope<P>
where
    P: Problem + 'static,
{
    fn execute(&self, problem: &P, state: &mut State) {
        state.with_substate(|state| {
            (self.init)(state);
            self.body.initialize(problem, state);
            self.body.execute(problem, state);
        });
    }
}

impl<P: Problem + 'static> Scope<P> {
    pub fn new(body: Vec<Box<dyn Component<P>>>) -> Box<dyn Component<P>> {
        Self::new_with(|_| {}, body)
    }

    pub fn new_with(
        init: fn(&mut State),
        body: Vec<Box<dyn Component<P>>>,
    ) -> Box<dyn Component<P>> {
        let body = Block::new(body);
        Box::new(Scope { body, init })
    }
}

#[derive(Serialize)]
#[serde(bound = "")]
pub struct Block<P: Problem>(Vec<Box<dyn Component<P>>>);

impl<P> Component<P> for Block<P>
where
    P: Problem + 'static,
{
    fn initialize(&self, problem: &P, state: &mut State) {
        for component in &self.0 {
            component.initialize(problem, state);
        }
    }

    fn execute(&self, problem: &P, state: &mut State) {
        for component in &self.0 {
            component.execute(problem, state);
        }
    }
}

impl<P: Problem + 'static> Block<P> {
    pub fn new(components: Vec<Box<dyn Component<P>>>) -> Box<dyn Component<P>> {
        Box::new(Block(components))
    }
}

#[derive(Serialize)]
#[serde(bound = "")]
pub struct Loop<P: Problem> {
    condition: Box<dyn Condition<P>>,
    body: Box<dyn Component<P>>,
}

impl<P> Component<P> for Loop<P>
where
    P: Problem + 'static,
{
    fn initialize(&self, problem: &P, state: &mut State) {
        self.condition.initialize(problem, state);
        self.body.initialize(problem, state);
    }

    fn execute(&self, problem: &P, state: &mut State) {
        self.condition.initialize(problem, state);
        while self.condition.evaluate(problem, state) {
            self.body.execute(problem, state);
        }
    }
}

impl<P: Problem + 'static> Loop<P> {
    pub fn new(
        condition: Box<dyn Condition<P>>,
        body: Vec<Box<dyn Component<P>>>,
    ) -> Box<dyn Component<P>> {
        let body = Block::new(body);
        Box::new(Loop { condition, body })
    }
}

#[derive(Serialize)]
pub struct SimpleEvaluator;

impl SimpleEvaluator {
    pub fn new<P: Problem>() -> Box<dyn Component<P>> {
        Box::new(Self)
    }
}

impl<P: Problem> Component<P> for SimpleEvaluator {
    fn initialize(&self, _problem: &P, state: &mut State) {
        state.require::<Population>();
    }

    fn execute(&self, problem: &P, state: &mut State) {
        let population = state.get_mut::<Population>().current_mut();

        for individual in population {
            let solution = individual.solution::<P::Encoding>();
            let fitness = Fitness::try_from(problem.evaluate(solution)).unwrap();
            individual.evaluate(fitness);
        }
    }
}

#[derive(Serialize)]
#[serde(bound = "")]
pub struct Logger<P: Problem> {
    criteria: Vec<Box<dyn Condition<P>>>,

    #[serde(skip)]
    loggers: Vec<LoggerFunction>,
}

impl<P: Problem + 'static> Logger<P> {
    pub fn builder() -> Self {
        Self {
            criteria: Vec::new(),
            loggers: Vec::new(),
        }
    }

    pub fn add_criteria(&mut self, criteria: Box<dyn Condition<P>>) {
        self.criteria.push(criteria);
    }

    pub fn with_criteria(mut self, criteria: Box<dyn Condition<P>>) -> Self {
        self.add_criteria(criteria);
        self
    }

    pub fn add_logger(&mut self, logger: LoggerFunction) {
        self.loggers.push(logger);
    }

    pub fn with_logger(mut self, logger: LoggerFunction) -> Self {
        self.add_logger(logger);
        self
    }

    pub fn with_auto_logger<T: CustomState + Clone + Serialize>(self) -> Self {
        self.with_logger(LoggerFunction::auto::<T>())
    }

    pub fn with_common_loggers(self) -> Self {
        self.with_auto_logger::<Iterations>()
            .with_auto_logger::<Evaluations>()
            .with_auto_logger::<BestFitness>()
            .with_auto_logger::<Progress>()
    }

    pub fn build(self) -> Box<dyn Component<P>> {
        Box::new(self)
    }
}

impl<P: Problem + 'static> Component<P> for Logger<P> {
    fn initialize(&self, problem: &P, state: &mut State) {
        for criteria in &self.criteria {
            criteria.initialize(problem, state);
        }
    }

    fn execute(&self, problem: &P, state: &mut State) {
        let criteria = self
            .criteria
            .iter()
            .map(|c| c.evaluate(problem, state))
            .any(|b| b); // normal any would be short-circuiting

        if criteria {
            let mut entry = LogEntry::default();

            for logger in &self.loggers {
                entry.state.push((logger.function)(state));
            }

            state.get_mut::<Log>().push(entry);
        }
    }
}
