use crate::{
    component::ExecResult,
    components::{evaluation, misc::debug::Debug, Block, Branch, Component, Loop, Scope},
    conditions::Condition,
    logging,
    problems::{Evaluator, MultiObjectiveProblem, SingleObjectiveProblem},
    state::{common, random::Random},
    Problem, State,
};

/// A heuristic configuration, constructed from a set of components.
#[derive(Clone)]
pub struct Configuration<P: Problem>(Box<dyn Component<P>>);

impl<P: Problem> Configuration<P> {
    /// Wraps a heuristic into a `Configuration`.
    ///
    /// Use [`Configuration::builder`] for a more convenient construction.
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
        todo!()
    }

    pub fn run(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        self.0.init(problem, state)?;
        self.0.require(problem, &state.requirements())?;
        self.0.execute(problem, state)?;
        Ok(())
    }

    /// Runs the heuristic on the given problem, returning the final state of the heuristic.
    ///
    /// The state is pre-initialized with a [Population][state::common::Population]
    /// and a [Log][tracking::Log].
    /// The random generator defaults to a randomly seeded RNG ([Random::default]).
    ///
    /// For initializing the state with custom state,
    /// see [optimize_with][Configuration::optimize_with].
    pub fn optimize(&self, problem: &P) -> ExecResult<State<P>> {
        let mut state = State::new();

        state.insert(logging::Log::new());
        state.insert(Random::default());
        state.insert(common::Populations::<P>::new());

        self.run(problem, &mut state)?;

        Ok(state)
    }

    /// Runs the heuristic on the given problem, initializing the state with a custom function,
    /// and returning the final state of the heuristic.
    ///
    /// The state is pre-initialized with a [Population][state::common::Population]
    /// and a [Log][tracking::Log].
    /// If no random generator is inserted in `init_state`, it will default
    /// to a randomly seeded RNG ([Random::default]).
    pub fn optimize_with<'a>(
        &self,
        problem: &P,
        init_state: impl FnOnce(&mut State<'a, P>) -> ExecResult<()>,
    ) -> ExecResult<State<'a, P>> {
        let mut state = State::new();

        state.insert(logging::Log::new());
        state.insert(common::Populations::<P>::new());

        init_state(&mut state)?;

        if !state.has::<Random>() {
            state.insert(Random::default());
        }

        self.run(problem, &mut state)?;

        Ok(state)
    }
}

impl<P: Problem> From<Box<dyn Component<P>>> for Configuration<P> {
    fn from(heuristic: Box<dyn Component<P>>) -> Self {
        Self::new(heuristic)
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

    pub fn do_if_some_(self, component: Option<Box<dyn Component<P>>>) -> Self {
        if let Some(component) = component {
            self.do_(component)
        } else {
            self
        }
    }

    pub fn do_many_(mut self, components: impl IntoIterator<Item = Box<dyn Component<P>>>) -> Self {
        for component in components {
            self.components.push(component);
        }
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

    pub fn build_component(self) -> Box<dyn Component<P>> {
        Block::new(self.components)
    }
}

// Convenience methods
impl<P: Problem> ConfigurationBuilder<P> {
    /// Asserts the condition on [State][state::State].
    ///
    /// Uses the [Debug][components::misc::Debug] component internally.
    #[track_caller]
    pub fn assert(
        self,
        assert: impl Fn(&State<P>) -> bool + Send + Sync + Clone + 'static,
    ) -> Self {
        self.debug(move |_problem, state| assert!(assert(state)))
    }

    /// Constructs a [Debug][components::misc::Debug] component with the given behaviour.
    pub fn debug(
        self,
        behaviour: impl Fn(&P, &mut State<P>) + Send + Sync + Clone + 'static,
    ) -> Self {
        self.do_(Debug::new(behaviour))
    }

    pub fn evaluate<T: Evaluator<Problem = P>>(self) -> Self {
        self.do_(evaluation::PopulationEvaluator::<T>::new())
    }
}

impl<P: SingleObjectiveProblem> ConfigurationBuilder<P> {
    pub fn update_best_individual(self) -> Self {
        self.do_(evaluation::BestIndividualUpdate::new())
    }
}

impl<P: MultiObjectiveProblem> ConfigurationBuilder<P> {
    pub fn update_pareto_front(self) -> Self {
        self.do_(evaluation::ParetoFrontUpdate::new())
    }
}
