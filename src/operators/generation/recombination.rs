//! Recombination Operators

use std::cmp::max;

use rand::{seq::IteratorRandom, Rng};
use serde::{Deserialize, Serialize};

use crate::{
    framework::{
        components::*,
        specializations::{Recombination, Recombinator},
        State,
    },
    problems::Problem,
};

/// Applies a n-point crossover to two parent solutions depending on crossover probability.
///
/// If pc = 1, the solutions are recombined.
#[derive(Serialize, Deserialize)]
pub struct NPointCrossover {
    /// Probability of recombining the solutions.
    pub pc: f64,
    pub points: usize,
}
impl NPointCrossover {
    pub fn new<P, D>(pc: f64, points: usize) -> Box<dyn Component<P>>
    where
        P: Problem<Encoding = Vec<D>>,
        D: Clone + PartialEq + 'static,
    {
        Box::new(Recombinator(Self { pc, points }))
    }
}
impl<P, D> Recombination<P> for NPointCrossover
where
    P: Problem<Encoding = Vec<D>>,
    D: Clone,
{
    fn recombine_solutions(
        &self,
        parents: Vec<Vec<D>>,
        offspring: &mut Vec<Vec<D>>,
        _problem: &P,
        state: &mut State,
    ) {
        let dim: usize = parents
            .iter()
            .min_by(|x, &y| (x.len()).cmp(&y.len()))
            .unwrap()
            .len();
        assert!(self.points < dim);
        let rng = state.random_mut();
        for pairs in parents.chunks(2) {
            if pairs.len() > 1 {
                let mut child1 = pairs[0].to_owned();
                let mut child2 = pairs[1].to_owned();
                if rng.gen::<f64>() <= self.pc {
                    let mut pos = (0..dim).choose_multiple(rng, self.points);
                    pos.sort_unstable();
                    for (i, &pt) in pos.iter().enumerate() {
                        if pairs[0].len() != pairs[1].len() {
                            if i < self.points - 1 {
                                child2[..pt].swap_with_slice(&mut child1[..pt]);
                            } else {
                                child1.truncate(pt);
                                child1.extend_from_slice(&pairs[1][pt..]);
                                child2.truncate(pt);
                                child2.extend_from_slice(&pairs[0][pt..]);
                            }
                        } else {
                            child2[pt..].swap_with_slice(&mut child1[pt..]);
                        }
                    }
                }
                offspring.push(child1);
                offspring.push(child2);
            } else {
                let child1 = pairs[0].to_owned();
                offspring.push(child1);
            }
        }
    }
}

#[cfg(test)]
mod npoint_crossover {
    use crate::problems::bmf::BenchmarkFunction;
    use crate::random::Random;

    use super::*;

    #[test]
    fn all_recombined() {
        let problem = BenchmarkFunction::sphere(3);
        let comp = NPointCrossover { pc: 1.0, points: 3 };
        let mut state = State::new_root();
        state.insert(Random::testing());
        let population = vec![
            vec![0.1, 0.2, 0.4, 0.5, 0.9],
            vec![0.2, 0.3, 0.6, 0.7, 0.8],
            vec![0.11, 0.21, 0.41, 0.51, 0.91],
        ];
        let parents_length = population.len();
        let mut offspring = Vec::new();
        comp.recombine_solutions(population, &mut offspring, &problem, &mut state);
        assert_eq!(offspring.len(), parents_length);
    }
}

/// Applies a uniform crossover to two parent solutions depending on crossover probability.
///
/// If pc = 1, the solutions are recombined.
#[derive(Serialize, Deserialize)]
pub struct UniformCrossover {
    /// Probability of recombining the solutions.
    pub pc: f64,
}
impl UniformCrossover {
    pub fn new<P, D>(pc: f64) -> Box<dyn Component<P>>
    where
        P: Problem<Encoding = Vec<D>>,
        D: Clone + PartialEq + 'static,
    {
        Box::new(Recombinator(Self { pc }))
    }
}
impl<P, D> Recombination<P> for UniformCrossover
where
    P: Problem<Encoding = Vec<D>>,
    D: Clone,
{
    fn recombine_solutions(
        &self,
        parents: Vec<Vec<D>>,
        offspring: &mut Vec<Vec<D>>,
        _problem: &P,
        state: &mut State,
    ) {
        for pairs in parents.chunks(2) {
            if pairs.len() == 1 {
                let child1 = pairs[0].to_owned();
                offspring.push(child1);
                continue;
            }
            let mut child1 = Vec::new();
            let mut child2 = Vec::new();
            let rng = state.random_mut();
            if rng.gen::<f64>() <= self.pc {
                for i in 0..max(pairs[0].len(), pairs[1].len()) {
                    if i < pairs[0].len() && i < pairs[1].len() {
                        let (a, b) = if rng.gen_bool(0.5) { (0, 1) } else { (1, 0) };
                        child1.push(pairs[a][i].clone());
                        child2.push(pairs[b][i].clone());
                    } else if i >= pairs[0].len() {
                        child2.push(pairs[1][i].clone());
                    } else if i >= pairs[1].len() {
                        child1.push(pairs[0][i].clone());
                    }
                }
            } else {
                child1 = pairs[0].to_owned();
                child2 = pairs[1].to_owned();
            }
            offspring.push(child1);
            offspring.push(child2);
        }
    }
}

#[cfg(test)]
mod uniform_crossover {
    use crate::problems::bmf::BenchmarkFunction;
    use crate::random::Random;

    use super::*;

    #[test]
    fn all_recombined() {
        let problem = BenchmarkFunction::sphere(3);
        let comp = UniformCrossover { pc: 1.0 };
        let mut state = State::new_root();
        state.insert(Random::testing());
        let population = vec![
            vec![0.1, 0.2, 0.4, 0.5, 0.9],
            vec![0.2, 0.3, 0.6, 0.7, 0.8],
            vec![0.11, 0.21, 0.41, 0.51, 0.91],
        ];
        let parents_length = population.len();
        let mut offspring = Vec::new();
        comp.recombine_solutions(population, &mut offspring, &problem, &mut state);
        assert_eq!(offspring.len(), parents_length);
    }
}

/// Applies a cycle crossover to two parent solutions depending on crossover probability.
///
/// Usually exclusive to combinatorial problems.
///
/// If pc = 1, the solutions are recombined.
#[derive(Serialize, Deserialize)]
pub struct CycleCrossover {
    /// Probability of recombining the solutions.
    pub pc: f64,
}
impl CycleCrossover {
    pub fn new<P, D>(pc: f64) -> Box<dyn Component<P>>
    where
        P: Problem<Encoding = Vec<D>>,
        D: Clone + PartialEq + 'static,
    {
        Box::new(Recombinator(Self { pc }))
    }
}
impl<P, D: Clone> Recombination<P> for CycleCrossover
where
    P: Problem<Encoding = Vec<D>>,
    D: Clone + PartialEq,
{
    fn recombine_solutions(
        &self,
        parents: Vec<Vec<D>>,
        offspring: &mut Vec<Vec<D>>,
        _problem: &P,
        state: &mut State,
    ) {
        for pairs in parents.chunks(2) {
            if pairs.len() == 1 {
                let child1 = pairs[0].to_owned();
                offspring.push(child1);
                continue;
            }

            let mut child1 = Vec::new();
            let mut child2 = Vec::new();
            let rng = state.random_mut();
            if rng.gen::<f64>() <= self.pc {
                let mut cycles = vec![-1; pairs[0].len()];
                let mut cycle_number = 1;
                let cycle_start: Vec<usize> = (0..cycles.len()).collect();

                for mut pos in cycle_start {
                    while cycles[pos] < 0 {
                        cycles[pos] = cycle_number;
                        pos = pairs[0].iter().position(|r| r == &pairs[1][pos]).unwrap();
                    }

                    cycle_number += 1;
                }

                for (i, n) in cycles.iter().enumerate() {
                    if n % 2 == 0 {
                        child1.push(pairs[0][i].clone());
                        child2.push(pairs[1][i].clone());
                    } else {
                        child1.push(pairs[1][i].clone());
                        child2.push(pairs[0][i].clone());
                    }
                }
            } else {
                child1 = pairs[0].to_owned();
                child2 = pairs[1].to_owned();
            }
            offspring.push(child1);
            offspring.push(child2);
        }
    }
}

#[cfg(test)]
mod cycle_crossover {
    use crate::problems::bmf::BenchmarkFunction;
    use crate::random::Random;

    use super::*;

    #[test]
    fn all_recombined() {
        let problem = BenchmarkFunction::sphere(3);
        let comp = CycleCrossover { pc: 1.0 };
        let mut state = State::new_root();
        state.insert(Random::testing());
        let population = vec![
            vec![8.0, 4.0, 7.0, 3.0, 6.0, 2.0, 5.0, 1.0, 9.0, 0.0],
            vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0],
        ];
        let parents_length = population.len();
        let mut offspring = Vec::new();
        comp.recombine_solutions(population, &mut offspring, &problem, &mut state);
        assert_eq!(offspring.len(), parents_length);
    }
}
