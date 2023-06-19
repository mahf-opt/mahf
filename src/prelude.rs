//! The MAHF prelude imports the most relevant modules, structs and traits you may need for experiments.

pub use crate::{
    components::{
        self, evaluation, initialization, mapping, mutation, recombination, replacement, selection,
    },
    conditions,
    heuristics::*,
    logging,
    population::{
        AsSolutions, AsSolutionsMut, BestIndividual, IntoIndividuals, IntoSingle, IntoSingleRef,
        IntoSolutions,
    },
    problems,
    state::{self, common},
    ExecResult, MultiObjective, SingleObjective, ValueOf,
};
