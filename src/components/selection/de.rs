//! Selection components for Differential Evolution (DE).

use better_any::{Tid, TidAble};
use derive_more::{Deref, DerefMut};
use eyre::{ensure, ContextCompat};
use rand::prelude::IteratorRandom;
use rand::Rng;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};

use crate::{component::ExecResult, components::{
    selection::{functional as f, selection, Selection},
    Component,
}, problems::SingleObjectiveProblem, state::random::Random, CustomState, Individual, Problem, State};
use crate::components::archive::DEKeepParentsArchive;
use crate::prelude::StateReq;

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

/// Selects individuals in the form [current, pbest, `y * 2 - 1` random] for every individual in the population, keeping the order.
///
/// Originally proposed for, and used as selection in [`de`], specifically SHADE.
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
pub struct SHADECurrentToPBest {
    // Number of difference vectors ∈ {1, 2}.
    y: u32,
    // Minimum value of p ∈ [0,1].
    p_min: f64,
    // Population size to initialize `IndividualP` as vec with p for each individual.
    pop_size: usize,
    // Maximum number of individuals that can be added through `DEKeepParentsArchive`.
    // Set to 0 if no archive is configured.
    max_archive: usize,
}

impl SHADECurrentToPBest {
    pub fn from_params(y: u32, p_min: f64, pop_size: usize, max_archive: usize) -> ExecResult<Self> {
        ensure!(
            [1, 2].contains(&y),
            "`y` needs to be one of {{1, 2}}, but was {}",
            y
        );
        Ok(Self { y, p_min, pop_size, max_archive })
    }

    pub fn new<P: SingleObjectiveProblem>(y: u32, p_min: f64, pop_size: usize, max_archive: usize) -> ExecResult<Box<dyn Component<P>>> {
        Ok(Box::new(Self::from_params(y , p_min, pop_size, max_archive)?))
    }
}

impl<P: SingleObjectiveProblem> Component<P> for SHADECurrentToPBest {
    fn init(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
        let p = std::iter::repeat_with(|| state.random_mut().gen_range(self.p_min..=0.2))
            .take(self.pop_size).collect::<Vec<_>>();
        state.insert(IndividualP(p));
        Ok(())
    }
    fn execute(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
        let mut populations = state.populations_mut();
        let mut rng = state.random_mut();
        let current = populations.current();
        let size = (self.y * 2 - 1) as usize;
        let ps = state.get_value::<IndividualP>();
        let mut sorted = current.iter().collect::<Vec<_>>();
        sorted.sort_unstable_by_key(|i| *i.objective());
        let pop_size = current.len() as f64;
        
        let mut pbests = Vec::new();
        for p in ps {
            let max_index = (p * pop_size) as usize;
            let random_index = &mut rng.gen_range(0..=max_index);
            pbests.push(sorted[*random_index]);
        }
        
        // For now, this operator requires an DEKeepParentsArchive to be present, even if it's empty.
        let archive = state.borrow::<DEKeepParentsArchive<P>>();
        let archived_population = archive.parents();
        
        let selection: Vec<_> = if self.max_archive == 0 { 
            current
                .iter()
                .zip(pbests)
                .flat_map(|(individual, pbest)| {
                    let mut selection = vec![individual, pbest];
    
                    // Sample only individuals randomly that are not `individual`
                    let remaining_population: Vec<_> =
                        current.iter().filter(|&i| i != individual).collect();
                    selection.extend(remaining_population.choose_multiple(&mut *rng, size));
                    selection
                })
                .collect()
            } else {
            current
                .iter()
                .zip(pbests)
                .flat_map(|(individual, pbest)| {
                    let mut selection = vec![individual, pbest];

                    // Sample only individuals randomly that are not `individual`
                    let remaining_population: Vec<_> =
                        current.iter().filter(|&i| i != individual).collect();
                    selection.extend(remaining_population.choose_multiple(&mut *rng, size-1));

                    // Additionally sample one individual from the combination with the archive
                    let mut combined_population: Vec<&Individual<P>> = Vec::new();
                    for i in archived_population {
                        combined_population.push(&i);
                    }
                    for j in remaining_population {
                        combined_population.push(j);
                    }
                    selection.extend(combined_population.choose_multiple(&mut *rng, 1));
                    selection
                })
                .collect()
            };

        // generate new probabilities for next iteration
        let p = std::iter::repeat_with(|| rng.gen_range(self.p_min..=0.2))
            .take(current.len()).collect::<Vec<_>>();
        state.set_value::<IndividualP>(p);
        
        // push selection to population stack
        let cloned_selection = selection.into_iter().cloned().collect();
        populations.push(cloned_selection);
        
        Ok(())
    }
}

/// The vector of selection parameters p for SHADE.
#[derive(Default, Tid, Deref, DerefMut)]
pub struct IndividualP(
    pub Vec<f64>,
);

impl CustomState<'_> for IndividualP {}
