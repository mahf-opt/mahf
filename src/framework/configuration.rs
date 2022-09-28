use crate::{
    framework::{
        components::{Block, Branch, Component, Loop, Scope},
        conditions::Condition,
    },
    operators,
    problems::{MultiObjectiveProblem, Problem, SingleObjectiveProblem},
    state,
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

    pub fn into_inner(self) -> Box<dyn Component<P>> {
        self.0
    }

    pub fn into_builder(self) -> ConfigurationBuilder<P> {
        ConfigurationBuilder::new().do_(self.0)
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

// Basic functionality
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

    pub fn do_optional_(self, component: Option<Box<dyn Component<P>>>) -> Self {
        if let Some(component) = component {
            self.do_(component)
        } else {
            self
        }
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

    pub fn build_component(self) -> Box<dyn Component<P>> {
        Block::new(self.components)
    }
}

// Convenience methods
impl<P: Problem> ConfigurationBuilder<P> {
    /// Asserts the condition on [State][state::State].
    ///
    /// Uses the [Debug][operators::misc::Debug] component internally.
    #[track_caller]
    pub fn assert(self, assert: impl Fn(&state::State) -> bool + Send + Sync + 'static) -> Self {
        self.debug(move |_problem, state| assert!(assert(state)))
    }

    /// Constructs a [Debug][operators::misc::Debug] component with the given behaviour.
    pub fn debug(self, behaviour: impl Fn(&P, &mut state::State) + Send + Sync + 'static) -> Self {
        self.do_(operators::misc::Debug::new(behaviour))
    }

    pub fn evaluate_sequential(self) -> Self {
        self.do_(operators::evaluation::SequentialEvaluator::new())
    }
}

impl<P: SingleObjectiveProblem> ConfigurationBuilder<P> {
    pub fn update_best_individual(self) -> Self {
        self.do_(operators::evaluation::UpdateBestIndividual::new())
    }
}

impl<P: SingleObjectiveProblem> ConfigurationBuilder<P>
where
    P::Encoding: std::fmt::Debug,
{
    pub fn single_objective_summary(self) -> Self {
        self.do_(operators::misc::PrintSingleObjectiveSummary::new())
    }
}

impl<P: MultiObjectiveProblem> ConfigurationBuilder<P> {
    pub fn update_pareto_front(self) -> Self {
        self.do_(operators::evaluation::UpdateParetoFront::new())
    }
}
