//! Replacement operators that can be applied to PSO.
//! While typically PSO does not need a replacement operator, certain hybridization approaches do.
//! In these cases, it is necessary to keep the relation of the solutions to their velocities.

use eyre::{ensure};
use rand::Rng;
use rand::seq::SliceRandom;
use serde::{Serialize};

use crate::{
    identifier::{Global, Identifier, PhantomId},
    problems::LimitedVectorProblem, Component, ExecResult,
    SingleObjectiveProblem, State,
};
use crate::components::swarm::pso::{ParticleVelocities};
use crate::prelude::StateReq;

/// Replaces the [`n_worst`] individuals in a PSO population with the same number of offspring.
/// 
/// Keeps the former [`ParticleVelocities`] associated to the respective individuals.
/// Generates new random [`ParticleVelocities`] for the offspring.
#[derive(Clone, Serialize)]
pub struct ReplaceNWorstPSO<I: Identifier = Global> {
    pub n_worst: u32,
    pub v_max: f64,
    id: PhantomId<I>,
}

impl<I: Identifier> ReplaceNWorstPSO<I> {
    pub fn from_params(n_worst: u32, v_max: f64) -> Self {
        Self {
            n_worst,
            v_max,
            id: PhantomId::default(),
        }
    }
    pub fn new_with_id<P>(n_worst: u32, v_max: f64) -> Box<dyn Component<P>>
    where
        P: LimitedVectorProblem<Element = f64>,
        P: SingleObjectiveProblem,
    {
        Box::new(Self::from_params(n_worst, v_max))
    }
}

impl ReplaceNWorstPSO<Global> {
    pub fn new<P>(n_worst: u32, v_max: f64) -> Box<dyn Component<P>>
    where
        P: LimitedVectorProblem<Element = f64>,
        P: SingleObjectiveProblem,
    {
        Self::new_with_id(n_worst, v_max)
    }
}

impl<P, I> Component<P> for ReplaceNWorstPSO<I>
where
    P: LimitedVectorProblem<Element = f64>,
    P: SingleObjectiveProblem,
    I: Identifier,
{
    fn require(&self, _problem: &P, state_req: &StateReq<P>) -> ExecResult<()> {
        state_req.require::<Self, ParticleVelocities<I>>()?;
        Ok(())
    }
    
    fn execute(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        let mut populations = state.populations_mut();
        let mut offspring = populations.pop();
        let mut parents = populations.pop();
        let mut velocities = state.borrow_value_mut::<ParticleVelocities<I>>();
        
        ensure!(
            parents.len() >= self.n_worst as usize,
            "In PSO replacement: more individuals to replace than available in parents ({} vs. {})",
            parents.len(),
            self.n_worst as usize
        );

        ensure!(
            offspring.len() == self.n_worst as usize,
            "In PSO replacement: not the same offspring to replace than removed from parents ({} vs. {})",
            offspring.len(),
            self.n_worst as usize
        );
        
        ensure!(
            parents.len() == velocities.len(),
            "In PSO replacement: number of particles and velocities do not match ({} vs. {})",
            parents.len(),
            velocities.len()
        );
        
        // Get indices sorted by parents objective values (from best to worst)
        let mut indices = (0..parents.len()).collect::<Vec<_>>();
        indices.sort_unstable_by_key(|&i| *parents[i].objective());
        // split vec of indices to keep only the n_worst indices in remove_indices
        let mut remove_indices = indices.split_off(self.n_worst as usize);
        // Sort and reverse the remaining indices to be able to remove those individuals from the
        // parents without changing the index
        remove_indices.sort();
        let sorted_indices = remove_indices.iter().rev();

        ensure!(
            sorted_indices.len() == self.n_worst as usize,
            "In PSO replacement: not the same sorted indices as specified ({} vs. {})",
            sorted_indices.len(),
            self.n_worst as usize
        );
        
        // remove every individual and velocity from respective Vec that is indicated by index
        for i in sorted_indices {
            parents.remove(*i);
            velocities.remove(*i);
        }
        
        // initialise velocities of offspring
        let mut new_velocities = std::iter::repeat_with(|| {
            std::iter::repeat_with(|| state.random_mut().gen_range(-self.v_max..=self.v_max))
                .take(problem.dimension())
                .collect::<Vec<_>>()
        })
            .take(offspring.len())
            .collect::<Vec<_>>();
        // add offspring to reduced parent population and offspring velocities to parent velocities
        parents.append(&mut offspring);
        velocities.append(&mut new_velocities);
        
        // push new population in state
        populations.push(parents);
        Ok(())
    }
}

/// Replaces the [`n_best`] individuals in a PSO population with the same number of offspring.
///
/// Keeps the former [`ParticleVelocities`] associated to the respective individuals.
/// Generates new random [`ParticleVelocities`] for the offspring.
#[derive(Clone, Serialize)]
pub struct ReplaceNBestPSO<I: Identifier = Global> {
    pub n_best: u32,
    pub v_max: f64,
    id: PhantomId<I>,
}

impl<I: Identifier> ReplaceNBestPSO<I> {
    pub fn from_params(n_best: u32, v_max: f64) -> Self {
        Self {
            n_best,
            v_max,
            id: PhantomId::default(),
        }
    }
    pub fn new_with_id<P>(n_best: u32, v_max: f64) -> Box<dyn Component<P>>
    where
        P: LimitedVectorProblem<Element = f64>,
        P: SingleObjectiveProblem,
    {
        Box::new(Self::from_params(n_best, v_max))
    }
}

impl ReplaceNBestPSO<Global> {
    pub fn new<P>(n_best: u32, v_max: f64) -> Box<dyn Component<P>>
    where
        P: LimitedVectorProblem<Element = f64>,
        P: SingleObjectiveProblem,
    {
        Self::new_with_id(n_best, v_max)
    }
}

impl<P, I> Component<P> for ReplaceNBestPSO<I>
where
    P: LimitedVectorProblem<Element = f64>,
    P: SingleObjectiveProblem,
    I: Identifier,
{
    fn require(&self, _problem: &P, state_req: &StateReq<P>) -> ExecResult<()> {
        state_req.require::<Self, ParticleVelocities<I>>()?;
        Ok(())
    }

    fn execute(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        let mut populations = state.populations_mut();
        let mut offspring = populations.pop();
        let mut parents = populations.pop();
        let mut velocities = state.borrow_value_mut::<ParticleVelocities<I>>();

        ensure!(
            parents.len() >= self.n_best as usize,
            "In PSO replacement: more individuals to replace than available in parents ({} vs. {})",
            parents.len(),
            self.n_best as usize
        );

        ensure!(
            offspring.len() == self.n_best as usize,
            "In PSO replacement: not the same offspring to replace than removed from parents ({} vs. {})",
            offspring.len(),
            self.n_best as usize
        );

        ensure!(
            parents.len() == velocities.len(),
            "In PSO replacement: number of particles and velocities do not match ({} vs. {})",
            parents.len(),
            velocities.len()
        );

        // Get indices sorted by parents objective values (from best to worst)
        let mut indices = (0..parents.len()).collect::<Vec<_>>();
        indices.sort_unstable_by_key(|&i| *parents[i].objective());
        // split vec of indices to keep only the n_best indices in _remove_indices
        let _remove_indices = indices.split_off(self.n_best as usize);
        // Sort and reverse the remaining indices to be able to remove those individuals from the
        // parents without changing the index
        indices.sort();
        let sorted_indices = indices.iter().rev();

        ensure!(
            sorted_indices.len() == self.n_best as usize,
            "In PSO replacement: not the same sorted indices as specified ({} vs. {})",
            sorted_indices.len(),
            self.n_best as usize
        );
        
        // remove every individual and velocity from respective Vec that is indicated by index
        for i in sorted_indices {
            parents.remove(*i);
            velocities.remove(*i);
        }

        // initialise velocities of offspring
        let mut new_velocities = std::iter::repeat_with(|| {
            std::iter::repeat_with(|| state.random_mut().gen_range(-self.v_max..=self.v_max))
                .take(problem.dimension())
                .collect::<Vec<_>>()
        })
            .take(offspring.len())
            .collect::<Vec<_>>();
        // add offspring to reduced parent population and offspring velocities to parent velocities
        parents.append(&mut offspring);
        velocities.append(&mut new_velocities);

        // push new population in state
        populations.push(parents);
        Ok(())
    }
}

/// Replaces [`n_random`] individuals in a PSO population with the same number of offspring.
///
/// Keeps the former [`ParticleVelocities`] associated to the respective individuals.
/// Generates new random [`ParticleVelocities`] for the offspring.
#[derive(Clone, Serialize)]
pub struct ReplaceNRandomPSO<I: Identifier = Global> {
    pub n_random: u32,
    pub v_max: f64,
    id: PhantomId<I>,
}

impl<I: Identifier> ReplaceNRandomPSO<I> {
    pub fn from_params(n_random: u32, v_max: f64) -> Self {
        Self {
            n_random,
            v_max,
            id: PhantomId::default(),
        }
    }
    pub fn new_with_id<P>(n_random: u32, v_max: f64) -> Box<dyn Component<P>>
    where
        P: LimitedVectorProblem<Element = f64>,
        P: SingleObjectiveProblem,
    {
        Box::new(Self::from_params(n_random, v_max))
    }
}

impl ReplaceNRandomPSO<Global> {
    pub fn new<P>(n_random: u32, v_max: f64) -> Box<dyn Component<P>>
    where
        P: LimitedVectorProblem<Element = f64>,
        P: SingleObjectiveProblem,
    {
        Self::new_with_id(n_random, v_max)
    }
}

impl<P, I> Component<P> for ReplaceNRandomPSO<I>
where
    P: LimitedVectorProblem<Element = f64>,
    P: SingleObjectiveProblem,
    I: Identifier,
{
    fn require(&self, _problem: &P, state_req: &StateReq<P>) -> ExecResult<()> {
        state_req.require::<Self, ParticleVelocities<I>>()?;
        Ok(())
    }

    fn execute(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        let mut populations = state.populations_mut();
        let mut offspring = populations.pop();
        let mut parents = populations.pop();
        let mut velocities = state.borrow_value_mut::<ParticleVelocities<I>>();

        ensure!(
            parents.len() >= self.n_random as usize,
            "In PSO replacement: more individuals to replace than available in parents ({} vs. {})",
            parents.len(),
            self.n_random as usize
        );

        ensure!(
            offspring.len() == self.n_random as usize,
            "In PSO replacement: not the same offspring to replace than removed from parents ({} vs. {})",
            offspring.len(),
            self.n_random as usize
        );

        ensure!(
            parents.len() == velocities.len(),
            "In PSO replacement: number of particles and velocities do not match ({} vs. {})",
            parents.len(),
            velocities.len()
        );

        // Get random indices of PSO population
        let mut rng = state.random_mut();
        let mut indices = (0..parents.len()).collect::<Vec<_>>();
        indices.shuffle(&mut *rng);
        // split vec of indices to keep only the n_random indices
        let _redundant_indices = indices.split_off(self.n_random as usize);
        // Sort and reverse the remaining indices to be able to remove those individuals from the
        // parents without changing the index
        indices.sort();
        let sorted_indices = indices.iter().rev();

        ensure!(
            sorted_indices.len() == self.n_random as usize,
            "In PSO replacement: not the same sorted indices as specified ({} vs. {})",
            sorted_indices.len(),
            self.n_random as usize
        );
        
        // remove every individual and velocity from respective Vec that is indicated by index
        for i in sorted_indices {
            parents.remove(*i);
            velocities.remove(*i);
        }

        // initialise velocities of offspring
        let mut new_velocities = std::iter::repeat_with(|| {
            std::iter::repeat_with(|| rng.gen_range(-self.v_max..=self.v_max))
                .take(problem.dimension())
                .collect::<Vec<_>>()
        })
            .take(offspring.len())
            .collect::<Vec<_>>();
        // add offspring to reduced parent population and offspring velocities to parent velocities
        parents.append(&mut offspring);
        velocities.append(&mut new_velocities);

        // push new population in state
        populations.push(parents);
        Ok(())
    }
}

