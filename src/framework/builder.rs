use crate::framework::components::{Block, Branch, Component, Condition, Loop, Scope};
use crate::framework::Configuration;
use crate::problems::Problem;

pub struct ConfigurationBuilder<P> {
    components: Vec<Box<dyn Component<P>>>,
}

impl<P: Problem + 'static> Default for ConfigurationBuilder<P> {
    fn default() -> Self {
        Self::new()
    }
}

impl<P: Problem + 'static> ConfigurationBuilder<P> {
    pub fn new() -> Self {
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
        Block::new(self.components)
    }
}
