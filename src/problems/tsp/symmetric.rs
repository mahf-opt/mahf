//! This module contains instances of the symmetric traveling salesman problem.

use crate::{
    fitness::Fitness,
    problems::{
        tsp::{Coordinates, Dimension, DistanceMeasure, Edge, Route},
        Optimum, Problem, VectorProblem,
    },
};
use anyhow::{anyhow, Error, Result};
use pest_consume::Parser;

// Converts the parsing-tree for symmetric TSP that was constructed by `pest`
// into rust-usable data types using the `pest_consume` package.
#[allow(clippy::upper_case_acronyms)]
mod parser {
    // Parser for .tsp files
    pub(super) mod tsp {
        use crate::problems::tsp::{distances, Coordinates, DistanceMeasure, SymmetricTsp};
        use pest_consume::{match_nodes, Error, Parser};

        type Result<T> = std::result::Result<T, Error<Rule>>;
        type Node<'i> = pest_consume::Node<'i, Rule, ()>;

        #[derive(Parser)]
        #[grammar = "problems/tsp/grammars/symmetric.tsp.pest"]
        pub struct TspParser;

        #[pest_consume::parser]
        impl TspParser {
            pub fn file(input: Node) -> Result<SymmetricTsp> {
                Ok(match_nodes!(input.into_children();
                    [tsp(tsp), _] => tsp,
                ))
            }

            fn tsp(input: Node) -> Result<SymmetricTsp> {
                Ok(match_nodes!(input.clone().into_children();
                    [
                        name(name),
                        dimension(dimension),
                        edge_weight_type(distance_measure),
                        node_coord_section_coords(positions),
                    ] => {
                        if dimension != positions.len() {
                            return Err(input.error("dimension not equal to number of nodes"))
                        }
                        SymmetricTsp {
                            name,
                            best_solution: None,
                            dimension,
                            positions,
                            distance_measure,
                        }
                    },
                ))
            }

            fn name(input: Node) -> Result<String> {
                Ok(input.as_str().to_string())
            }

            fn dimension(input: Node) -> Result<usize> {
                input.as_str().parse().map_err(|e| input.error(e))
            }

            fn edge_weight_type(input: Node) -> Result<DistanceMeasure> {
                Ok(match input.as_str() {
                    "EUC_2D" => distances::euclidean_distance,
                    "MAN_2D" => distances::manhattan_distance,
                    "MAX_2D" => distances::maximum_distance,
                    _ => unreachable!(),
                })
            }

            fn index(input: Node) -> Result<usize> {
                input.as_str().parse().map_err(|e| input.error(e))
            }

            fn coord(input: Node) -> Result<f64> {
                input.as_str().parse().map_err(|e| input.error(e))
            }

            fn coords(input: Node) -> Result<Coordinates> {
                Ok(match_nodes!(input.into_children();
                    [index(_i), coord(x), coord(y)] => vec![x, y],
                ))
            }

            fn node_coord_section_coords(input: Node) -> Result<Vec<Coordinates>> {
                Ok(match_nodes!(input.into_children();
                    [coords(c)..] => c.collect(),
                ))
            }

            #[allow(non_snake_case, unused_variables)]
            fn EOI(input: Node) -> Result<()> {
                Ok(())
            }
        }
    }

    // Parser for .opt.tour files
    pub(super) mod opt {
        use crate::{
            fitness::Fitness,
            problems::tsp::{symmetric::Optimum, Route},
        };
        use pest_consume::{match_nodes, Error, Parser};

        type Result<T> = std::result::Result<T, Error<Rule>>;
        type Node<'i> = pest_consume::Node<'i, Rule, ()>;

        #[derive(Parser)]
        #[grammar = "problems/tsp/grammars/symmetric.opt.tour.pest"]
        pub struct TspOptParser;

        #[pest_consume::parser]
        impl TspOptParser {
            pub fn file(input: Node) -> Result<Optimum<Route>> {
                Ok(match_nodes!(input.into_children();
                    [opt(opt), _] => opt,
                ))
            }

            fn opt(input: Node) -> Result<Optimum<Route>> {
                Ok(match_nodes!(input.clone().into_children();
                    // Only fitness value
                    [
                        name(_name),
                        dimension(_dimension),
                        best_solution(fitness),
                    ] => {
                        Optimum {
                            fitness,
                            solution: None,
                        }
                    },
                    // Tour nodes are present
                    [
                        name(_name),
                        dimension(dimension),
                        best_solution(fitness),
                        tour_section_nodes(nodes),
                    ] => {
                        if dimension != nodes.len() {
                            return Err(input.error("dimension not equal to number of nodes"))
                        }
                        Optimum {
                            fitness,
                            solution: Some(nodes),
                        }
                    },
                ))
            }

            fn name(input: Node) -> Result<String> {
                Ok(input.as_str().to_string())
            }

            fn dimension(input: Node) -> Result<usize> {
                input.as_str().parse().map_err(|e| input.error(e))
            }

            fn best_solution(input: Node) -> Result<Fitness> {
                input
                    .as_str()
                    .parse()
                    .map_err(|e| input.error(e))
                    .map(|f: f64| Fitness::try_from(f).unwrap())
            }

            fn index(input: Node) -> Result<usize> {
                let i: usize = input.as_str().parse().map_err(|e| input.error(e))?;
                if i == 0 {
                    Err(input.error("node index can't be zero"))
                } else {
                    Ok(i - 1)
                }
            }

            fn tour_section_nodes(input: Node) -> Result<Route> {
                Ok(match_nodes!(input.into_children();
                    [index(i)..] => i.collect::<Vec<_>>(),
                ))
            }

            #[allow(non_snake_case, unused_variables)]
            fn EOI(input: Node) -> Result<()> {
                Ok(())
            }
        }
    }
}

/// This enum represents built in instances of the symmetric travelling salesman problem.
#[rustfmt::skip]
#[derive(Debug)]
pub enum Instances {
    A280,
    BERLIN52,
    BIER127,
    CH130, CH150,
    D198, D493, D657, D1291, D1655, D2103, D15112, D18512,
    EIL51, EIL76, EIL101,
    FL417, FL1400, FL1577, FL3795,
    NRW1379,
    USA13509,
}

impl std::fmt::Display for Instances {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl TryFrom<&str> for Instances {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "A280" => Ok(Instances::A280),
            "BERLIN52" => Ok(Instances::BERLIN52),
            "BIER127" => Ok(Instances::BIER127),
            "CH130" => Ok(Instances::CH130),
            "CH150" => Ok(Instances::CH150),
            "D198" => Ok(Instances::D198),
            "D493" => Ok(Instances::D493),
            "D657" => Ok(Instances::D657),
            "D1291" => Ok(Instances::D1291),
            "D1655" => Ok(Instances::D1655),
            "D2103" => Ok(Instances::D2103),
            "D15112" => Ok(Instances::D15112),
            "D18512" => Ok(Instances::D18512),
            "EIL51" => Ok(Instances::EIL51),
            "EIL76" => Ok(Instances::EIL76),
            "EIL101" => Ok(Instances::EIL101),
            "FL417" => Ok(Instances::FL417),
            "FL1400" => Ok(Instances::FL1400),
            "FL1577" => Ok(Instances::FL1577),
            "FL3795" => Ok(Instances::FL3795),
            "NRW1379" => Ok(Instances::NRW1379),
            "USA13509" => Ok(Instances::USA13509),
            _ => Err(anyhow!("Unkonwn instance {}", value)),
        }
    }
}

impl Instances {
    /// Tries to load the built-in instance.
    pub fn try_load(&self) -> Result<SymmetricTsp> {
        #[rustfmt::skip]
        let (data, opt) = match self {
            Self::A280 => (include_str!("./tsplib/a280.tsp"), Some(include_str!("./tsplib/a280.opt.tour"))),
            Self::BERLIN52 => (include_str!("./tsplib/berlin52.tsp"), Some(include_str!("./tsplib/berlin52.opt.tour"))),
            Self::BIER127 => (include_str!("./tsplib/bier127.tsp"), Some(include_str!("./tsplib/bier127.opt.tour"))),
            Self::CH130 => (include_str!("./tsplib/ch130.tsp"), Some(include_str!("./tsplib/ch130.opt.tour"))),
            Self::CH150 => (include_str!("./tsplib/ch150.tsp"), Some(include_str!("./tsplib/ch150.opt.tour"))),
            Self::D198 => (include_str!("./tsplib/d198.tsp"), Some(include_str!("./tsplib/d198.opt.tour"))),
            Self::D493 => (include_str!("./tsplib/d493.tsp"), Some(include_str!("./tsplib/d493.opt.tour"))),
            Self::D657 => (include_str!("./tsplib/d657.tsp"), Some(include_str!("./tsplib/d657.opt.tour"))),
            Self::D1291 => (include_str!("./tsplib/d1291.tsp"), Some(include_str!("./tsplib/d1291.opt.tour"))),
            Self::D1655 => (include_str!("./tsplib/d1655.tsp"), Some(include_str!("./tsplib/d1655.opt.tour"))),
            Self::D2103 => (include_str!("./tsplib/d2103.tsp"), Some(include_str!("./tsplib/d2103.opt.tour"))),
            Self::D15112 => (include_str!("./tsplib/d15112.tsp"), Some(include_str!("./tsplib/d15112.opt.tour"))),
            Self::D18512 => (include_str!("./tsplib/d18512.tsp"), Some(include_str!("./tsplib/d18512.opt.tour"))),
            Self::EIL51 => (include_str!("./tsplib/eil51.tsp"), Some(include_str!("./tsplib/eil51.opt.tour"))),
            Self::EIL76 => (include_str!("./tsplib/eil76.tsp"), Some(include_str!("./tsplib/eil76.opt.tour"))),
            Self::EIL101 => (include_str!("./tsplib/eil101.tsp"), Some(include_str!("./tsplib/eil101.opt.tour"))),
            Self::FL417 => (include_str!("./tsplib/fl417.tsp"), Some(include_str!("./tsplib/fl417.opt.tour"))),
            Self::FL1400 =>  (include_str!("./tsplib/fl1400.tsp"), Some(include_str!("./tsplib/fl1400.opt.tour"))),
            Self::FL1577 =>  (include_str!("./tsplib/fl1577.tsp"), Some(include_str!("./tsplib/fl1577.opt.tour"))),
            Self::FL3795 =>  (include_str!("./tsplib/fl3795.tsp"), Some(include_str!("./tsplib/fl3795.opt.tour"))),
            Self::NRW1379 =>  (include_str!("./tsplib/nrw1379.tsp"), Some(include_str!("./tsplib/nrw1379.opt.tour"))),
            Self::USA13509 =>  (include_str!("./tsplib/usa13509.tsp"), Some(include_str!("./tsplib/usa13509.opt.tour"))),
        };
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

/// Represents an instance of the symmetric travelling salesman problem.
#[derive(serde::Serialize)]
pub struct SymmetricTsp {
    /// Name of the instance
    pub name: String,
    /// Dimension of the instance
    pub dimension: Dimension,
    /// Best possible solution
    pub best_solution: Option<Optimum<Route>>,
    /// The cities coordinates
    #[serde(skip)]
    pub positions: Vec<Coordinates>,
    /// How distance should be computed
    #[serde(skip)]
    distance_measure: DistanceMeasure,
}

impl Problem for SymmetricTsp {
    type Encoding = Route;

    fn evaluate(&self, solution: &Self::Encoding) -> f64 {
        solution
            .iter()
            .copied()
            .zip(solution.iter().copied().skip(1))
            .map(|edge| self.distance(edge))
            .sum()
    }

    fn name(&self) -> &str {
        "SymmetricTsp"
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
        self.best_solution.as_ref().map(|o| o.fitness.into())
    }

    /// Returns the weight/distance of the given edge.
    pub fn distance(&self, edge: Edge) -> f64 {
        let (a, b) = edge;
        (self.distance_measure)(&self.positions[a], &self.positions[b]).into()
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
                .min_by_key(|(_, &r)| Fitness::try_from(self.distance((last, r))).unwrap())
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
        let input = parser::tsp::TspParser::parse(parser::tsp::Rule::file, data)
            .map_err(|e| Error::new(e).context("error while parsing .tsp file"))?
            .single()
            .unwrap();
        let mut tsp = parser::tsp::TspParser::file(input)
            .map_err(|e| Error::new(e).context("tsp data type conversions failed"))?;

        if let Some(opt) = opt {
            let opt_input = parser::opt::TspOptParser::parse(parser::opt::Rule::file, opt)
                .map_err(|e| Error::new(e).context("error while parsing .opt.tour file"))?
                .single()
                .unwrap();
            let opt = parser::opt::TspOptParser::file(opt_input)
                .map_err(|e| Error::new(e).context("tsp opt tour data type conversions failed"))?;
            if let Some(sol) = &opt.solution {
                if sol.len() != tsp.dimension {
                    return Err(anyhow!("dimension of opt does not match problem dimension",));
                }
            }
            tsp.best_solution = Some(opt);
        }

        Ok(tsp)
    }
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
        assert_float_eq!(best_solution.fitness.into(), 7542.0, ulps <= 2);
        assert_eq!(best_solution.solution.unwrap(), opt_tour);
    }

    #[test]
    fn evaluating_berlin52() {
        let tsp = Instances::BERLIN52.load();
        let best = tsp.best_solution.as_ref().unwrap();

        // There seems to be a difference between the best solutions supplied in BEST_SOLUTION
        // and the evaluated routes from the opt.tour files. Is this due to rounding errors?
        assert_float_eq!(
            tsp.evaluate(best.solution.as_ref().unwrap()),
            best.fitness.into(),
            abs <= 50.0
        );
    }
}
