use crate::framework::components::{Block, Branch, Component, Condition, Loop};
use crate::framework::Configuration;
use crate::problems::Problem;

pub trait ComponentIntegration<P> {
    fn integrate(&mut self, component: Box<dyn Component<P>>);
}

pub struct HeuristicBuilder<P> {
    components: Vec<Box<dyn Component<P>>>,
}

impl<P> ComponentIntegration<P> for HeuristicBuilder<P> {
    fn integrate(&mut self, component: Box<dyn Component<P>>) {
        self.components.push(component);
    }
}

impl<P: Problem + 'static> HeuristicBuilder<P> {
    pub fn new() -> Self {
        Self {
            components: Vec::new(),
        }
    }

    pub fn do_(mut self, component: impl Component<P>) -> Self {
        self.components.push(Box::new(component));
        self
    }

    pub fn while_(self, condition: impl Condition<P>) -> LoopBuilder<P, Self> {
        LoopBuilder::new(condition, self)
    }

    pub fn if_(self, condition: impl Condition<P>) -> BranchBuilder<P, Self> {
        BranchBuilder::new(condition, self)
    }

    pub fn build(self) -> Configuration<P> {
        Block::new(self.components)
    }
}

pub struct LoopBuilder<P, B> {
    condition: Box<dyn Condition<P>>,
    body: Vec<Box<dyn Component<P>>>,
    parent: B,
}

impl<P, B> ComponentIntegration<P> for LoopBuilder<P, B> {
    fn integrate(&mut self, component: Box<dyn Component<P>>) {
        self.body.push(component);
    }
}

impl<P: Problem + 'static, B: ComponentIntegration<P>> LoopBuilder<P, B> {
    pub fn new(condition: impl Condition<P>, parent: B) -> Self {
        Self {
            condition: Box::new(condition),
            body: Vec::new(),
            parent,
        }
    }

    pub fn do_(mut self, component: impl Component<P>) -> Self {
        self.body.push(Box::new(component));
        self
    }

    pub fn while_(self, condition: impl Condition<P>) -> LoopBuilder<P, Self> {
        LoopBuilder::new(condition, self)
    }

    pub fn if_(self, condition: impl Condition<P>) -> BranchBuilder<P, Self> {
        BranchBuilder::new(condition, self)
    }

    pub fn while_end(self) -> B {
        let Self {
            condition,
            body,
            mut parent,
        } = self;
        let component = Loop::new(condition, body);
        parent.integrate(component);
        parent
    }
}

pub struct BranchBuilder<P, B> {
    condition: Box<dyn Condition<P>>,
    if_body: Vec<Box<dyn Component<P>>>,
    else_body: Option<Vec<Box<dyn Component<P>>>>,
    if_phase: bool,
    parent: B,
}

impl<P, B> ComponentIntegration<P> for BranchBuilder<P, B> {
    fn integrate(&mut self, component: Box<dyn Component<P>>) {
        if self.if_phase {
            self.if_body.push(component);
        } else {
            self.else_body.as_mut().unwrap().push(component);
        }
    }
}

impl<P: Problem + 'static, B: ComponentIntegration<P>> BranchBuilder<P, B> {
    pub fn new(condition: impl Condition<P>, parent: B) -> Self {
        Self {
            condition: Box::new(condition),
            if_body: Vec::new(),
            if_phase: true,
            else_body: None,
            parent,
        }
    }

    pub fn do_(mut self, component: impl Component<P>) -> Self {
        let component = Box::new(component);
        if self.if_phase {
            self.if_body.push(component);
        } else {
            self.else_body.as_mut().unwrap().push(component);
        }
        self
    }

    pub fn if_(self, condition: impl Condition<P>) -> BranchBuilder<P, Self> {
        BranchBuilder::new(condition, self)
    }

    pub fn else_(mut self) -> Self {
        self.if_phase = false;
        self
    }

    pub fn while_(self, condition: impl Condition<P>) -> LoopBuilder<P, Self> {
        LoopBuilder::new(condition, self)
    }

    pub fn if_end(self) -> B {
        let Self {
            condition,
            if_body,
            else_body,
            mut parent,
            ..
        } = self;

        let component = if let Some(else_body) = else_body {
            Branch::new_with_else(condition, if_body, else_body)
        } else {
            Branch::new(condition, if_body)
        };
        parent.integrate(component);
        parent
    }
}
