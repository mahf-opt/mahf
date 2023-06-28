//! Evaluate individuals, track best individuals and Pareto fronts.

use std::any::type_name;

use color_eyre::Section;
use derivative::Derivative;
use eyre::eyre;
use serde::Serialize;

use crate::{
    component::ExecResult,
    components::Component,
    identifier::{Global, Identifier, PhantomId},
    population::BestIndividual,
    problems::{Evaluate, MultiObjectiveProblem, SingleObjectiveProblem},
    state::{common, StateReq},
    Problem, State,
};

/// Evaluates all [`Individual`]s in the [current population].
///
/// [`Individual`]: crate::Individual
/// [current population]: common::Populations::current
///
/// This component should be inserted before any component that requires an [objective value]
/// on the individuals.
///
/// [objective value]: crate::Individual::objective
///
/// # Evaluator
///
/// Fails with an `Err` if [`Evaluator`]`<P, I>` is not present in the [`State`], and
/// no default evaluator was specified with [`new_with`].
///
/// [`Evaluator`]: common::Evaluator
/// [`new_with`]: PopulationEvaluator::new_with
///
/// # Examples
///
/// An `PopulationEvaluator` is usually created by calling the
/// `{`[`evaluate`], [`evaluate_with`], [`evaluate_with_init`]`}` method
/// on [`Configuration::builder`].
///
/// [`evaluate`]: crate::configuration::ConfigurationBuilder::evaluate
/// [`evaluate_with`]: crate::configuration::ConfigurationBuilder::evaluate_with
/// [`evaluate_with_init`]: crate::configuration::ConfigurationBuilder::evaluate_with_init
/// [`Configuration::builder`]: crate::Configuration::builder
///
/// To allow for multiple evaluators, an [`identifier`] is used to distinguish them.
/// The [`Global`] identifier acts as a default.
///
/// [`identifier`]: crate::identifier
/// [`Global`]: Global
///
/// You also usually want to call [`update_best_individual`] or [`update_pareto_front`]
/// directly afterwards:
///
/// [`update_best_individual`]: crate::configuration::ConfigurationBuilder::update_best_individual
/// [`update_pareto_front`]: crate::configuration::ConfigurationBuilder::update_pareto_front
///
/// ```no_run
/// # use mahf::{SingleObjectiveProblem, problems::ObjectiveFunction};
/// # fn component1<P: SingleObjectiveProblem + ObjectiveFunction>() -> Box<dyn mahf::Component<P>> { unimplemented!() }
/// # fn component2<P: SingleObjectiveProblem + ObjectiveFunction>() -> Box<dyn mahf::Component<P>> { unimplemented!() }
/// # fn component_that_requires_evaluation<P: SingleObjectiveProblem + ObjectiveFunction>() -> Box<dyn mahf::Component<P>> { unimplemented!() }
/// use mahf::Configuration;
///
/// # pub fn example<P: SingleObjectiveProblem + ObjectiveFunction>() -> Configuration<P> {
/// Configuration::builder()
///     .do_(component1())
///     .do_(component2())
///     .evaluate()
///     .update_best_individual()
///     .do_(component_that_requires_evaluation())
///     .build()
/// # }
/// ```
#[derive(Serialize, Derivative)]
#[serde(bound = "")]
#[derivative(Clone(bound = ""))]
pub struct PopulationEvaluator<P: Problem, I: Identifier = Global> {
    #[serde(skip)]
    constructor: Option<fn() -> common::Evaluator<'static, P, I>>,
    id: PhantomId<I>,
}

impl<P, I> PopulationEvaluator<P, I>
where
    P: Problem,
    I: Identifier,
{
    /// Creates a new `PopulationEvaluator`.
    pub fn from_params(constructor: Option<fn() -> common::Evaluator<'static, P, I>>) -> Self {
        Self {
            constructor,
            id: PhantomId::default(),
        }
    }

    /// Creates a new `PopulationEvaluator`.
    ///
    /// For specifying a `constructor`, see [`new_with`].
    ///
    /// [`new_with`]: Self::new_with
    pub fn new() -> Box<dyn Component<P>> {
        Box::new(Self::from_params(None))
    }

    /// Creates a `PopulationEvaluator`, constructing an [`Evaluator`] using `T` if none
    /// with the identifier `I` is present in the [`State`].
    ///
    /// [`Evaluator`]: common::Evaluator
    pub fn new_with<T>() -> Box<dyn Component<P>>
    where
        T: Evaluate<Problem = P> + Default + 'static,
    {
        Box::new(Self::from_params(Some(|| {
            common::Evaluator::new(T::default())
        })))
    }
}

impl<P, I: Identifier> Component<P> for PopulationEvaluator<P, I>
where
    P: Problem,
    I: Identifier,
{
    fn init(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
        state.insert(common::Evaluations(0));

        if !state.has::<common::Evaluator<P, I>>() {
            if let Some(constructor) = self.constructor {
                state.insert(constructor());
            } else {
                return Err(eyre!("no evaluator found")).with_suggestion(|| {
                    format!(
                        "add an evaluator with identifier {} to the state",
                        type_name::<I>()
                    )
                });
            }
        }
        Ok(())
    }

    fn require(&self, _problem: &P, state_req: &StateReq<P>) -> ExecResult<()> {
        state_req.require::<Self, common::Populations<P>>()?;
        state_req.require::<Self, common::Evaluator<P, I>>()?;
        Ok(())
    }

    fn execute(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        let population = state.populations_mut().try_pop();
        if let Some(mut population) = population {
            state.holding::<common::Evaluator<P, I>>(
                |evaluator: &mut common::Evaluator<P, I>, state| {
                    evaluator
                        .as_inner_mut()
                        .evaluate(problem, state, &mut population);
                    Ok(())
                },
            )?;
            *state.borrow_value_mut::<common::Evaluations>() += population.len() as u32;
            state.populations_mut().push(population);
        }
        Ok(())
    }
}

/// Updates the [`BestIndividual`] yet found.
///
/// Note that this component only works on [`SingleObjectiveProblem`]s.
///
/// # Examples
///
/// The component is usually created by calling the [`update_best_individual`] method
/// on [`Configuration::builder`].
///
/// You also usually want to evaluate the individuals beforehand:
///
/// [`update_best_individual`]: crate::configuration::ConfigurationBuilder::update_best_individual
/// [`Configuration::builder`]: crate::Configuration::builder
///
/// ```no_run
/// # use mahf::{SingleObjectiveProblem, problems::ObjectiveFunction};
/// # fn component1<P: SingleObjectiveProblem + ObjectiveFunction>() -> Box<dyn mahf::Component<P>> { unimplemented!() }
/// # fn component2<P: SingleObjectiveProblem + ObjectiveFunction>() -> Box<dyn mahf::Component<P>> { unimplemented!() }
/// # fn component3<P: SingleObjectiveProblem + ObjectiveFunction>() -> Box<dyn mahf::Component<P>> { unimplemented!() }
/// use mahf::Configuration;
///
/// # pub fn example<P: SingleObjectiveProblem + ObjectiveFunction>() -> Configuration<P> {
/// Configuration::builder()
///     .do_(component1())
///     .do_(component2())
///     .evaluate()
///     .update_best_individual()
///     .do_(component3())
///     .build()
/// # }
/// ```
#[derive(Clone, Serialize)]
pub struct BestIndividualUpdate;

impl BestIndividualUpdate {
    /// Creates a new `BestIndividualUpdate`.
    pub fn from_params() -> Self {
        Self
    }

    /// Creates a new `BestIndividualUpdate`.
    pub fn new<P: SingleObjectiveProblem>() -> Box<dyn Component<P>> {
        Box::new(Self)
    }
}

impl<P: SingleObjectiveProblem> Component<P> for BestIndividualUpdate {
    fn init(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
        state.insert(common::BestIndividual::<P>::default());
        Ok(())
    }

    fn execute(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
        let populations = state.populations();
        let population = populations.current();
        let best = population.best_individual();

        if let Some(best) = best {
            state.borrow_mut::<common::BestIndividual<P>>().update(best);
        }
        Ok(())
    }
}

/// Updates the current approximation of the [`ParetoFront`].
///
/// Note that this component only works on [`MultiObjectiveProblem`]s.
///
/// [`ParetoFront`]: common::ParetoFront
///
/// # Examples
///
/// The component is usually created by calling the [`update_pareto_front`] method
/// on [`Configuration::builder`].
///
/// You also usually want to evaluate the individuals beforehand.
///
/// [`update_pareto_front`]: crate::configuration::ConfigurationBuilder::update_pareto_front
/// [`Configuration::builder`]: crate::Configuration::builder
///
/// ```no_run
/// # use mahf::{MultiObjectiveProblem, problems::ObjectiveFunction};
/// # fn component1<P: MultiObjectiveProblem + ObjectiveFunction>() -> Box<dyn mahf::Component<P>> { unimplemented!() }
/// # fn component2<P: MultiObjectiveProblem + ObjectiveFunction>() -> Box<dyn mahf::Component<P>> { unimplemented!() }
/// # fn component3<P: MultiObjectiveProblem + ObjectiveFunction>() -> Box<dyn mahf::Component<P>> { unimplemented!() }
/// use mahf::Configuration;
///
/// # pub fn example<P: MultiObjectiveProblem + ObjectiveFunction>() -> Configuration<P> {
/// Configuration::builder()
///     .do_(component1())
///     .do_(component2())
///     .evaluate()
///     .update_pareto_front()
///     .do_(component3())
///     .build()
/// # }
/// ```
#[derive(Clone, Serialize)]
pub struct ParetoFrontUpdate;

impl ParetoFrontUpdate {
    /// Creates a new `ParetoFrontUpdate`.
    pub fn from_params() -> Self {
        Self
    }

    /// Creates a new `ParetoFrontUpdate`.
    pub fn new<P: MultiObjectiveProblem>() -> Box<dyn Component<P>> {
        Box::new(Self)
    }
}

impl<P: MultiObjectiveProblem> Component<P> for ParetoFrontUpdate {
    fn init(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
        state.insert(common::ParetoFront::<P>::default());
        Ok(())
    }

    fn execute(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
        let populations = state.populations();
        let mut front = state.borrow_mut::<common::ParetoFront<P>>();

        for individual in populations.current() {
            front.update(individual);
        }

        Ok(())
    }
}
