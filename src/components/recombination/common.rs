use std::cmp::min;

use rand::{
    distributions::{Bernoulli, Uniform},
    seq::IteratorRandom,
    Rng,
};
use serde::{Deserialize, Serialize};

use crate::{
    component::ExecResult,
    components::{
        recombination::{functional as f, recombination, OptionalPair, Recombination},
        Component,
    },
    problems::VectorProblem,
    state::random::Random,
    State,
};

#[derive(Clone, Serialize, Deserialize)]
pub struct NPointCrossover {
    pub n: usize,
    pub pc: f64,
    pub insert_both: bool,
}

impl NPointCrossover {
    pub fn from_params(n: usize, pc: f64, insert_both: bool) -> Self {
        Self { n, pc, insert_both }
    }

    pub fn new<P, D>(n: usize, pc: f64, insert_both: bool) -> Box<dyn Component<P>>
    where
        P: VectorProblem<Element = D>,
        D: Clone,
    {
        Box::new(Self::from_params(n, pc, insert_both))
    }

    pub fn new_insert_single<P, D>(n: usize, pc: f64) -> Box<dyn Component<P>>
    where
        P: VectorProblem<Element = D>,
        D: Clone,
    {
        Self::new(n, pc, false)
    }

    pub fn new_insert_both<P, D>(n: usize, pc: f64) -> Box<dyn Component<P>>
    where
        P: VectorProblem<Element = D>,
        D: Clone,
    {
        Self::new(n, pc, true)
    }
}

impl<P, D> Recombination<P> for NPointCrossover
where
    P: VectorProblem<Element = D>,
    D: Clone,
{
    fn recombine(
        &self,
        parent1: &P::Encoding,
        parent2: &P::Encoding,
        rng: &mut Random,
    ) -> OptionalPair<P::Encoding> {
        if rng.gen::<f64>() <= self.pc {
            let dim = min(parent1.len(), parent2.len());
            let indices = (0..dim).choose_multiple(rng, self.n);
            let children = f::multi_point_crossover(parent1, parent2, &indices);
            OptionalPair::from_pair(children, self.insert_both)
        } else {
            OptionalPair::None
        }
    }
}

impl<P, D> Component<P> for NPointCrossover
where
    P: VectorProblem<Element = D>,
    D: Clone,
{
    fn execute(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        recombination(self, problem, state)
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct UniformCrossover {
    pub pc: f64,
    pub insert_both: bool,
}

impl UniformCrossover {
    pub fn from_params(pc: f64, insert_both: bool) -> Self {
        Self { pc, insert_both }
    }

    pub fn new<P, D>(pc: f64, insert_both: bool) -> Box<dyn Component<P>>
    where
        P: VectorProblem<Element = D>,
        D: Clone,
    {
        Box::new(Self::from_params(pc, insert_both))
    }

    pub fn new_insert_single<P, D>(pc: f64) -> Box<dyn Component<P>>
    where
        P: VectorProblem<Element = D>,
        D: Clone,
    {
        Self::new(pc, false)
    }

    pub fn new_insert_both<P, D>(pc: f64) -> Box<dyn Component<P>>
    where
        P: VectorProblem<Element = D>,
        D: Clone,
    {
        Self::new(pc, true)
    }
}

impl<P, D> Recombination<P> for UniformCrossover
where
    P: VectorProblem<Element = D>,
    D: Clone,
{
    fn recombine(
        &self,
        parent1: &P::Encoding,
        parent2: &P::Encoding,
        rng: &mut Random,
    ) -> OptionalPair<P::Encoding> {
        if rng.gen::<f64>() <= self.pc {
            let dim = min(parent1.len(), parent2.len());
            let mask: Vec<_> = rng
                .sample_iter(Bernoulli::new(0.5).unwrap())
                .take(dim)
                .collect();
            let children = f::uniform_crossover(parent1, parent2, &mask);
            OptionalPair::from_pair(children, self.insert_both)
        } else {
            OptionalPair::None
        }
    }
}

impl<P, D> Component<P> for UniformCrossover
where
    P: VectorProblem<Element = D>,
    D: Clone,
{
    fn execute(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        recombination(self, problem, state)
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ArithmeticCrossover {
    pub pc: f64,
    pub insert_both: bool,
}

impl ArithmeticCrossover {
    pub fn from_params(pc: f64, insert_both: bool) -> Self {
        Self { pc, insert_both }
    }

    pub fn new<P>(pc: f64, insert_both: bool) -> Box<dyn Component<P>>
    where
        P: VectorProblem<Element = f64>,
    {
        Box::new(Self::from_params(pc, insert_both))
    }

    pub fn new_insert_single<P>(pc: f64) -> Box<dyn Component<P>>
    where
        P: VectorProblem<Element = f64>,
    {
        Self::new(pc, false)
    }

    pub fn new_insert_both<P>(pc: f64) -> Box<dyn Component<P>>
    where
        P: VectorProblem<Element = f64>,
    {
        Self::new(pc, true)
    }
}

impl<P> Recombination<P> for ArithmeticCrossover
where
    P: VectorProblem<Element = f64>,
{
    fn recombine(
        &self,
        parent1: &P::Encoding,
        parent2: &P::Encoding,
        rng: &mut Random,
    ) -> OptionalPair<P::Encoding> {
        if rng.gen::<f64>() <= self.pc {
            let dim = min(parent1.len(), parent2.len());
            let alphas: Vec<_> = rng
                .sample_iter(Uniform::from(0.0..=1.0))
                .take(dim)
                .collect();
            let children = f::arithmetic_crossover(parent1, parent2, &alphas);
            OptionalPair::from_pair(children, self.insert_both)
        } else {
            OptionalPair::None
        }
    }
}

impl<P> Component<P> for ArithmeticCrossover
where
    P: VectorProblem<Element = f64>,
{
    fn execute(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        recombination(self, problem, state)
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CycleCrossover {
    pub pc: f64,
    pub insert_both: bool,
}

impl CycleCrossover {
    pub fn from_params(pc: f64, insert_both: bool) -> Self {
        Self { pc, insert_both }
    }

    pub fn new<P, D>(pc: f64, insert_both: bool) -> Box<dyn Component<P>>
    where
        P: VectorProblem<Element = D>,
        D: Clone + PartialEq + Ord + 'static,
    {
        Box::new(Self::from_params(pc, insert_both))
    }

    pub fn new_insert_single<P, D>(pc: f64) -> Box<dyn Component<P>>
    where
        P: VectorProblem<Element = D>,
        D: Clone + PartialEq + Ord + 'static,
    {
        Self::new(pc, false)
    }

    pub fn new_insert_both<P, D>(pc: f64) -> Box<dyn Component<P>>
    where
        P: VectorProblem<Element = D>,
        D: Clone + PartialEq + Ord + 'static,
    {
        Self::new(pc, true)
    }
}

impl<P, D> Recombination<P> for CycleCrossover
where
    P: VectorProblem<Element = D>,
    D: Clone + PartialEq + Ord + 'static,
{
    fn recombine(
        &self,
        parent1: &P::Encoding,
        parent2: &P::Encoding,
        rng: &mut Random,
    ) -> OptionalPair<P::Encoding> {
        if rng.gen::<f64>() <= self.pc {
            let children = f::cycle_crossover(parent1, parent2);
            OptionalPair::from_pair(children, self.insert_both)
        } else {
            OptionalPair::None
        }
    }
}

impl<P, D> Component<P> for CycleCrossover
where
    P: VectorProblem<Element = D>,
    D: Clone + PartialEq + Ord + 'static,
{
    fn execute(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        recombination(self, problem, state)
    }
}
