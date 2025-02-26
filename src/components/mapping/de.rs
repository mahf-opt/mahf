//! State mappings for adaptive Differential Evolution (DE), e.g. SHADE.

use std::marker::PhantomData;
use better_any::{Tid, TidAble};
use derive_more::{Deref, DerefMut};
use rand::distributions::{Distribution, Uniform};
use rand_distr::{Cauchy, Normal};
use serde::Serialize;
use crate::{component::ExecResult, components::Component, CustomState,SingleObjectiveProblem, State};
use crate::component::AnyComponent;
use crate::components::mutation::de::SHADEParamF;
use crate::components::recombination::de::SHADEParamCR;
use crate::identifier::{Global, Identifier, PhantomId};
use crate::prelude::StateReq;
use crate::problems::LimitedVectorProblem;


/// History of means for calculating F.
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

/// History of means for calculating CR.
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

/// Adapation of current F and CR values to be used in the respective iteration.
#[derive(Clone, Serialize)]
pub struct SHADEAdaptation<I: Identifier = Global> {
    pub history: usize,
    id: PhantomId<I>
}

impl<I: Identifier> SHADEAdaptation<I> {
    pub fn from_params(history: usize) -> ExecResult<Self> {
        Ok(Self {
            history,
            id: PhantomId::default(),
        })
    }

    pub fn new_with_id<P>(history: usize) -> ExecResult<Box<dyn Component<P>>>
    where
        P: SingleObjectiveProblem + LimitedVectorProblem<Element = f64>,
    {
        Ok(Box::new(Self::from_params(history)?))
    }
}

impl SHADEAdaptation<Global> {
    pub fn new<P>(history: usize) -> ExecResult<Box<dyn Component<P>>>
    where
        P: SingleObjectiveProblem + LimitedVectorProblem<Element = f64>,
    {
        Self::new_with_id(history)
    }
}

impl<P, I> Component<P> for SHADEAdaptation<I>
where 
    P: SingleObjectiveProblem + LimitedVectorProblem<Element = f64>,
    I: Identifier,
{
    fn init(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
        // Initialise the history with all 0.5 for the specified lengths.
        state.insert(SHADEHistoryF::<I>::new(vec![0.5; self.history]));
        state.insert(SHADEHistoryCR::<I>::new(vec![0.5; self.history]));
        let mut current_fs = state.borrow_value_mut::<SHADEParamF<I>>();
        let mut current_crs = state.borrow_value_mut::<SHADEParamCR<I>>();
        let history_fs = state.borrow_value_mut::<SHADEHistoryF<I>>();
        let history_crs = state.borrow_value_mut::<SHADEHistoryCR<I>>();

        let mut rng = state.random_mut();

        // set the initial values of the F and CR to be used in the first iteration.
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
        // Draw new F and CR in each iteration after the history has been updated.
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

/// Update the history of the F and CR parameters at the end of every iteration.
#[derive(Clone, Serialize)]
pub struct SHADEAdaptationHistoryUpdate<I: Identifier = Global>
{
    pub k: usize,
    id: PhantomId<I>
}

impl<I: Identifier> SHADEAdaptationHistoryUpdate<I> {
    pub fn from_params() -> ExecResult<Self> {
        Ok(Self {k: 1, id: PhantomId::default()})
    }

    pub fn new<P>() -> ExecResult<Box<dyn Component<P>>>
    where
        P: SingleObjectiveProblem + LimitedVectorProblem<Element = f64>,
    {
        Ok(Box::new(Self::from_params()?))
    }
}

impl<P, I> Component<P> for SHADEAdaptationHistoryUpdate<I>
where
    P: SingleObjectiveProblem + LimitedVectorProblem<Element = f64>,
    I: Identifier,
{
    fn init(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
        state.insert(HistoryCounter::<Self>::new(self.k));
        Ok(())
    }
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

        let mut current_fs = state.get_value::<SHADEParamF<I>>();
        let mut current_crs = state.get_value::<SHADEParamCR<I>>();
        let mut f_history = state.get_value::<SHADEHistoryF<I>>();
        let mut cr_history = state.get_value::<SHADEHistoryCR<I>>();
        let mut k = state.get_value::<HistoryCounter<Self>>();
        let mut counter = state.borrow_value_mut::<HistoryCounter<Self>>();


        // get the indices where the offspring was better than the parents and the difference in fitness
        let mut indices = Vec::new();
        let mut differences = Vec::new();
        for (i, offspring) in o.into_iter().enumerate() {
            if p[i].objective() > offspring.objective() {
                indices.push(i);
                let diff = p[i].objective().value() - offspring.objective().value();
                differences.push(diff);
            }
        }

        // update the memory
        if indices.len() == 0 {
            f_history[k] = f_history[k - 1];
            cr_history[k] = cr_history[k - 1];
            *counter = 1usize;
        } else {
            // Calculate weights
            let sum = differences.iter().sum::<f64>();
            let weights = differences.iter().map(|d| d / sum).collect::<Vec<_>>();
            // Get F and CR values of successful individuals
            let sf_values = indices.iter().map(|i| current_fs[*i]).collect::<Vec<_>>();
            let scr_values = indices.iter().map(|i| current_crs[*i]).collect::<Vec<_>>();
            // Calculate weighted Lehmer mean for F
            let upper_sum = sf_values.iter().zip(&weights).map(|(f, w)| w * f.powi(2)).sum::<f64>();
            let lower_sum = sf_values.iter().zip(&weights).map(|(f, w)| w * f).sum::<f64>();
            f_history[k] = upper_sum / lower_sum;
            // Calculate weighted mean for CR
            cr_history[k] = scr_values.iter().zip(&weights).map(|(cr, w)| w * cr).sum::<f64>();
            *counter += 1;
        }
        
        Ok(())
    }
}

#[derive(Deref, DerefMut, Tid)]
pub struct HistoryCounter<T: AnyComponent + 'static>(
    #[deref]
    #[deref_mut]
    usize,
    PhantomData<T>,
);

impl<T: AnyComponent> HistoryCounter<T> {
    pub fn new(value: usize) -> Self {
        Self(value, PhantomData)
    }
}

impl<T: AnyComponent> CustomState<'_> for HistoryCounter<T> {}