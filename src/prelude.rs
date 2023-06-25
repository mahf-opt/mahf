//! The MAHF prelude imports the most relevant modules, structs and traits you may need for experiments.

pub use crate::{
    components::{
        self, evaluation, initialization, mapping, mutation, recombination, replacement, selection,
        Component,
    },
    conditions::{self, Condition},
    heuristics::*,
    logging,
    population::{
        AsSolutions, AsSolutionsMut, BestIndividual, IntoIndividuals, IntoSingle, IntoSingleRef,
        IntoSolutions,
    },
    problems::{self, Problem},
    state::{self, common, CustomState, State, StateReq},
    Configuration, ExecResult, MultiObjective, SingleObjective, ValueOf,
};
