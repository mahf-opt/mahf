#![allow(clippy::new_ret_no_self)]

use crate::{
    framework::{
        components::Component,
        state::{self, StateTree},
    },
    problems::Problem,
};

pub type Configuration<P> = Box<dyn Component<P>>;

pub struct Block<P>(Vec<Box<dyn Component<P>>>);

impl<P> Component<P> for Block<P>
where
    P: Problem + 'static,
{
    fn execute(&self, problem: &P, state: &mut StateTree) {
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

pub struct Loop<P> {
    condition: Box<dyn Component<P>>,
    body: Box<dyn Component<P>>,
}

impl<P> Component<P> for Loop<P>
where
    P: Problem + 'static,
{
    fn initialize(&self, problem: &P, state: &mut StateTree) {
        self.condition.initialize(problem, state);
    }

    fn execute(&self, problem: &P, state: &mut StateTree) {
        state.with_substate(|state| {
            state.insert(state::common::Loop(true));
            self.body.initialize(problem, state);
            loop {
                self.condition.execute(problem, state);
                if !state.get::<state::common::Loop>().0 {
                    break;
                }
                self.body.execute(problem, state);
            }
        });
    }
}

impl<P: Problem + 'static> Loop<P> {
    pub fn new(
        condition: Box<dyn Component<P>>,
        body: Box<dyn Component<P>>,
    ) -> Box<dyn Component<P>> {
        Box::new(Loop { condition, body })
    }
}
