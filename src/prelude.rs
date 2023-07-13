//! The MAHF prelude imports the most relevant modules, structs and traits you may need for experiments.

pub use crate::{
    components::{
        self, boundary, evaluation, generative, initialization, mapping, mutation, recombination,
        replacement, selection, swarm, Component,
    },
    conditions::{self, Condition},
    heuristics::*,
    identifier,
    lens::ValueOf,
    logging,
    population::{
        AsSolutions, AsSolutionsMut, BestIndividual, IntoIndividuals, IntoSingle, IntoSingleRef,
        IntoSolutions,
    },
    problems::{self, evaluate, ObjectiveFunction, Problem},
    state::{self, common, CustomState, State, StateReq},
    Configuration, ExecResult, MultiObjective, SingleObjective,
};
