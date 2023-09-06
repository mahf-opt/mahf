//! Metaheuristic configurations.

use std::{fs::File, path::Path};

use eyre::WrapErr;

use crate::{
    component::ExecResult,
    components::{evaluation, utils::debug, Block, Branch, Component, Loop, Scope},
    conditions::Condition,
    identifier,
    identifier::Identifier,
    logging,
    problems::{Evaluate, MultiObjectiveProblem, SingleObjectiveProblem},
    state::{common, random::Random},
    Problem, State,
};

/// A (meta)heuristic configuration.
///
/// A grouping of components is called a metaheuristic configuration, and the `Configuration` struct
/// provides useful abstractions for dealing with them.
///
/// A configuration therefore fully specifies a metaheuristic algorithm,
/// with all its components and parameters.
///
/// # Application
///
/// Applying a `Configuration` to a problem is as simple as calling its [`optimize`] method with
/// the problem and an evaluator as argument.
/// The method returns the [`State`] after execution, enabling inspection of any custom state,
/// including the final population, or the best individual found.
///
/// It is also possible to pre-initialize the State before execution using the [`optimize_with`]
/// method, e.g. to set the random number seed or customize what state should be logged at runtime.
///
/// [`optimize`]: Configuration::optimize
/// [`optimize_with`]: Configuration::optimize
/// [`Evaluator`]: common::Evaluator
///
///
/// # Building `Configuration`s
///
/// Because an object-oriented approach to specifying control flow is not very intuitive,
/// `Configuration` also exposes a simple and idiomatic way to construct metaheuristics
/// through Rust’s builder pattern.
///
/// See [`Configuration::builder`] for more information.
///
/// # Serialization
///
/// For the purpose of easily identifying which experiment was done with which components
/// and parameters, it is serializable (but not deserializable).
pub struct Configuration<P: Problem>(Box<dyn Component<P>>);

impl<P: Problem> Configuration<P> {
    /// Constructs a `Configuration` from some heuristic components.
    ///
    /// Use [`Configuration::builder`] for a more convenient construction.
    pub fn new(heuristic: Box<dyn Component<P>>) -> Self {
        Self(heuristic)
    }

    /// Creates a builder for constructing a `Configuration`.
    ///
    /// The builder exposes familiar control flow methods like [`do_`], [`while_`], [`if_`],
    /// and [`if_else_`] along with shortcut methods to construct other often-used components.
    ///
    /// [`do_`]: ConfigurationBuilder::do_
    /// [`while_`]: ConfigurationBuilder::while_
    /// [`if_`]: ConfigurationBuilder::if_
    /// [`if_else_`]: ConfigurationBuilder::if_else_
    ///
    /// # Examples
    ///
    /// Constructing a simple genetic algorithm using the builder syntax:
    ///
    /// ```
    /// use mahf::prelude::*;
    /// # use mahf::problems::{LimitedVectorProblem, ObjectiveFunction};
    /// # use mahf::SingleObjectiveProblem;
    ///
    /// # fn example<P: SingleObjectiveProblem + LimitedVectorProblem<Element = f64> + ObjectiveFunction>() -> Configuration<P> {
    /// # let population_size = 30;
    /// # let n = 5_000;
    /// # let num_selected = 30;
    /// # let size = 5;
    /// # let std_dev = 0.1;
    /// # let rm = 1.0;
    /// # let max_population_size = 30;
    /// let ga = Configuration::builder()
    ///     .do_(initialization::RandomSpread::new(population_size))
    ///     .evaluate()
    ///     .update_best_individual()
    ///     .while_(conditions::LessThanN::iterations(n), |builder| {
    ///         builder
    ///             .do_(selection::Tournament::new(num_selected, size))
    ///             .do_(recombination::ArithmeticCrossover::new_insert_both(1.))
    ///             .do_(mutation::NormalMutation::new(std_dev, rm))
    ///             .evaluate()
    ///             .update_best_individual()
    ///             .do_(replacement::MuPlusLambda::new(max_population_size))
    ///     })
    ///     .build();
    /// # ga
    /// # }

    /// ```
    pub fn builder() -> ConfigurationBuilder<P> {
        ConfigurationBuilder::new()
    }

    /// Returns a reference to the root [`Component`].
    pub fn heuristic(&self) -> &dyn Component<P> {
        self.0.as_ref()
    }

    /// Consumes the `Configuration`, returning the root [`Component`].
    pub fn into_inner(self) -> Box<dyn Component<P>> {
        self.0
    }

    /// Creates a builder pre-initialized with the root [`Component`].
    pub fn into_builder(self) -> ConfigurationBuilder<P> {
        Self::builder().do_(self.0)
    }

    /// Serializes the `Configuration` into the file at `path` using [`ron`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use mahf::{ExecResult, Problem};
    /// use mahf::Configuration;
    ///
    /// # fn example<P: Problem>(problem: P) -> ExecResult<Configuration<P>> {
    /// let config = Configuration::builder()/* ... */.build();
    /// config.to_ron("path/to/ron")?;
    /// # Ok(config)
    /// # }
    /// ```
    pub fn to_ron(&self, path: impl AsRef<Path>) -> ExecResult<()> {
        ron::ser::to_writer_pretty(
            std::io::BufWriter::new(
                File::create(path).wrap_err("failed to create configuration file")?,
            ),
            self.heuristic(),
            ron::ser::PrettyConfig::default().struct_names(true),
        )
        .wrap_err("failed to serialize configuration")
    }

    /// Runs the `Configuration` on the `problem` using a given [`State`].
    ///
    /// Note that the caller is responsible for initializing `state` properly.
    pub fn run(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        self.0.init(problem, state)?;
        self.0.require(problem, &state.requirements())?;
        self.0.execute(problem, state)?;
        Ok(())
    }

    /// Runs the heuristic `Configuration` on the given `problem`, returning the final [`State`]
    /// after execution of the heuristic.
    ///
    /// # Initialization
    ///
    /// The state is pre-initialized with [`Populations`] and [`Log`].
    ///
    /// The random generator defaults to a randomly seeded RNG ([`Random::default`]).
    ///
    /// The `evaluator` is inserted wrapped inside an [`Evaluator`] with the [`Global`] identifier.
    ///
    /// For initializing the state with custom state, e.g. a fixed random seed,
    /// see [`optimize_with`].
    ///
    /// [`Populations`]: common::Populations
    /// [`Log`]: logging::Log
    /// [`Evaluator`]: common::Evaluator
    /// [`Global`]: identifier::Global
    /// [`optimize_with`]: Self::optimize_with
    ///
    ///
    /// # Examples
    ///
    /// Optimizing some `problem` with a sequential evaluator:
    ///
    /// ```
    /// # use mahf::problems::ObjectiveFunction;
    /// use mahf::{problems::Sequential, Configuration};
    ///
    /// # fn example<P: ObjectiveFunction>(problem: P) {
    /// let config = Configuration::builder()
    ///     /* configuration definition */
    ///     .build();
    /// let state = config.optimize(&problem, Sequential::new());
    /// # }
    /// ```
    pub fn optimize<'a, T>(&self, problem: &P, evaluator: T) -> ExecResult<State<'a, P>>
    where
        T: Evaluate<Problem = P> + 'a,
    {
        let mut state = State::new();

        state.insert(logging::Log::new());
        state.insert(Random::default());
        state.insert(common::Populations::<P>::new());
        state.insert(common::Evaluator::<P, identifier::Global>::new(evaluator));

        self.run(problem, &mut state)?;

        Ok(state)
    }

    /// Runs the heuristic `Configuration` on the given `problem`, initializing the [`State`]
    /// beforehand with a custom function, returning the final [`State`] after execution
    /// of the heuristic.
    ///
    /// # Initialization
    ///
    /// The state is pre-initialized with [`Populations`] and [`Log`].
    ///
    /// If no random generator is inserted in `init_state`, it will default
    /// to a randomly seeded RNG ([Random::default]).
    ///
    /// Note that the evaluator has to be inserted **manually** into the [`State`], using e.g. `{`[`State::insert_evaluator`], [`State::insert_evaluator_as`]`}`.
    ///
    /// [`Populations`]: common::Populations
    /// [`Log`]: logging::Log
    /// [`optimize_with`]: Self::optimize_with
    ///
    /// # Examples
    ///
    /// Optimizing some `problem` with a sequential evaluator and a random seed of `42`:
    ///
    /// ```
    /// # use mahf::problems::ObjectiveFunction;
    /// use mahf::{identifier::Global, problems::Sequential, Configuration, Random};
    ///
    /// # fn example<P: ObjectiveFunction>(problem: P) {
    /// let config = Configuration::builder()/* ... */.build();
    /// let state = config.optimize_with(&problem, |state| {
    ///     state.insert_evaluator(Sequential::new());
    ///     state.insert(Random::new(42));
    ///     Ok(())
    /// });
    /// # }
    /// ```
    pub fn optimize_with<'a>(
        &self,
        problem: &P,
        init_state: impl FnOnce(&mut State<'a, P>) -> ExecResult<()>,
    ) -> ExecResult<State<'a, P>> {
        let mut state = State::new();

        state.insert(logging::Log::new());
        state.insert(common::Populations::<P>::new());

        init_state(&mut state)?;

        if !state.contains::<Random>() {
            state.insert(Random::default());
        }

        self.run(problem, &mut state)?;

        Ok(state)
    }
}

impl<P: Problem> Clone for Configuration<P> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<P: Problem> From<Box<dyn Component<P>>> for Configuration<P> {
    fn from(heuristic: Box<dyn Component<P>>) -> Self {
        Self::new(heuristic)
    }
}

/// A simple DSL for building a (meta)heuristic [`Configuration`].
///
/// Its recommended usage is through the [`Configuration::builder`] method.
pub struct ConfigurationBuilder<P: Problem> {
    components: Vec<Box<dyn Component<P>>>,
}

impl<P: Problem> ConfigurationBuilder<P> {
    /// Constructs a new builder.
    fn new() -> Self {
        Self {
            components: Vec::new(),
        }
    }

    /// Adds the `component` to the current [`Block`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use mahf::Problem;
    /// # fn component1<P: Problem>() -> Box<dyn mahf::Component<P>> { unimplemented!() }
    /// # fn component2<P: Problem>() -> Box<dyn mahf::Component<P>> { unimplemented!() }
    /// # fn component3<P: Problem>() -> Box<dyn mahf::Component<P>> { unimplemented!() }
    /// use mahf::Configuration;
    ///
    /// # pub fn example<P: Problem>() -> Configuration<P> {
    /// Configuration::builder()
    ///     .do_(component1())
    ///     .do_(component2())
    ///     .do_(component3())
    ///     .build()
    /// # }
    /// ```
    pub fn do_(mut self, component: Box<dyn Component<P>>) -> Self {
        self.components.push(component);
        self
    }

    /// Adds the `component` to the current [`Block`], or does nothing if it is `None`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use mahf::Problem;
    /// # fn component<P: Problem>() -> Box<dyn mahf::Component<P>> { unimplemented!() }
    /// use mahf::Configuration;
    ///
    /// # pub fn example<P: Problem>() -> Configuration<P> {
    /// Configuration::builder()
    ///     .do_if_some_(Some(component()))
    ///     .do_if_some_(None)
    ///     .build()
    /// # }
    /// ```
    pub fn do_if_some_(self, component: Option<Box<dyn Component<P>>>) -> Self {
        if let Some(component) = component {
            self.do_(component)
        } else {
            self
        }
    }

    /// Adds all `components` to the current [`Block`].
    ///
    /// This is equivalent to calling [`do_`] repeatedly, but offers better visual grouping
    /// for multiple components which serve the same purpose, e.g. calculating metrics.
    ///
    /// [`do_`]: Self::do_
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use mahf::Problem;
    /// # fn metric1<P: Problem>() -> Box<dyn mahf::Component<P>> { unimplemented!() }
    /// # fn metric2<P: Problem>() -> Box<dyn mahf::Component<P>> { unimplemented!() }
    /// # fn metric3<P: Problem>() -> Box<dyn mahf::Component<P>> { unimplemented!() }
    /// use mahf::Configuration;
    ///
    /// # pub fn example<P: Problem>() -> Configuration<P> {
    /// Configuration::builder()
    ///     .do_many_([metric1(), metric2(), metric3()])
    ///     .build()
    /// # }
    /// ```
    pub fn do_many_(mut self, components: impl IntoIterator<Item = Box<dyn Component<P>>>) -> Self {
        for component in components {
            self.components.push(component);
        }
        self
    }

    /// Loops the `body` while the `condition` is `true`.
    ///
    /// Internally, the [`Loop`] component is created with the given `condition` and `body`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use mahf::Problem;
    /// # fn condition<P: Problem>() -> Box<dyn mahf::Condition<P>> { unimplemented!() }
    /// # fn component1<P: Problem>() -> Box<dyn mahf::Component<P>> { unimplemented!() }
    /// # fn component2<P: Problem>() -> Box<dyn mahf::Component<P>> { unimplemented!() }
    /// # fn component3<P: Problem>() -> Box<dyn mahf::Component<P>> { unimplemented!() }
    /// use mahf::Configuration;
    ///
    /// # pub fn example<P: Problem>() -> Configuration<P> {
    /// Configuration::builder()
    ///     .while_(condition(), |builder| {
    ///         builder
    ///             .do_(component1())
    ///             .do_(component2())
    ///             .do_(component3())
    ///     })
    ///     .build()
    /// # }
    /// ```
    pub fn while_(
        self,
        condition: Box<dyn Condition<P>>,
        body: impl FnOnce(ConfigurationBuilder<P>) -> ConfigurationBuilder<P>,
    ) -> Self {
        let components = body(ConfigurationBuilder::new()).components;
        self.do_(Loop::new(condition, components))
    }

    /// Executes the `body` if the `condition` is `true`.
    ///
    /// Internally, the [`Branch`] component is created with the given `condition` and `body`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use mahf::Problem;
    /// # fn condition<P: Problem>() -> Box<dyn mahf::Condition<P>> { unimplemented!() }
    /// # fn component1<P: Problem>() -> Box<dyn mahf::Component<P>> { unimplemented!() }
    /// # fn component2<P: Problem>() -> Box<dyn mahf::Component<P>> { unimplemented!() }
    /// # fn component3<P: Problem>() -> Box<dyn mahf::Component<P>> { unimplemented!() }
    /// use mahf::Configuration;
    ///
    /// # pub fn example<P: Problem>() -> Configuration<P> {
    /// Configuration::builder()
    ///     .if_(condition(), |builder| {
    ///         builder
    ///             .do_(component1())
    ///             .do_(component2())
    ///             .do_(component3())
    ///     })
    ///     .build()
    /// # }
    /// ```
    pub fn if_(
        self,
        condition: Box<dyn Condition<P>>,
        body: impl FnOnce(ConfigurationBuilder<P>) -> ConfigurationBuilder<P>,
    ) -> Self {
        let components = body(ConfigurationBuilder::new()).components;
        self.do_(Branch::new(condition, components))
    }

    /// Executes the `if_body` or `else_body` depending on the `condition`.
    ///
    /// Internally, the [`Branch`] component is created with the given `condition`, `if_body`, and `else_body`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use mahf::Problem;
    /// # fn condition<P: Problem>() -> Box<dyn mahf::Condition<P>> { unimplemented!() }
    /// # fn component1<P: Problem>() -> Box<dyn mahf::Component<P>> { unimplemented!() }
    /// # fn component2<P: Problem>() -> Box<dyn mahf::Component<P>> { unimplemented!() }
    /// # fn component3<P: Problem>() -> Box<dyn mahf::Component<P>> { unimplemented!() }
    /// # fn component4<P: Problem>() -> Box<dyn mahf::Component<P>> { unimplemented!() }
    /// use mahf::Configuration;
    ///
    /// # pub fn example<P: Problem>() -> Configuration<P> {
    /// Configuration::builder()
    ///     .if_else_(condition(),
    ///         |if_builder| {
    ///             if_builder
    ///                 .do_(component1())
    ///                 .do_(component2())
    ///         },
    ///         |else_builder| {
    ///             else_builder
    ///                 .do_(component3())
    ///                 .do_(component4())
    ///     })
    ///     .build()
    /// # }
    /// ```
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

    /// Executes the `body` in a new scope, where shadowing custom state is possible.
    ///
    /// This is especially useful for nested heuristics with an inner loop.
    ///
    /// Internally, the [`Scope`] component is created with the given `body`.
    ///
    /// ```no_run
    /// # use mahf::Problem;
    /// # fn heuristic<P: Problem>() -> Box<dyn mahf::Component<P>> { unimplemented!() }
    /// use mahf::Configuration;
    ///
    /// # pub fn example<P: Problem>() -> Configuration<P> {
    /// Configuration::builder()
    ///     .scope_(|builder| builder.do_(heuristic()))
    ///     .build()
    /// # }
    /// ```
    pub fn scope_(
        self,
        body: impl FnOnce(ConfigurationBuilder<P>) -> ConfigurationBuilder<P>,
    ) -> Self {
        let components = body(ConfigurationBuilder::new()).components;
        self.do_(Scope::new(components))
    }

    /// Consumes the builder, creating a [`Configuration`].
    pub fn build(self) -> Configuration<P> {
        Configuration::new(Block::new(self.components))
    }

    /// Consumes the builder, creating a [`Component`].
    ///
    /// This method is usually only used for heuristic templates.
    pub fn build_component(self) -> Box<dyn Component<P>> {
        Block::new(self.components)
    }
}

impl<P: Problem> ConfigurationBuilder<P> {
    /// Asserts some condition on the [`State`].
    ///
    /// Internally, the [`Debug`] component is created, passing an `assert!` closure.
    /// See its documentation for limitations.
    ///
    /// Note that the execution panics when the assertion is violated.
    ///
    /// [`Debug`]: debug::Debug
    ///
    /// # Examples
    ///
    /// Asserting that the size of the current population equals `1`:
    ///
    /// ```no_run
    /// # use mahf::Problem;
    /// use mahf::Configuration;
    ///
    /// # pub fn example<P: Problem>() -> Configuration<P> {
    /// Configuration::builder()
    ///     .assert(|state| state.populations().current().len() == 1)
    ///     .build()
    /// # }
    /// ```
    #[track_caller]
    pub fn assert(
        self,
        assert: impl Fn(&State<P>) -> bool + Send + Sync + Clone + 'static,
    ) -> Self {
        self.debug(move |_problem, state| assert!(assert(state)))
    }

    /// Enables arbitrary `behaviour` for debugging purposes.
    ///
    /// Internally, the [`Debug`] component is created with the given `behaviour`.
    /// See its documentation for limitations.
    ///
    /// # Examples
    ///
    /// Printing the current population.
    /// Note that most useful behaviour imposes some restriction on the [`Problem`]
    /// which are usually not required, e.g. [`Problem::Encoding`]`: Debug` in this case:
    ///
    /// ```no_run
    /// # use std::fmt::Debug;
    /// # use mahf::Problem;
    /// use mahf::Configuration;
    ///
    /// # pub fn example<P>() -> Configuration<P> where P: Problem, P::Encoding: Debug {
    /// Configuration::builder()
    ///     .debug(|_problem, state| {
    ///         println!("Current Population: {:?}", state.populations().current())
    ///     })
    ///     .build()
    /// # }
    /// ```
    pub fn debug(
        self,
        behaviour: impl Fn(&P, &mut State<P>) + Send + Sync + Clone + 'static,
    ) -> Self {
        self.do_(debug::Debug::new(behaviour))
    }

    /// Evaluates all [`Individual`]s in the [current population] using the [`Evaluator`] with [`Global`] identifier.
    ///
    /// Internally, the [`PopulationEvaluator`] component is created.
    ///
    /// [`Individual`]: crate::Individual
    /// [current population]: common::Populations::current
    /// [`Evaluator`]: common::Evaluator
    /// [`Global`]: identifier::Global
    /// [`PopulationEvaluator`]: evaluation::PopulationEvaluator
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use mahf::Problem;
    /// use mahf::Configuration;
    ///
    /// pub fn example<P: Problem>() -> Configuration<P> {
    /// Configuration::builder()
    ///     .evaluate()
    ///     .build()
    /// # }
    /// ```
    pub fn evaluate(self) -> Self {
        self.do_(evaluation::PopulationEvaluator::new())
    }

    /// Evaluates all [`Individual`]s in the [current population] using the [`Evaluator`] with identifier `I`.
    ///
    /// Internally, the [`PopulationEvaluator`] component is created with the given identifier.
    ///
    /// The default identifier is [`Global`].
    ///
    /// [`Individual`]: crate::Individual
    /// [current population]: common::Populations::current
    /// [`Evaluator`]: common::Evaluator
    /// [`PopulationEvaluator`]: evaluation::PopulationEvaluator
    /// [`Global`]: identifier::Global
    ///
    /// # Examples
    ///
    /// Calling `evaluate` with the `Global` identifier:
    ///
    /// ```no_run
    /// # use mahf::Problem;
    /// use mahf::Configuration;
    /// use mahf::identifier::Global;
    ///
    /// pub fn example<P: Problem>() -> Configuration<P> {
    /// Configuration::builder()
    ///     .evaluate_with::<Global>()
    ///     .build()
    /// # }
    /// ```
    pub fn evaluate_with<I>(self) -> Self
    where
        I: Identifier,
    {
        self.do_(evaluation::PopulationEvaluator::<I>::new_with())
    }
}

impl<P: SingleObjectiveProblem> ConfigurationBuilder<P> {
    /// Updates the [`BestIndividual`] yet found.
    ///
    /// Internally, the [`BestIndividualUpdate`] component is created.
    ///
    /// [`BestIndividual`]: common::BestIndividual
    /// [`BestIndividualUpdate`]: evaluation::BestIndividualUpdate
    ///
    /// # Examples
    ///
    /// You also usually want to evaluate the individuals beforehand:
    ///
    /// ```no_run
    /// # use mahf::SingleObjectiveProblem;
    /// use mahf::Configuration;
    ///
    /// # pub fn example<P: SingleObjectiveProblem>() -> Configuration<P> {
    /// Configuration::builder()
    ///     .evaluate()
    ///     .update_best_individual()
    ///     .build()
    /// # }
    /// ```
    pub fn update_best_individual(self) -> Self {
        self.do_(evaluation::BestIndividualUpdate::new())
    }
}

impl<P: MultiObjectiveProblem> ConfigurationBuilder<P> {
    /// Updates the current approximation of the [`ParetoFront`].
    ///
    /// Internally, the [`ParetoFrontUpdate`] component is created.
    ///
    /// [`ParetoFront`]: common::ParetoFront
    /// [`ParetoFrontUpdate`]: evaluation::ParetoFrontUpdate
    ///
    /// # Examples
    ///
    /// You also usually want to evaluate the individuals beforehand:
    ///
    /// ```no_run
    /// # use mahf::MultiObjectiveProblem;
    /// use mahf::Configuration;
    ///
    /// # pub fn example<P: MultiObjectiveProblem>() -> Configuration<P> {
    /// Configuration::builder()
    ///     .evaluate()
    ///     .update_pareto_front()
    ///     .build()
    /// # }
    /// ```
    pub fn update_pareto_front(self) -> Self {
        self.do_(evaluation::ParetoFrontUpdate::new())
    }
}
