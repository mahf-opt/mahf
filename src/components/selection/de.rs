//! Selection components for Differential Evolution (DE).

use eyre::{ensure, ContextCompat};
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};

use crate::{
    component::ExecResult,
    components::{
        selection::{functional as f, selection, Selection},
        Component,
    },
    problems::SingleObjectiveProblem,
    state::random::Random,
    Individual, Problem, State,
};

/// Selects `y * 2 + 1` random unique individuals for every individual in the population, keeping the order.
///
/// Originally proposed for, and used as selection in [`de`].
///
/// [`de`]: crate::heuristics::de
///
/// # Dependencies
///
/// This component is meant to be used together with [`DEMutation`], as it initializes the population
/// in a representation necessary to perform this special mutation.
///
/// [`DEMutation`]: crate::components::mutation::de::DEMutation
#[derive(Clone, Serialize, Deserialize)]
pub struct DERand {
    // Number of difference vectors ∈ {1, 2}.
    y: u32,
}

impl DERand {
    pub fn from_params(y: u32) -> ExecResult<Self> {
        ensure!(
            [1, 2].contains(&y),
            "`y` needs to be one of {{1, 2}}, but was {}",
            y
        );
        Ok(Self { y })
    }

    pub fn new<P: Problem>(y: u32) -> ExecResult<Box<dyn Component<P>>> {
        Ok(Box::new(Self::from_params(y)?))
    }
}

impl<P: Problem> Selection<P> for DERand {
    fn select<'a>(
        &self,
        population: &'a [Individual<P>],
        rng: &mut Random,
    ) -> ExecResult<Vec<&'a Individual<P>>> {
        let size = (self.y * 2 + 1) as usize;
        let selection = (0..population.len())
            .flat_map(|_| population.choose_multiple(rng, size))
            .collect();
        Ok(selection)
    }
}

impl<P: Problem> Component<P> for DERand {
    fn execute(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        selection(self, problem, state)
    }
}

/// Selects individuals in the form [best, `y * 2` random] for every individual in the population, keeping the order.
///
/// Originally proposed for, and used as selection in [`de`].
///
/// [`de`]: crate::heuristics::de
///
/// # Dependencies
///
/// This component is meant to be used together with [`DEMutation`], as it initializes the population
/// in a representation necessary to perform this special mutation.
///
/// [`DEMutation`]: crate::components::mutation::de::DEMutation
#[derive(Serialize, Deserialize, Clone)]
pub struct DEBest {
    // Number of difference vectors ∈ {1, 2}.
    y: u32,
}

impl DEBest {
    pub fn from_params(y: u32) -> ExecResult<Self> {
        ensure!(
            [1, 2].contains(&y),
            "`y` needs to be one of {{1, 2}}, but was {}",
            y
        );
        Ok(Self { y })
    }

    pub fn new<P: SingleObjectiveProblem>(y: u32) -> ExecResult<Box<dyn Component<P>>> {
        Ok(Box::new(Self::from_params(y)?))
    }
}

impl<P: SingleObjectiveProblem> Selection<P> for DEBest {
    fn select<'a>(
        &self,
        population: &'a [Individual<P>],
        rng: &mut Random,
    ) -> ExecResult<Vec<&'a Individual<P>>> {
        let size = (self.y * 2) as usize;
        let best = f::best(population).wrap_err("population is empty")?;
        let selection = (0..population.len())
            .flat_map(|_| {
                let mut selection = vec![best];
                selection.extend(population.choose_multiple(rng, size));
                selection
            })
            .collect();
        Ok(selection)
    }
}

impl<P: SingleObjectiveProblem> Component<P> for DEBest {
    fn execute(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        selection(self, problem, state)
    }
}

/// Selects individuals in the form [current, best, `y * 2 - 1` random] for every individual in the population, keeping the order.
///
/// Originally proposed for, and used as selection in [`de`].
///
/// [`de`]: crate::heuristics::de
///
/// # Dependencies
///
/// This component is meant to be used together with [`DEMutation`], as it initializes the population
/// in a representation necessary to perform this special mutation.
///
/// [`DEMutation`]: crate::components::mutation::de::DEMutation
#[derive(Clone, Serialize, Deserialize)]
pub struct DECurrentToBest {
    // Number of difference vectors ∈ {1, 2}.
    y: u32,
}

impl DECurrentToBest {
    pub fn from_params(y: u32) -> ExecResult<Self> {
        ensure!(
            [1, 2].contains(&y),
            "`y` needs to be one of {{1, 2}}, but was {}",
            y
        );
        Ok(Self { y })
    }

    pub fn new<P: SingleObjectiveProblem>(y: u32) -> ExecResult<Box<dyn Component<P>>> {
        Ok(Box::new(Self::from_params(y)?))
    }
}

impl<P: SingleObjectiveProblem> Selection<P> for DECurrentToBest {
    fn select<'a>(
        &self,
        population: &'a [Individual<P>],
        rng: &mut Random,
    ) -> ExecResult<Vec<&'a Individual<P>>> {
        let size = (self.y * 2 - 1) as usize;
        let best = f::best(population).wrap_err("population is empty")?;
        let selection = population
            .iter()
            .flat_map(|individual| {
                let mut selection = vec![individual, best];

                // Sample only individuals randomly that are not `individual`
                let remaining_population: Vec<_> =
                    population.iter().filter(|&i| i != individual).collect();

                selection.extend(remaining_population.choose_multiple(rng, size));
                selection
            })
            .collect();
        Ok(selection)
    }
}

impl<P: SingleObjectiveProblem> Component<P> for DECurrentToBest {
    fn execute(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        selection(self, problem, state)
    }
}
