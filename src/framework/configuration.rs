use crate::{
    framework::components::{Block, Branch, Component, Condition, Loop, Scope},
    problems::Problem,
};

pub struct Configuration<P: Problem>(Box<dyn Component<P>>);

impl<P: Problem> Configuration<P> {
    pub fn new(heuristic: Box<dyn Component<P>>) -> Self {
        Configuration(heuristic)
    }

    pub fn builder() -> ConfigurationBuilder<P> {
        ConfigurationBuilder::new()
    }

    pub fn heuristic(&self) -> &dyn Component<P> {
        self.0.as_ref()
    }
}

pub struct ConfigurationBuilder<P: Problem> {
    components: Vec<Box<dyn Component<P>>>,
}

impl<P: Problem> ConfigurationBuilder<P> {
    fn new() -> Self {
        Self {
            components: Vec::new(),
        }
    }

    pub fn do_(mut self, component: Box<dyn Component<P>>) -> Self {
        self.components.push(component);
        self
    }

    pub fn while_(
        self,
        condition: Box<dyn Condition<P>>,
        body: impl FnOnce(ConfigurationBuilder<P>) -> ConfigurationBuilder<P>,
    ) -> Self {
        let components = body(ConfigurationBuilder::new()).components;
        self.do_(Loop::new(condition, components))
    }

    pub fn if_(
        self,
        condition: Box<dyn Condition<P>>,
        body: impl FnOnce(ConfigurationBuilder<P>) -> ConfigurationBuilder<P>,
    ) -> Self {
        let components = body(ConfigurationBuilder::new()).components;
        self.do_(Branch::new(condition, components))
    }

    pub fn if_else_(
        self,
        condition: Box<dyn Condition<P>>,
        if_body: impl FnOnce(ConfigurationBuilder<P>) -> ConfigurationBuilder<P>,
        else_body: impl FnOnce(ConfigurationBuilder<P>) -> ConfigurationBuilder<P>,
    ) -> Self {
        let if_components = if_body(ConfigurationBuilder::new()).components;
        let else_components = else_body(ConfigurationBuilder::new()).components;
        self.do_(Branch::new_with_else(
            condition,
            if_components,
            else_components,
        ))
    }

    pub fn scope_(
        self,
        body: impl FnOnce(ConfigurationBuilder<P>) -> ConfigurationBuilder<P>,
    ) -> Self {
        let components = body(ConfigurationBuilder::new()).components;
        self.do_(Scope::new(components))
    }

    pub fn build(self) -> Configuration<P> {
        Configuration::new(Block::new(self.components))
    }
}
