#![allow(unused_variables)]
#![allow(clippy::new_ret_no_self)]

//! Framework components.

use crate::{
    framework::{
        common_state::{BestFitness, Evaluations, Population},
        state::State,
        Fitness, Individual,
    },
    problems::Problem,
    tracking::logfn::LogSet,
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
        state.insert(Evaluations(0));
        state.insert(BestFitness(Fitness::default()))
    }

    fn execute(&self, problem: &P, state: &mut State) {
        let population = state.get_mut::<Population>().current_mut();

        for individual in population.iter_mut() {
            let solution = individual.solution::<P::Encoding>();
            let fitness = Fitness::try_from(problem.evaluate(solution)).unwrap();
            individual.evaluate(fitness);
        }

        let evaluations = population.len() as u32;
        let best_fitness = population
            .iter()
            .map(Individual::fitness)
            .min()
            .unwrap_or_default();
        let best_fitness = Ord::min(best_fitness, state.get_value::<BestFitness>());

        state.set_value::<BestFitness>(best_fitness);
        state.get_mut::<Evaluations>().0 += evaluations;
    }
}

#[derive(Serialize)]
#[serde(bound = "")]
pub struct Logger<P: Problem> {
    #[serde(skip)]
    sets: Vec<LogSet<P>>,
}

impl<P: Problem + 'static> Logger<P> {
    pub fn builder() -> Self {
        Logger { sets: Vec::new() }
    }

    pub fn with_set(mut self, set: LogSet<P>) -> Self {
        self.sets.push(set);
        self
    }

    pub fn build(self) -> Box<dyn Component<P>> {
        Box::new(self)
    }
}

impl<P: Problem + 'static> Component<P> for Logger<P> {
    fn initialize(&self, problem: &P, state: &mut State) {
        for set in &self.sets {
            for criteria in &set.criteria {
                criteria.initialize(problem, state);
            }
        }
    }

    fn execute(&self, problem: &P, state: &mut State) {
        for set in &self.sets {
            set.execute(problem, state);
        }
    }
}
