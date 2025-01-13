//! Common replacement components.

use eyre::ensure;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};

use crate::{
    component::ExecResult,
    components::{
        replacement::{replacement, Replacement},
        Component,
    },
    problems::SingleObjectiveProblem,
    state::random::Random,
    Individual, Problem, State,
};

/// Discards all individuals in the child population, keeping the parents unchanged.
///
/// The opposite of this is [`Generational`].
#[derive(Clone, Serialize, Deserialize)]
pub struct DiscardOffspring;

impl DiscardOffspring {
    pub fn from_params() -> Self {
        Self
    }

    pub fn new<P: Problem>() -> Box<dyn Component<P>> {
        Box::new(Self::from_params())
    }
}

impl<P: Problem> Replacement<P> for DiscardOffspring {
    fn replace(
        &self,
        parents: Vec<Individual<P>>,
        _offspring: Vec<Individual<P>>,
        _rng: &mut Random,
    ) -> ExecResult<Vec<Individual<P>>> {
        Ok(parents)
    }
}

impl<P: Problem> Component<P> for DiscardOffspring {
    fn execute(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        replacement(self, problem, state)
    }
}

/// Merges the parent and child populations.
#[derive(Clone, Serialize, Deserialize)]
pub struct Merge;

impl Merge {
    pub fn from_params() -> Self {
        Self
    }

    pub fn new<P: Problem>() -> Box<dyn Component<P>> {
        Box::new(Self::from_params())
    }
}

impl<P: Problem> Replacement<P> for Merge {
    fn replace(
        &self,
        parents: Vec<Individual<P>>,
        offspring: Vec<Individual<P>>,
        _rng: &mut Random,
    ) -> ExecResult<Vec<Individual<P>>> {
        Ok(parents.into_iter().chain(offspring).collect())
    }
}

impl<P: Problem> Component<P> for Merge {
    fn execute(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        replacement(self, problem, state)
    }
}

/// Keeps the `max_population_size` fittest individuals from both populations.
#[derive(Clone, Serialize, Deserialize)]
pub struct MuPlusLambda {
    /// Maximal allowed population size.
    pub max_population_size: u32,
}

impl MuPlusLambda {
    pub fn from_params(max_population_size: u32) -> Self {
        Self {
            max_population_size,
        }
    }

    pub fn new<P: SingleObjectiveProblem>(max_population_size: u32) -> Box<dyn Component<P>> {
        Box::new(Self::from_params(max_population_size))
    }
}

impl<P: SingleObjectiveProblem> Replacement<P> for MuPlusLambda {
    fn replace(
        &self,
        mut parents: Vec<Individual<P>>,
        offspring: Vec<Individual<P>>,
        _rng: &mut Random,
    ) -> ExecResult<Vec<Individual<P>>> {
        parents.extend(offspring);
        parents.sort_unstable_by_key(|i| *i.objective());
        parents.truncate(self.max_population_size as usize);
        Ok(parents)
    }
}

impl<P: SingleObjectiveProblem> Component<P> for MuPlusLambda {
    fn execute(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        replacement(self, problem, state)
    }
}

/// Keeps the `max_population_size` fittest individuals from children.
#[derive(Clone, Serialize, Deserialize)]
pub struct MuCommaLambda {
    /// Maximal allowed population size.
    pub max_population_size: u32,
}

impl MuCommaLambda {
    pub fn from_params(max_population_size: u32) -> Self {
        Self {
            max_population_size,
        }
    }

    pub fn new<P: SingleObjectiveProblem>(max_population_size: u32) -> Box<dyn Component<P>> {
        Box::new(Self::from_params(max_population_size))
    }
}

impl<P: SingleObjectiveProblem> Replacement<P> for MuCommaLambda {
    fn replace(
        &self,
        _parents: Vec<Individual<P>>,
        offspring: Vec<Individual<P>>,
        _rng: &mut Random,
    ) -> ExecResult<Vec<Individual<P>>> {
        let mut population = offspring.clone();
        population.sort_unstable_by_key(|i| *i.objective());
        population.truncate(self.max_population_size as usize);
        Ok(population)
    }
}

impl<P: SingleObjectiveProblem> Component<P> for MuCommaLambda {
    fn execute(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        replacement(self, problem, state)
    }
}

/// Discards all individuals in the parent population, keeping the children unchanged.
///
/// The opposite of this is [`DiscardOffspring`].
///
/// If 'max_population_size' is given, the new population is simply truncated.
#[derive(Clone, Serialize, Deserialize)]
pub struct Generational {
    /// Maximal allowed population size.
    pub max_population_size: u32,
}

impl Generational {
    pub fn from_params(max_population_size: u32) -> Self {
        Self {
            max_population_size,
        }
    }

    pub fn new<P: SingleObjectiveProblem>(max_population_size: u32) -> Box<dyn Component<P>> {
        Box::new(Self::from_params(max_population_size))
    }
}

impl<P: Problem> Replacement<P> for Generational {
    fn replace(
        &self,
        _parents: Vec<Individual<P>>,
        offspring: Vec<Individual<P>>,
        _rng: &mut Random,
    ) -> ExecResult<Vec<Individual<P>>> {
        if offspring.len() > self.max_population_size as usize {
            let mut population = offspring.clone();
            population.truncate(self.max_population_size as usize);
            Ok(population)
        } else {
            Ok(offspring)
        }
    }
}

impl<P: Problem> Component<P> for Generational {
    fn execute(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        replacement(self, problem, state)
    }
}

/// Keeps `max_population_size` random individuals from parents and children without replacement.
#[derive(Clone, Serialize, Deserialize)]
pub struct RandomReplacement {
    pub max_population_size: u32,
}

impl RandomReplacement {
    pub fn from_params(max_population_size: u32) -> Self {
        Self {
            max_population_size,
        }
    }

    pub fn new<P: SingleObjectiveProblem>(max_population_size: u32) -> Box<dyn Component<P>> {
        Box::new(Self::from_params(max_population_size))
    }
}

impl<P: Problem> Replacement<P> for RandomReplacement {
    fn replace(
        &self,
        mut parents: Vec<Individual<P>>,
        offspring: Vec<Individual<P>>,
        rng: &mut Random,
    ) -> ExecResult<Vec<Individual<P>>> {
        parents.extend(offspring);
        parents.shuffle(rng);
        parents.truncate(self.max_population_size as usize);
        Ok(parents)
    }
}

impl<P: Problem> Component<P> for RandomReplacement {
    fn execute(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        replacement(self, problem, state)
    }
}

/// Keeps the better individual from parent and offspring at the same index.
///
/// # Errors
///
/// Returns an `Err` if the parent and child populations have different size.
#[derive(Clone, Serialize, Deserialize)]
pub struct KeepBetterAtIndex;

impl KeepBetterAtIndex {
    pub fn from_params() -> Self {
        Self
    }

    pub fn new<P: SingleObjectiveProblem>() -> Box<dyn Component<P>> {
        Box::new(Self::from_params())
    }
}

impl<P: SingleObjectiveProblem> Replacement<P> for KeepBetterAtIndex {
    fn replace(
        &self,
        parents: Vec<Individual<P>>,
        offspring: Vec<Individual<P>>,
        _rng: &mut Random,
    ) -> ExecResult<Vec<Individual<P>>> {
        ensure!(
            parents.len() == offspring.len(),
            "parents and offspring need to be the same size"
        );

        let population = parents
            .into_iter()
            .zip(offspring)
            .map(|(parent, offspring)| {
                if parent.objective() > offspring.objective() {
                    offspring
                } else {
                    parent
                }
            })
            .collect();
        Ok(population)
    }
}

impl<P: SingleObjectiveProblem> Component<P> for KeepBetterAtIndex {
    fn execute(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        replacement(self, problem, state)
    }
}
