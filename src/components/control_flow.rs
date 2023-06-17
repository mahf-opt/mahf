use derivative::Derivative;
use serde::Serialize;

use crate::{
    component::ExecResult,
    components::Component,
    conditions::Condition,
    problems::Problem,
    state::{common, State, StateReq},
};

#[derive(Serialize, Derivative)]
#[serde(bound = "")]
#[serde(transparent)]
#[derivative(Clone(bound = ""))]
pub struct Block<P: Problem>(Vec<Box<dyn Component<P>>>);

impl<P: Problem> Block<P> {
    /// Creates a new [Block] from a collection of [Component]s.
    pub fn new(
        components: impl IntoIterator<Item = Box<dyn Component<P>>>,
    ) -> Box<dyn Component<P>> {
        Box::new(Self(components.into_iter().collect()))
    }
}

impl<P: Problem> Component<P> for Block<P> {
    fn init(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        for component in &self.0 {
            component.init(problem, state)?;
        }
        Ok(())
    }

    fn require(&self, problem: &P, state_req: &StateReq<P>) -> ExecResult<()> {
        for component in &self.0 {
            component.require(problem, state_req)?;
        }
        Ok(())
    }

    fn execute(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        for component in &self.0 {
            component.execute(problem, state)?;
        }
        Ok(())
    }
}

impl<I: IntoIterator<Item = Box<dyn Component<P>>>, P: Problem> From<I> for Box<dyn Component<P>> {
    fn from(value: I) -> Self {
        Block::new(value)
    }
}

#[derive(Serialize, Derivative)]
#[serde(bound = "")]
#[derivative(Clone(bound = ""))]
pub struct Loop<P: Problem> {
    #[serde(rename = "while")]
    condition: Box<dyn Condition<P>>,
    #[serde(rename = "do")]
    body: Box<dyn Component<P>>,
}

impl<P: Problem> Loop<P> {
    pub fn new(
        condition: Box<dyn Condition<P>>,
        body: impl Into<Box<dyn Component<P>>>,
    ) -> Box<dyn Component<P>> {
        Box::new(Loop {
            condition,
            body: body.into(),
        })
    }
}

impl<P: Problem> Component<P> for Loop<P> {
    fn init(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        state.insert(common::Iterations(0));

        self.condition.init(problem, state)?;
        self.body.init(problem, state)?;

        Ok(())
    }

    fn require(&self, problem: &P, state_req: &StateReq<P>) -> ExecResult<()> {
        self.condition.require(problem, state_req)?;
        self.body.require(problem, state_req)?;
        Ok(())
    }

    fn execute(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        self.condition.init(problem, state)?;
        while self.condition.evaluate(problem, state)? {
            self.body.execute(problem, state)?;
            *state.try_borrow_value_mut::<common::Iterations>()? += 1;
        }
        Ok(())
    }
}

#[derive(Serialize, Derivative)]
#[serde(bound = "")]
#[derivative(Clone(bound = ""))]
pub struct Branch<P: Problem> {
    condition: Box<dyn Condition<P>>,
    if_body: Box<dyn Component<P>>,
    else_body: Option<Box<dyn Component<P>>>,
}

impl<P: Problem> Branch<P> {
    pub fn new(
        condition: Box<dyn Condition<P>>,
        body: impl Into<Box<dyn Component<P>>>,
    ) -> Box<dyn Component<P>> {
        Box::new(Branch {
            condition,
            if_body: body.into(),
            else_body: None,
        })
    }

    pub fn new_with_else(
        condition: Box<dyn Condition<P>>,
        if_body: impl Into<Box<dyn Component<P>>>,
        else_body: impl Into<Box<dyn Component<P>>>,
    ) -> Box<dyn Component<P>> {
        Box::new(Branch {
            condition,
            if_body: if_body.into(),
            else_body: Some(else_body.into()),
        })
    }
}

impl<P: Problem> Component<P> for Branch<P> {
    fn init(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        self.condition.init(problem, state)?;
        self.if_body.init(problem, state)?;
        if let Some(else_body) = &self.else_body {
            else_body.init(problem, state)?;
        }
        Ok(())
    }

    fn require(&self, problem: &P, state_req: &StateReq<P>) -> ExecResult<()> {
        self.condition.require(problem, state_req)?;
        self.if_body.require(problem, state_req)?;
        if let Some(else_body) = &self.else_body {
            else_body.require(problem, state_req)?;
        }
        Ok(())
    }

    fn execute(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        if self.condition.evaluate(problem, state)? {
            self.if_body.execute(problem, state)?;
        } else if let Some(else_body) = &self.else_body {
            else_body.execute(problem, state)?;
        }
        Ok(())
    }
}

#[derive(Serialize, Derivative)]
#[serde(bound = "")]
#[derivative(Clone(bound = ""))]
pub struct Scope<P: Problem> {
    body: Box<dyn Component<P>>,
    #[serde(skip)]
    init: fn(&mut State<P>) -> ExecResult<()>,
}

impl<P: Problem> Scope<P> {
    pub fn new(body: Vec<Box<dyn Component<P>>>) -> Box<dyn Component<P>> {
        Self::new_with(|_| Ok(()), body)
    }

    /// Creates a new [Scope] while overriding some state.
    pub fn new_with(
        init: fn(&mut State<P>) -> ExecResult<()>,
        body: impl Into<Box<dyn Component<P>>>,
    ) -> Box<dyn Component<P>> {
        Box::new(Scope {
            body: body.into(),
            init,
        })
    }
}

impl<P: Problem> Component<P> for Scope<P> {
    fn execute(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        state.with_inner_state(|state| {
            (self.init)(state)?;
            self.body.init(problem, state)?;
            self.body.execute(problem, state)?;
            Ok(())
        })?;
        Ok(())
    }
}
