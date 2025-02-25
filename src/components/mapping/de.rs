//! State mappings for adaptive Differential Evolution (DE), e.g. SHADE.

use std::marker::PhantomData;
use better_any::{Tid, TidAble};
use derivative::Derivative;
use derive_more::{Deref, DerefMut};
use eyre::ensure;
use rand::distributions::{Distribution, Uniform};
use rand_distr::{Cauchy, Normal};
use serde::Serialize;
use crate::{component::ExecResult, components::{
    mapping::{mapping, Mapping},
    Component,
}, lens::{AnyLens, ValueLens}, state::random::Random, CustomState, Problem, SingleObjectiveProblem, State};
use crate::components::archive::DEKeepParentsArchive;
use crate::components::mutation::de::SHADEParamF;
use crate::components::recombination::de::SHADEParamCR;
use crate::identifier::{Global, Identifier, PhantomId};
use crate::prelude::StateReq;
use crate::problems::LimitedVectorProblem;

#[derive(Clone, Deref, DerefMut, Tid)]
pub struct SHADEHistoryF<I: Identifier + 'static = Global> (
    #[deref]
    #[deref_mut]
    Vec<f64>,
    PhantomData<I>
);

impl<I: Identifier> SHADEHistoryF<I> {
    pub fn new(mean_f: Vec<f64>) -> Self {
        Self(mean_f, PhantomData)
    }
}

impl<I: Identifier> CustomState<'_> for SHADEHistoryF<I> {}

#[derive(Clone, Deref, DerefMut, Tid)]
pub struct SHADEHistoryCR<I: Identifier + 'static = Global> (
    #[deref]
    #[deref_mut]
    Vec<f64>,
    PhantomData<I>
);

impl<I: Identifier> SHADEHistoryCR<I> {
    pub fn new(mean_cr: Vec<f64>) -> Self {
        Self(mean_cr, PhantomData)
    }
}

impl<I: Identifier> CustomState<'_> for SHADEHistoryCR<I> {}

#[derive(Clone, Serialize)]
pub struct SHADEAdaptation<I: Identifier = Global>(PhantomId<I>);

impl<I: Identifier> SHADEAdaptation<I> {
    pub fn from_params() -> Self {
        Self(PhantomId::default())
    }

    pub fn new<P>() -> Box<dyn Component<P>>
    where
        P: SingleObjectiveProblem + LimitedVectorProblem<Element = f64>,
    {
        Box::new(Self::from_params())
    }
}

impl<P, I> Component<P> for SHADEAdaptation<I>
where 
    P: SingleObjectiveProblem + LimitedVectorProblem<Element = f64>,
    I: Identifier,
{
    fn init(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
        state.insert(SHADEHistoryF::<I>::new(vec![0.5]));
        state.insert(SHADEHistoryCR::<I>::new(vec![0.5]));
        let mut current_fs = state.borrow_value_mut::<SHADEParamF<I>>();
        let mut current_crs = state.borrow_value_mut::<SHADEParamCR<I>>();
        let history_fs = state.borrow_value_mut::<SHADEHistoryF<I>>();
        let history_crs = state.borrow_value_mut::<SHADEHistoryCR<I>>();

        let mut rng = state.random_mut();
        
        for i in 0..current_fs.len() {
            let distribution = Cauchy::new(history_fs[0], 0.1).unwrap();
            let mut random_new = distribution.sample(&mut *rng);
            if random_new > 1.0 {
                random_new = 1.0;
            } else {
                while random_new <= 0.0 {
                    random_new = distribution.sample(&mut *rng);
                    if random_new > 1.0 {
                        random_new = 1.0;
                    }
                }
            }
            current_fs[i] = random_new;
        }
        for i in 0..current_crs.len() {
            let distribution = Normal::new(history_crs[0], 0.1).unwrap();
            let mut random_new = distribution.sample(&mut *rng);
            if random_new > 1.0 {
                random_new = 1.0;
            } else if random_new < 0.0 { 
                random_new = 0.0;
            }
            current_crs[i] = random_new;
        }
        Ok(())
    }

    fn require(&self, _problem: &P, state_req: &StateReq<P>) -> ExecResult<()> {
        state_req.require::<Self, SHADEParamF<I>>()?;
        state_req.require::<Self, SHADEParamCR<I>>()?;
        Ok(())
    }
    
    fn execute(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
        let mut current_fs = state.borrow_value_mut::<SHADEParamF<I>>();
        let mut current_crs = state.borrow_value_mut::<SHADEParamCR<I>>();
        let history_fs = state.borrow_value_mut::<SHADEHistoryF<I>>();
        let history_crs = state.borrow_value_mut::<SHADEHistoryCR<I>>();

        let mut rng = state.random_mut();
        
        let history_distribution = Uniform::new(0, history_fs.len());
        let random_history = history_distribution.sample(&mut *rng);

        for i in 0..current_fs.len() {
            let distribution = Cauchy::new(history_fs[random_history], 0.1).unwrap();
            let mut random_new = distribution.sample(&mut *rng);
            if random_new > 1.0 {
                random_new = 1.0;
            } else {
                while random_new <= 0.0 {
                    random_new = distribution.sample(&mut *rng);
                    if random_new > 1.0 {
                        random_new = 1.0;
                    }
                }
            }
            current_fs[i] = random_new;
        }
        for i in 0..current_crs.len() {
            let distribution = Normal::new(history_crs[random_history], 0.1).unwrap();
            let mut random_new = distribution.sample(&mut *rng);
            if random_new > 1.0 {
                random_new = 1.0;
            } else if random_new < 0.0 {
                random_new = 0.0;
            }
            current_crs[i] = random_new;
        }
        Ok(())
    }
}

#[derive(Clone, Serialize)]
pub struct SHADEAdaptationHistoryUpdate<I: Identifier = Global>(PhantomId<I>);

impl<I: Identifier> SHADEAdaptationHistoryUpdate<I> {
    pub fn from_params() -> Self {
        Self(PhantomId::default())
    }

    pub fn new<P>() -> Box<dyn Component<P>>
    where
        P: SingleObjectiveProblem + LimitedVectorProblem<Element = f64>,
    {
        Box::new(Self::from_params())
    }
}

impl<P, I> Component<P> for SHADEAdaptationHistoryUpdate<I>
where
    P: SingleObjectiveProblem + LimitedVectorProblem<Element = f64>,
    I: Identifier,
{
    fn require(&self, _problem: &P, state_req: &StateReq<P>) -> ExecResult<()> {
        state_req.require::<Self, SHADEHistoryF<I>>()?;
        state_req.require::<Self, SHADEHistoryCR<I>>()?;
        Ok(())
    }

    fn execute(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
        let mut populations = state.populations_mut();
        let offspring = populations.pop();
        let parents = populations.pop();
        let o = offspring.clone();
        let p = parents.clone();
        populations.push(parents);
        populations.push(offspring);

        let mut indices = Vec::new();
        for (i, offspring) in o.into_iter().enumerate() {
            if p[i].objective() < offspring.objective() {
                indices.push(i);
            }
        }
        //TODO finish history update

        Ok(())
    }
}