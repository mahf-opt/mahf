//! Collection of benchmark functions with corresponding tests.

pub mod implementations;
#[cfg(test)]
pub mod tests;

use crate::problems::{HasKnownOptimum, LimitedVectorProblem, Problem, VectorProblem};
use anyhow::anyhow;
use std::convert::TryFrom;

/// Wraps the benchmark functions as [`Problem`]s.
///
/// All functions have been scaled to [-1, 1].
#[derive(Clone, Copy, serde::Serialize)]
pub struct BenchmarkFunction {
    name: &'static str,
    dimension: usize,
    #[serde(skip)]
    domain: [f64; 2],

    #[serde(skip)]
    known_optimum: f64,

    #[serde(skip)]
    implementation: Function,
}

impl Problem for BenchmarkFunction {
    type Encoding = Vec<f64>;

    fn evaluate(&self, solution: &Self::Encoding) -> f64 {
        (self.implementation)(solution)
    }

    fn name(&self) -> &str {
        self.name
    }
}

impl VectorProblem for BenchmarkFunction {
    type T = f64;

    fn dimension(&self) -> usize {
        self.dimension
    }
}

impl LimitedVectorProblem for BenchmarkFunction {
    fn range(&self, _dimension: usize) -> std::ops::Range<Self::T> {
        0.0..1.0
    }
}

impl HasKnownOptimum for BenchmarkFunction {
    fn known_optimum(&self) -> f64 {
        self.known_optimum
    }
}

/// A benchmark function.
pub type Function = fn(&[f64]) -> f64;

impl TryFrom<&str> for BenchmarkFunction {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let err = || anyhow!("Invalid bmf format. Expected 'fn_name<dimension>'.");
        let mut parts = value.split(&['<', '>'][..]);
        let name = parts.next().ok_or_else(err)?;
        let dimension = parts.next().ok_or_else(err)?.parse::<usize>()?;

        match name {
            "sphere" => Ok(BenchmarkFunction::sphere(dimension)),
            "rastrigin" => Ok(BenchmarkFunction::rastrigin(dimension)),
            "ackley" => Ok(BenchmarkFunction::ackley(dimension)),
            "ackley_n4" => Ok(BenchmarkFunction::ackley_n4(dimension)),
            "alpine_n1" => Ok(BenchmarkFunction::alpine_n1(dimension)),
            "alpine_n2" => Ok(BenchmarkFunction::alpine_n2(dimension)),
            "brown" => Ok(BenchmarkFunction::brown(dimension)),
            "exponential" => Ok(BenchmarkFunction::exponential(dimension)),
            "griewank" => Ok(BenchmarkFunction::griewank(dimension)),
            "happy_cat" => Ok(BenchmarkFunction::happy_cat(dimension)),
            "periodic" => Ok(BenchmarkFunction::periodic(dimension)),
            "powell_sum" => Ok(BenchmarkFunction::powell_sum(dimension)),
            "qing" => Ok(BenchmarkFunction::qing(dimension)),
            "ridge" => Ok(BenchmarkFunction::ridge(dimension)),
            "rosenbrock" => Ok(BenchmarkFunction::rosenbrock(dimension)),
            "salomon" => Ok(BenchmarkFunction::salomon(dimension)),
            "schwefel_220" => Ok(BenchmarkFunction::schwefel_220(dimension)),
            "schwefel_221" => Ok(BenchmarkFunction::schwefel_221(dimension)),
            "schwefel_222" => Ok(BenchmarkFunction::schwefel_222(dimension)),
            "schwefel_223" => Ok(BenchmarkFunction::schwefel_223(dimension)),
            "schwefel" => Ok(BenchmarkFunction::schwefel(dimension)),
            "shubert_n3" => Ok(BenchmarkFunction::shubert_n3(dimension)),
            "shubert_n4" => Ok(BenchmarkFunction::shubert_n4(dimension)),
            "shubert" => Ok(BenchmarkFunction::shubert(dimension)),
            "styblinski_tang" => Ok(BenchmarkFunction::styblinski_tang(dimension)),
            "sum_squares" => Ok(BenchmarkFunction::sum_squares(dimension)),
            "yang_n2" => Ok(BenchmarkFunction::yang_n2(dimension)),
            "yang_n3" => Ok(BenchmarkFunction::yang_n3(dimension)),
            "yang_n4" => Ok(BenchmarkFunction::yang_n4(dimension)),
            "zakharov" => Ok(BenchmarkFunction::zakharov(dimension)),
            "ackley_n2" => Ok(BenchmarkFunction::ackley_n2(dimension)),
            "ackley_n3" => Ok(BenchmarkFunction::ackley_n3(dimension)),
            "adjiman" => Ok(BenchmarkFunction::adjiman(dimension)),
            "bartels_conn" => Ok(BenchmarkFunction::bartels_conn(dimension)),
            "beale" => Ok(BenchmarkFunction::beale(dimension)),
            "bird" => Ok(BenchmarkFunction::bird(dimension)),
            "bohachevsky_n1" => Ok(BenchmarkFunction::bohachevsky_n1(dimension)),
            "bohachevsky_n2" => Ok(BenchmarkFunction::bohachevsky_n2(dimension)),
            "booth" => Ok(BenchmarkFunction::booth(dimension)),
            "brent" => Ok(BenchmarkFunction::brent(dimension)),
            "bukin_n6" => Ok(BenchmarkFunction::bukin_n6(dimension)),
            "cross_in_tray" => Ok(BenchmarkFunction::cross_in_tray(dimension)),
            "deckkers_aarts" => Ok(BenchmarkFunction::deckkers_aarts(dimension)),
            "drop_wave" => Ok(BenchmarkFunction::drop_wave(dimension)),
            "easom" => Ok(BenchmarkFunction::easom(dimension)),
            "egg_crate" => Ok(BenchmarkFunction::egg_crate(dimension)),
            "goldstein_price" => Ok(BenchmarkFunction::goldstein_price(dimension)),
            "gramacy_lee" => Ok(BenchmarkFunction::gramacy_lee(dimension)),
            "himmelblau" => Ok(BenchmarkFunction::himmelblau(dimension)),
            "holder_table" => Ok(BenchmarkFunction::holder_table(dimension)),
            "keane" => Ok(BenchmarkFunction::keane(dimension)),
            "leon" => Ok(BenchmarkFunction::leon(dimension)),
            "levi_n13" => Ok(BenchmarkFunction::levi_n13(dimension)),
            "matyas" => Ok(BenchmarkFunction::matyas(dimension)),
            "mccormick" => Ok(BenchmarkFunction::mccormick(dimension)),
            "schaffer_n1" => Ok(BenchmarkFunction::schaffer_n1(dimension)),
            "schaffer_n2" => Ok(BenchmarkFunction::schaffer_n2(dimension)),
            "schaffer_n3" => Ok(BenchmarkFunction::schaffer_n3(dimension)),
            "schaffer_n4" => Ok(BenchmarkFunction::schaffer_n4(dimension)),
            "three_hump_camel" => Ok(BenchmarkFunction::three_hump_camel(dimension)),
            "wolfe" => Ok(BenchmarkFunction::wolfe(dimension)),
            _ => Err(anyhow!("Unknown benchmark funktion {}", name)),
        }
    }
}

#[cfg(test)]
mod try_from_tests {
    use super::*;

    #[test]
    fn try_from_sphere() {
        let bmf = BenchmarkFunction::try_from("sphere<20>").unwrap();
        assert_eq!(bmf.name, "sphere");
        assert_eq!(bmf.dimension, 20);
    }
}
