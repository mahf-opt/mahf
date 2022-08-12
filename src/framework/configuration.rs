use crate::{
    framework::{
        components::{Block, Branch, Component, Loop, Scope},
        conditions::Condition,
    },
    problems::Problem,
};

/// A heuristic, constructed from a set of components.
pub struct Configuration<P: Problem>(Box<dyn Component<P>>);

impl<P: Problem> Configuration<P> {
    /// Wraps a heuristic into a `Configuration`.
    ///
    /// Use `Configuration::builder` for a more convenient construction.
    pub fn new(heuristic: Box<dyn Component<P>>) -> Self {
        Configuration(heuristic)
    }

    /// Creates a `ConfigurationBuilder`.
    pub fn builder() -> ConfigurationBuilder<P> {
        ConfigurationBuilder::new()
    }

    /// Returns the root component.
    pub fn heuristic(&self) -> &dyn Component<P> {
        self.0.as_ref()
    }
}

impl<P: Problem> From<Configuration<P>> for Box<dyn Component<P>> {
    fn from(config: Configuration<P>) -> Self {
        config.0
    }
}

/// A simple DSL for building a heuristic.
pub struct ConfigurationBuilder<P: Problem> {
    components: Vec<Box<dyn Component<P>>>,
}

impl<P: Problem> ConfigurationBuilder<P> {
    fn new() -> Self {
        Self {
            components: Vec::new(),
        }
    }

    /// Adds a sequential component.
    pub fn do_(mut self, component: Box<dyn Component<P>>) -> Self {
        self.components.push(component);
        self
    }

    /// Runs the body while the condition is true.
    pub fn while_(
        self,
        condition: Box<dyn Condition<P>>,
        body: impl FnOnce(ConfigurationBuilder<P>) -> ConfigurationBuilder<P>,
    ) -> Self {
        let components = body(ConfigurationBuilder::new()).components;
        self.do_(Loop::new(condition, components))
    }

    /// Runs the body if the condition is true.
    pub fn if_(
        self,
        condition: Box<dyn Condition<P>>,
        body: impl FnOnce(ConfigurationBuilder<P>) -> ConfigurationBuilder<P>,
    ) -> Self {
        let components = body(ConfigurationBuilder::new()).components;
        self.do_(Branch::new(condition, components))
    }

    /// Same as `if_` but with an `else`.
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

    /// Creates a [Scope].
    pub fn scope_(
        self,
        body: impl FnOnce(ConfigurationBuilder<P>) -> ConfigurationBuilder<P>,
    ) -> Self {
        let components = body(ConfigurationBuilder::new()).components;
        self.do_(Scope::new(components))
    }

    /// Finalizes the configuration.
    pub fn build(self) -> Configuration<P> {
        Configuration::new(Block::new(self.components))
    }
}
