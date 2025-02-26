//! Recombination components for Differential Evolution (DE).

use std::marker::PhantomData;
use better_any::{Tid, TidAble};
use derive_more::{Deref, DerefMut};
use eyre::ContextCompat;
use itertools::multizip;
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::{component::ExecResult, components::Component, population::{AsSolutions, AsSolutionsMut}, problems::VectorProblem, CustomState, Problem, State};
use crate::component::AnyComponent;
use crate::components::mutation::de::SHADEParamF;

/// Performs a binomial crossover, combining two individuals from two populations at the same index.
///
/// Originally proposed for, and used as recombination in [`de`].
///
/// Requires at least two populations on the stack, where the top population is modified.
///
/// Note that this crossover only has an effect if the two populations differ from each other.
///
/// [`de`]: crate::heuristics::de
///
/// # Errors
///
/// Returns an `Err` if there are less than two populations on the stack.
#[derive(Clone, Serialize, Deserialize)]
pub struct DEBinomialCrossover {
    pc: f64,
}

impl DEBinomialCrossover {
    pub fn from_params(pc: f64) -> Self {
        Self { pc }
    }

    pub fn new<P>(pc: f64) -> Box<dyn Component<P>>
    where
        P: Problem + VectorProblem<Element = f64>,
    {
        Box::new(Self::from_params(pc))
    }
}

impl<P> Component<P> for DEBinomialCrossover
where
    P: Problem + VectorProblem<Element = f64>,
{
    fn init(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
        if !state.contains::<SHADEParamCR::<Self>>() {
            let length = state.populations().current().len();
            state.insert(SHADEParamCR::<Self>::new(vec![self.pc; length]));
        }
        Ok(())
    }
    
    fn execute(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        let mut populations = state.populations_mut();
        let mut rng = state.random_mut();
        
        let cr_vector = state.get_value::<SHADEParamCR<Self>>();

        let mut mutations = populations
            .try_pop()
            .wrap_err("mutated individuals are missing")?;
        let bases = populations
            .get_current()
            .wrap_err("base population is missing")?;

        for (mutation, base, cr) in multizip((mutations.as_solutions_mut(), bases.as_solutions(), cr_vector)) {
            let index = rng.gen_range(0..problem.dimension());

            for i in 0..problem.dimension() {
                if rng.gen::<f64>() <= cr || i == index {
                    mutation[i] = base[i];
                }
            }
        }

        populations.push(mutations);
        Ok(())
    }
}

/// Performs a exponential crossover, combining two individuals from two populations at the same index.
///
/// Originally proposed for, and used as recombination in [`de`].
///
/// Requires at least two populations on the stack, where the top population is modified.
///
/// Note that this crossover only has an effect if the two populations differ from each other.
///
/// [`de`]: crate::heuristics::de
///
/// # Errors
///
/// Returns an `Err` if there are less than two populations on the stack.
#[derive(Clone, Serialize, Deserialize)]
pub struct DEExponentialCrossover {
    pc: f64,
}

impl DEExponentialCrossover {
    pub fn from_params(pc: f64) -> Self {
        Self { pc }
    }

    pub fn new<P>(pc: f64) -> Box<dyn Component<P>>
    where
        P: Problem + VectorProblem<Element = f64>,
    {
        Box::new(Self::from_params(pc))
    }
}

impl<P> Component<P> for DEExponentialCrossover
where
    P: Problem + VectorProblem<Element = f64>,
{
    fn init(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
        let length = state.populations().current().len();
        state.insert(SHADEParamCR::<Self>::new(vec![self.pc; length]));
        Ok(())
    }
    
    fn execute(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        let mut populations = state.populations_mut();
        let mut rng = state.random_mut();

        let cr_vector = state.get_value::<SHADEParamCR<Self>>();

        let mut mutations = populations.pop();
        let bases = populations.current();

        for (mutation, base, cr) in multizip((mutations.as_solutions_mut(), bases.as_solutions(), cr_vector)) {
            let index = rng.gen_range(0..problem.dimension());
            let mut i = index;

            loop {
                mutation[i] = base[i];
                i = (i + 1) % problem.dimension();

                if rng.gen::<f64>() > cr || i == index {
                    break;
                }
            }
        }

        populations.push(mutations);
        Ok(())
    }
}

/// CR as vector for values for every individual in the population as used in JADE and SHADE.
#[derive(Clone, Deref, DerefMut, Tid)]
pub struct SHADEParamCR<T: AnyComponent + 'static>(
    #[deref]
    #[deref_mut]
    Vec<f64>,
    PhantomData<T>,
);

impl<T: AnyComponent> SHADEParamCR<T> {
    pub fn new(cr: Vec<f64>) -> Self {
        Self(cr, PhantomData)
    }
}

impl<T: AnyComponent> CustomState<'_> for SHADEParamCR<T> {}