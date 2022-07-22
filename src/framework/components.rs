//! Framework components.

pub use crate::framework::conditions::Condition;

use crate::{
    framework::{
        state::{common, State},
        Fitness,
    },
    problems::Problem,
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
pub trait Component<P: Problem>: AnyComponent {
    #[allow(unused_variables)]
    fn initialize(&self, problem: &P, state: &mut State) {}
    fn execute(&self, problem: &P, state: &mut State);
}
erased_serde::serialize_trait_object!(<P: Problem> Component<P>);

#[derive(Serialize)]
#[serde(bound = "")]
pub struct Scope<P: Problem> {
    body: Box<dyn Component<P>>,

    #[serde(skip)]
    init: fn(&mut State),
}

impl<P: Problem> Component<P> for Scope<P> {
    fn execute(&self, problem: &P, state: &mut State) {
        state.with_substate(|state| {
            (self.init)(state);
            self.body.initialize(problem, state);
            self.body.execute(problem, state);
        });
    }
}

impl<P: Problem> Scope<P> {
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
#[serde(transparent)]
pub struct Block<P: Problem>(Vec<Box<dyn Component<P>>>);

impl<P: Problem> Component<P> for Block<P> {
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

impl<P: Problem> Block<P> {
    pub fn new(components: Vec<Box<dyn Component<P>>>) -> Box<dyn Component<P>> {
        Box::new(Block(components))
    }
}

#[derive(Serialize)]
#[serde(bound = "")]
pub struct Loop<P: Problem> {
    #[serde(rename = "while")]
    condition: Box<dyn Condition<P>>,
    #[serde(rename = "do")]
    body: Box<dyn Component<P>>,
}

impl<P: Problem> Component<P> for Loop<P> {
    fn initialize(&self, problem: &P, state: &mut State) {
        state.insert(common::Iterations(0));

        self.condition.initialize(problem, state);
        self.body.initialize(problem, state);
    }

    fn execute(&self, problem: &P, state: &mut State) {
        self.condition.initialize(problem, state);
        while self.condition.evaluate(problem, state) {
            self.body.execute(problem, state);
            *state.get_value_mut::<common::Iterations>() += 1;
        }
    }
}

impl<P: Problem> Loop<P> {
    pub fn new(
        condition: Box<dyn Condition<P>>,
        body: Vec<Box<dyn Component<P>>>,
    ) -> Box<dyn Component<P>> {
        let body = Block::new(body);
        Box::new(Loop { condition, body })
    }
}

#[derive(Serialize)]
#[serde(bound = "")]
pub struct Branch<P: Problem> {
    condition: Box<dyn Condition<P>>,
    if_body: Box<dyn Component<P>>,
    else_body: Option<Box<dyn Component<P>>>,
}

impl<P: Problem> Component<P> for Branch<P> {
    fn initialize(&self, problem: &P, state: &mut State) {
        self.condition.initialize(problem, state);
        self.if_body.initialize(problem, state);
        if let Some(else_body) = &self.else_body {
            else_body.initialize(problem, state);
        }
    }

    fn execute(&self, problem: &P, state: &mut State) {
        if self.condition.evaluate(problem, state) {
            self.if_body.execute(problem, state);
        } else if let Some(else_body) = &self.else_body {
            else_body.execute(problem, state);
        }
    }
}

impl<P: Problem + 'static> Branch<P> {
    pub fn new(
        condition: Box<dyn Condition<P>>,
        body: Vec<Box<dyn Component<P>>>,
    ) -> Box<dyn Component<P>> {
        let if_body = Block::new(body);
        Box::new(Branch {
            condition,
            if_body,
            else_body: None,
        })
    }

    pub fn new_with_else(
        condition: Box<dyn Condition<P>>,
        if_body: Vec<Box<dyn Component<P>>>,
        else_body: Vec<Box<dyn Component<P>>>,
    ) -> Box<dyn Component<P>> {
        let if_body = Block::new(if_body);
        let else_body = Some(Block::new(else_body));
        Box::new(Branch {
            condition,
            if_body,
            else_body,
        })
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
        state.require::<common::Population>();
        state.insert(common::Evaluations(0));
        state.insert(common::BestFitness(Fitness::default()));
        state.insert(common::BestIndividual(None));
    }

    fn execute(&self, problem: &P, state: &mut State) {
        let mut mut_state = state.get_states_mut();

        // Evaluate population
        let population = mut_state.population_stack_mut();

        if population.is_empty() {
            return;
        }

        for individual in population.current_mut().iter_mut() {
            let solution = individual.solution::<P::Encoding>();
            let fitness = Fitness::try_from(problem.evaluate(solution)).unwrap();
            individual.evaluate(fitness);
        }

        // Update best fitness and individual
        let best_individual = population.best();

        if mut_state
            .get_mut::<common::BestIndividual>()
            .replace_if_better(best_individual)
        {
            mut_state.set_value::<common::BestFitness>(best_individual.fitness());
        }

        // Update evaluations
        *mut_state.get_value_mut::<common::Evaluations>() += population.current().len() as u32;
    }
}
