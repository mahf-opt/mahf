use std::marker::PhantomData;

use better_any::{Tid, TidAble};
use derive_more::{Deref, DerefMut};
use eyre::{ensure, ContextCompat};
use itertools::multizip;
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::{
    component::{AnyComponent, ExecResult},
    components::Component,
    identifier::{Global, Identifier},
    population::{AsSolutions, AsSolutionsMut, BestIndividual},
    problems::{LimitedVectorProblem, SingleObjectiveProblem},
    state::StateReq,
    CustomState, Individual, Problem, State,
};

#[derive(Deref, DerefMut, Tid)]
pub struct ParticleVelocities<I: Identifier + 'static>(
    #[deref]
    #[deref_mut]
    Vec<Vec<f64>>,
    PhantomData<I>,
);

impl<I: Identifier> ParticleVelocities<I> {
    pub fn new(value: Vec<Vec<f64>>) -> Self {
        Self(value, PhantomData)
    }
}

impl<I: Identifier> CustomState<'_> for ParticleVelocities<I> {}

#[derive(Clone, Serialize, Deserialize)]
pub struct ParticleVelocitiesInit<I: Identifier = Global> {
    pub v_max: f64,
    phantom: PhantomData<I>,
}

impl<I: Identifier> ParticleVelocitiesInit<I> {
    pub fn from_params(v_max: f64) -> ExecResult<Self> {
        ensure!(v_max > 0., "`v_max` must be > 0, but was {}", v_max);
        Ok(Self {
            v_max,
            phantom: PhantomData,
        })
    }

    pub fn new<P>(v_max: f64) -> ExecResult<Box<dyn Component<P>>>
    where
        P: LimitedVectorProblem<Element = f64>,
    {
        Ok(Box::new(Self::from_params(v_max)?))
    }
}

impl<P, I> Component<P> for ParticleVelocitiesInit<I>
where
    P: LimitedVectorProblem<Element = f64>,
    I: Identifier,
{
    fn init(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
        state.insert(ParticleVelocities::<I>::new(Vec::new()));
        Ok(())
    }

    fn execute(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        let velocities = std::iter::repeat_with(|| {
            std::iter::repeat_with(|| state.random_mut().gen_range(-self.v_max..=self.v_max))
                .take(problem.dimension())
                .collect::<Vec<_>>()
        })
        .take(state.populations().current().len())
        .collect::<Vec<_>>();

        state.set_value::<ParticleVelocities<I>>(velocities);

        Ok(())
    }
}

#[derive(Deref, DerefMut, Tid)]
pub struct InertiaWeight<T: AnyComponent + 'static>(
    #[deref]
    #[deref_mut]
    f64,
    PhantomData<T>,
);

impl<T: AnyComponent> InertiaWeight<T> {
    pub fn new(value: f64) -> Self {
        Self(value, PhantomData)
    }
}

impl<T: AnyComponent> CustomState<'_> for InertiaWeight<T> {}

#[derive(Clone, Serialize, Deserialize)]
pub struct ParticleVelocitiesUpdate<I: Identifier = Global> {
    pub weight: f64,
    pub c_1: f64,
    pub c_2: f64,
    pub v_max: f64,
    phantom: PhantomData<I>,
}

impl<I: Identifier> ParticleVelocitiesUpdate<I> {
    pub fn from_params(weight: f64, c_1: f64, c_2: f64, v_max: f64) -> ExecResult<Self> {
        ensure!(weight > 0., "`weight` must be > 0, but was {}", weight);
        ensure!(c_1 > 0., "`c_1` must be > 0, but was {}", c_1);
        ensure!(c_2 > 0., "`c_2` must be > 0, but was {}", c_2);
        ensure!(v_max > 0., "`v_max` must be > 0, but was {}", v_max);
        Ok(Self {
            weight,
            c_1,
            c_2,
            v_max,
            phantom: PhantomData,
        })
    }

    pub fn new<P>(weight: f64, c_1: f64, c_2: f64, v_max: f64) -> ExecResult<Box<dyn Component<P>>>
    where
        P: LimitedVectorProblem<Element = f64>,
    {
        Ok(Box::new(Self::from_params(weight, c_1, c_2, v_max)?))
    }
}

impl<P, I> Component<P> for ParticleVelocitiesUpdate<I>
where
    P: LimitedVectorProblem<Element = f64>,
    I: Identifier,
{
    fn init(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
        state.insert(InertiaWeight::<Self>::new(self.weight));
        Ok(())
    }

    fn require(&self, _problem: &P, state_req: &StateReq) -> ExecResult<()> {
        state_req.require::<Self, BestParticles<P, I>>()?;
        state_req.require::<Self, BestParticle<P, I>>()?;
        Ok(())
    }

    fn execute(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
        let mut populations = state.populations_mut();
        let mut rng = state.random_mut();

        // Prepare parameters
        let &Self {
            c_1, c_2, v_max, ..
        } = self;
        let w = state.get_value::<InertiaWeight<Self>>();

        let mut rand = || rng.gen::<f64>();

        // Get necessary state like velocities `v`, personal bests `xp`, global best `xg`
        let xs = populations.current_mut().as_solutions_mut();
        let mut vs = state.borrow_value_mut::<ParticleVelocities<I>>();
        ensure!(
            vs.len() == xs.len(),
            "the number of particles and particle velocities is different ({} vs. {})",
            vs.len(),
            xs.len()
        );
        let xps = state.borrow_value::<BestParticles<P, I>>();
        ensure!(
            xps.len() == xs.len(),
            "the number of particles and local best particles is different ({} vs. {})",
            xps.len(),
            xs.len()
        );
        let best = state.borrow_value::<BestParticle<P, I>>();
        let xg = best.as_ref().wrap_err("global best is missing")?.solution();

        // Perform the update step.
        for (x, v, xp) in multizip((xs, &mut *vs, xps.as_solutions())) {
            for i in 0..v.len() {
                // Update and clamp velocity
                v[i] = w * v[i] + c_1 * rand() * (xp[i] - x[i]) + c_2 * rand() * (xg[i] - x[i]);
                v[i] = v[i].clamp(-v_max, v_max);
                // Add velocity to particle position
                x[i] += v[i];
            }
        }

        Ok(())
    }
}

#[derive(Deref, DerefMut, Tid)]
pub struct BestParticles<P: Problem + 'static, I: Identifier + 'static>(
    #[deref]
    #[deref_mut]
    Vec<Individual<P>>,
    PhantomData<I>,
);

impl<P: Problem, I: Identifier> BestParticles<P, I> {
    pub fn new(value: Vec<Individual<P>>) -> Self {
        Self(value, PhantomData)
    }
}

impl<P: Problem, I: Identifier> CustomState<'_> for BestParticles<P, I> {}

#[derive(Clone, Serialize, Deserialize)]
pub struct PersonalBestParticlesInit<I: Identifier = Global> {
    phantom: PhantomData<I>,
}

impl<I: Identifier> PersonalBestParticlesInit<I> {
    pub fn from_params() -> Self {
        Self {
            phantom: PhantomData,
        }
    }

    pub fn new<P>() -> Box<dyn Component<P>>
    where
        P: LimitedVectorProblem<Element = f64>,
    {
        Box::new(Self::from_params())
    }
}

impl<P, I> Component<P> for PersonalBestParticlesInit<I>
where
    P: LimitedVectorProblem<Element = f64>,
    I: Identifier,
{
    fn init(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
        state.insert(BestParticles::<P, I>::new(Vec::new()));
        Ok(())
    }

    fn execute(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
        state.set_value::<BestParticles<P, I>>(state.populations().current().to_owned());
        Ok(())
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PersonalBestParticlesUpdate<I: Identifier = Global> {
    phantom: PhantomData<I>,
}

impl<I: Identifier> PersonalBestParticlesUpdate<I> {
    pub fn from_params() -> Self {
        Self {
            phantom: PhantomData,
        }
    }

    pub fn new<P>() -> Box<dyn Component<P>>
    where
        P: LimitedVectorProblem<Element = f64>,
    {
        Box::new(Self::from_params())
    }
}

impl<P, I> Component<P> for PersonalBestParticlesUpdate<I>
where
    P: LimitedVectorProblem<Element = f64>,
    I: Identifier,
{
    fn execute(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
        let populations = state.populations();
        let mut bests = state.borrow_value_mut::<BestParticles<P, I>>();

        for (current, candidate) in multizip((&mut *bests, populations.current())) {
            if candidate.objective() < current.objective() {
                *current = candidate.clone();
            }
        }

        Ok(())
    }
}

#[derive(Deref, DerefMut, Tid)]
pub struct BestParticle<P: Problem + 'static, I: Identifier + 'static = Global>(
    #[deref]
    #[deref_mut]
    Option<Individual<P>>,
    PhantomData<I>,
);

impl<P: Problem, I: Identifier> BestParticle<P, I> {
    pub fn new(value: Option<Individual<P>>) -> Self {
        Self(value, PhantomData)
    }
}

impl<P: Problem, I: Identifier> CustomState<'_> for BestParticle<P, I> {}

#[derive(Clone, Serialize, Deserialize)]
pub struct GlobalBestParticleUpdate<I: Identifier = Global> {
    phantom: PhantomData<I>,
}

impl<I: Identifier> GlobalBestParticleUpdate<I> {
    pub fn from_params() -> Self {
        Self {
            phantom: PhantomData,
        }
    }

    pub fn new<P>() -> Box<dyn Component<P>>
    where
        P: SingleObjectiveProblem + LimitedVectorProblem<Element = f64>,
    {
        Box::new(Self::from_params())
    }
}

impl<P, I> Component<P> for GlobalBestParticleUpdate<I>
where
    P: SingleObjectiveProblem + LimitedVectorProblem<Element = f64>,
    I: Identifier,
{
    fn init(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
        state
            .entry::<BestParticle<P, I>>()
            .or_insert(BestParticle::<P, I>::new(None));
        Ok(())
    }

    fn execute(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
        let mut best = state.borrow_value_mut::<BestParticle<P, I>>();
        let candidate = state.populations().current().best_individual().cloned();

        match (&mut *best, candidate) {
            (Some(current), Some(candidate)) => {
                if candidate.objective() < current.objective() {
                    *current = candidate;
                }
            }
            (current, Some(candidate)) => *current = Some(candidate),
            _ => {}
        }

        Ok(())
    }
}
