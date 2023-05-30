//! This module contains instances of the symmetric traveling salesman problem.

use crate::{
    framework::{Individual, SingleObjective},
    problems::{
        tsp::{Coordinates, Dimension, Edge, Route},
        Evaluator, Problem, VectorProblem,
    },
    state::{common::EvaluatorInstance, State},
};
use anyhow::{anyhow, Result};
use include_dir::{include_dir, Dir};
use tspf::WeightKind;

static FILES: Dir = include_dir!("$CARGO_MANIFEST_DIR/src/problems/tsp/tsplib");

/// This enum represents built in instances of the symmetric travelling salesman problem.
#[derive(Debug, strum::EnumString, strum::AsRefStr)]
pub enum Instances {
    A280,
    ALI535,
    ATT48,
    ATT532,
    BAYG29,
    BAYS29,
    BERLIN52,
    BIER127,
    BRAZIL58,
    BRD14051,
    BRG180,
    BURMA14,
    CH130,
    CH150,
    D198,
    D493,
    D657,
    D1291,
    D1655,
    D2103,
    D15112,
    D18512,
    DANTZIG42,
    DSJ1000,
    EIL51,
    EIL76,
    EIL101,
    FL417,
    FL1400,
    FL1577,
    FL3795,
    FNL4461,
    FRI26,
    GIL262,
    GR17,
    GR21,
    GR24,
    GR48,
    GR96,
    GR120,
    GR137,
    GR202,
    GR229,
    GR431,
    GR666,
    HK48,
    KROA100,
    KROA150,
    KROA200,
    KROB100,
    KROB150,
    KROB200,
    KROC100,
    KROD100,
    KROE100,
    LIN105,
    LIN318,
    LINHP318,
    NRW1379,
    P654,
    PA561,
    PCB442,
    PCB1173,
    PCB3038,
    PLA7397,
    PLA33810,
    PLA85900,
    PR76,
    PR107,
    PR124,
    PR136,
    PR144,
    PR152,
    PR226,
    PR264,
    PR299,
    PR439,
    PR1002,
    PR2392,
    RAT99,
    RAT195,
    RAT575,
    RAT783,
    RD100,
    RD400,
    RL1304,
    RL1323,
    RL1889,
    RL5915,
    RL5934,
    RL11849,
    SI175,
    SI535,
    SI1032,
    ST70,
    SWISS42,
    TS225,
    TSP225,
    U159,
    U574,
    U724,
    U1060,
    U1432,
    U1817,
    U2152,
    U2319,
    ULYSSES16,
    ULYSSES22,
    USA13509,
    VM1084,
    VM1748,
}

impl std::fmt::Display for Instances {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl Instances {
    /// Tries to load the built-in instance.
    pub fn try_load(&self) -> Result<SymmetricTsp> {
        let name = self.as_ref().to_lowercase();

        let data = FILES
            .get_file(format!("{}.tsp", name))
            .unwrap()
            .contents_utf8()
            .unwrap();

        let opt = FILES
            .get_file(format!("{}.opt.tour", name))
            .map(|file| file.contents_utf8().unwrap());

        // Update name with enum if tsplib name is empty
        let mut tsp = SymmetricTsp::try_parse(data, opt)?;
        if tsp.name.is_empty() {
            tsp.name = self.to_string();
        }
        Ok(tsp)
    }

    /// Loads the built-in instance. This method should by design never panic.
    pub fn load(&self) -> SymmetricTsp {
        self.try_load().expect("Error while constructing instance")
    }
}

/// Represents the (global) optimum of the search space.
#[derive(Debug, Clone, serde::Serialize)]
pub struct TspOptimum {
    pub objective: SingleObjective,
    pub solution: Option<Route>,
}

/// Represents an instance of the symmetric travelling salesman problem.
#[derive(serde::Serialize)]
pub struct SymmetricTsp {
    /// Name of the instance
    pub name: String,
    /// Dimension of the instance
    pub dimension: Dimension,
    /// Best possible solution
    pub best_solution: Option<TspOptimum>,
    /// The cities coordinates
    #[serde(skip)]
    pub positions: Vec<Coordinates>,
    /// Inner tspf instance for distance measuring
    #[serde(skip)]
    inner: tspf::Tsp,
}

struct SymmetricTspEvaluator;

impl Evaluator for SymmetricTspEvaluator {
    type Problem = SymmetricTsp;

    fn evaluate(
        &mut self,
        problem: &Self::Problem,
        _state: &mut State<Self::Problem>,
        individuals: &mut [Individual<Self::Problem>],
    ) {
        for individual in individuals {
            individual.evaluate(problem.evaluate_solution(individual.solution()));
        }
    }
}

impl SymmetricTsp {
    fn evaluate_solution(&self, solution: &Route) -> SingleObjective {
        solution
            .iter()
            .copied()
            .zip(solution.iter().copied().skip(1))
            .map(|edge| self.distance(edge))
            .sum::<f64>()
            .try_into()
            .unwrap()
    }
}

impl Problem for SymmetricTsp {
    type Encoding = Route;
    type Objective = SingleObjective;

    fn name(&self) -> &str {
        "SymmetricTsp"
    }

    fn default_evaluator<'a>(&self) -> EvaluatorInstance<'a, Self> {
        EvaluatorInstance::functional(|problem, _state, individuals| {
            for individual in individuals {
                individual.evaluate(problem.evaluate_solution(individual.solution()));
            }
        })
    }
}

impl VectorProblem for SymmetricTsp {
    type T = usize;

    fn dimension(&self) -> usize {
        self.dimension
    }
}

impl SymmetricTsp {
    pub fn best_fitness(&self) -> Option<f64> {
        self.best_solution.as_ref().map(|o| o.objective.value())
    }

    /// Returns the weight/distance of the given edge.
    pub fn distance(&self, edge: Edge) -> f64 {
        let (a, b) = edge;

        // TODO: this seems like a bug in tspf
        if self.inner.weight_kind() == WeightKind::Explicit {
            self.inner.weight(a, b)
        } else {
            self.inner.weight(a + 1, b + 1)
        }
    }

    /// Greedily constructs a Route, always taking the shortest edge.
    pub fn greedy_route(&self) -> Route {
        let mut route = Vec::with_capacity(self.dimension);
        let mut remaining = (1..self.dimension).into_iter().collect::<Vec<usize>>();
        route.push(0);
        while !remaining.is_empty() {
            let last = *route.last().unwrap();
            let next_index = remaining
                .iter()
                .enumerate()
                .min_by_key(|(_, &r)| SingleObjective::try_from(self.distance((last, r))).unwrap())
                .unwrap()
                .0;
            let next = remaining.remove(next_index);
            route.push(next);
        }
        route
    }

    /// This method constructs a TSP instance from a string representation (`data`) and an optional solution (`opt`).
    /// There is no good reason to call it directly, just use `SymmetricTspInstances.try_load()` instead.
    fn try_parse(data: &str, opt: Option<&str>) -> Result<Self> {
        let tsp =
            tspf::TspBuilder::parse_str(data).map_err(|err| anyhow!("parsing failed: {}", err))?;

        let mut positions = vec![vec![]; tsp.dim()];
        for (index, point) in tsp.node_coords().iter() {
            positions[index - 1] = point.pos().clone();
        }

        let mut instance = SymmetricTsp {
            name: tsp.name().clone(),
            dimension: tsp.dim(),
            best_solution: None,
            positions,
            inner: tsp,
        };

        if let Some(opt) = opt {
            instance.best_solution = Some(parse_opt_file(&instance, opt));
        }

        Ok(instance)
    }
}

fn parse_opt_file(instance: &SymmetricTsp, opt_contents: &str) -> TspOptimum {
    let best_solution = opt_contents
        .lines()
        .find(|line| line.starts_with("BEST_SOLUTION"))
        .map(|line| &line["BEST_SOLUTION: ".len()..])
        .map(str::parse::<f64>)
        .map(Result::unwrap)
        .map(SingleObjective::try_from)
        .map(Result::unwrap);

    let route = opt_contents
        .lines()
        .skip_while(|&line| line != "TOUR_SECTION")
        .skip(1)
        .take_while(|&line| line != "-1")
        .map(str::parse::<usize>)
        .map(Result::unwrap)
        .map(|x| x - 1)
        .collect::<Route>();

    let solution = if route.is_empty() { None } else { Some(route) };

    assert!(best_solution.is_some() || solution.is_some());

    let objective =
        best_solution.unwrap_or_else(|| instance.evaluate_solution(solution.as_ref().unwrap()));

    TspOptimum {
        objective,
        solution,
    }
}

/// Calculates the 2D-Euclidean distance between two points.
#[inline]
pub fn rounded_euc_2d(a: &[f64], b: &[f64]) -> f64 {
    tspf::metric::euc_2d(a, b).round()
}

#[cfg(test)]
mod tests {
    use super::*;
    use float_eq::assert_float_eq;

    #[test]
    fn loading_berlin52() {
        let opt_tour = vec![
            0, 48, 31, 44, 18, 40, 7, 8, 9, 42, 32, 50, 10, 51, 13, 12, 46, 25, 26, 27, 11, 24, 3,
            5, 14, 4, 23, 47, 37, 36, 39, 38, 35, 34, 33, 43, 45, 15, 28, 49, 19, 22, 29, 1, 6, 41,
            20, 16, 2, 17, 30, 21,
        ];
        let tsp = Instances::BERLIN52.load();
        let best_solution = tsp.best_solution.unwrap();

        assert_eq!(tsp.dimension, 52);
        assert_float_eq!(best_solution.objective.value(), 7498.0, abs <= 1.0);
        assert_eq!(best_solution.solution.unwrap(), opt_tour);
    }

    #[test]
    fn loading_bier127() {
        let tsp = Instances::BIER127.load();
        assert_eq!(tsp.best_solution.unwrap().objective.value(), 118282.0);
    }
}
