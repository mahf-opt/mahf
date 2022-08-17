use crate::{
    framework::{
        components::{Block, Branch, Component, Loop, Scope},
        conditions::Condition,
        state,
    },
    operators,
    problems::{MultiObjectiveProblem, Problem, SingleObjectiveProblem},
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

    pub fn do_(mut self, component: Box<dyn Component<P>>) -> Self {
        self.components.push(component);
        self
    }

    pub fn do_if_some_(self, component: Option<Box<dyn Component<P>>>) -> Self {
        if let Some(component) = component {
            self.do_(component)
        } else {
            self
        }
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

    pub fn build_component(self) -> Box<dyn Component<P>> {
        Block::new(self.components)
    }
}

// Convenience methods
impl<P: Problem> ConfigurationBuilder<P> {
    /// Asserts the condition on [State][state::State].
    ///
    /// Uses the [Debug][operators::misc::Debug] component internally.
    pub fn assert(self, assert: impl Fn(&state::State) -> bool + Send + Sync + 'static) -> Self {
        self.debug(move |_problem, state| assert!(assert(state)))
    }

    /// Constructs a [Debug][operators::misc::Debug] component with the given behaviour.
    pub fn debug(self, behaviour: impl Fn(&P, &mut state::State) + Send + Sync + 'static) -> Self {
        self.do_(operators::misc::Debug::new(behaviour))
    }

    pub fn evaluate_serial(self) -> Self {
        self.do_(operators::evaluation::SerialEvaluator::new())
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
