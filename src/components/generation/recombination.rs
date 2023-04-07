//! Recombination Operators

use std::cmp::max;

use rand::{seq::IteratorRandom, Rng};
use serde::{Deserialize, Serialize};

use crate::{
    components::Component,
    framework::{AnyComponent, Individual},
    problems::{Problem, VectorProblem},
    state::State,
};

/// Specialized component trait to generate a new population from the current one.
///
/// This trait is especially useful for components that combine multiple solutions.
/// For modifying solutions independently, see [Generation].
///
/// # Implementing [Component]
///
/// Types implementing this trait can implement [Component] by wrapping the type in a [Recombinator].
pub trait Recombination<P: Problem> {
    fn recombine_solutions(
        &self,
        parents: Vec<P::Encoding>,
        offspring: &mut Vec<P::Encoding>,
        problem: &P,
        state: &mut State<P>,
    );
}

#[derive(serde::Serialize, Clone)]
pub struct Recombinator<T: Clone>(pub T);

impl<T, P, D> Component<P> for Recombinator<T>
where
    P: Problem<Encoding = Vec<D>>,
    T: AnyComponent + Recombination<P> + Serialize + Clone,
    D: Clone + PartialEq + 'static,
{
    fn execute(&self, problem: &P, state: &mut State<P>) {
        let population = state.populations_mut().pop();
        let population = population
            .into_iter()
            .map(Individual::into_solution)
            .collect();
        let mut offspring = Vec::new();
        self.0
            .recombine_solutions(population, &mut offspring, problem, state);
        let offspring = offspring
            .into_iter()
            .map(Individual::<P>::new_unevaluated)
            .collect();
        state.populations_mut().push(offspring);
    }
}

/// Applies a n-point crossover to two parent solutions depending on crossover probability.
///
/// If pc = 1, the solutions are recombined.
#[derive(Serialize, Deserialize, Clone)]
pub struct NPointCrossover {
    /// Probability of recombining the solutions.
    pub pc: f64,
    /// Number of points (N)
    pub points: usize,
    /// If false, the second child is discarded.
    pub keep_both: bool,
}
impl NPointCrossover {
    pub fn new<P, D>(pc: f64, points: usize, keep_both: bool) -> Box<dyn Component<P>>
    where
        P: Problem<Encoding = Vec<D>>,
        D: Clone + PartialEq + 'static,
    {
        Box::new(Recombinator(Self {
            pc,
            points,
            keep_both,
        }))
    }

    pub fn new_single<P, D>(pc: f64, points: usize) -> Box<dyn Component<P>>
    where
        P: Problem<Encoding = Vec<D>>,
        D: Clone + PartialEq + 'static,
    {
        Self::new(pc, points, false)
    }

    pub fn new_both<P, D>(pc: f64, points: usize) -> Box<dyn Component<P>>
    where
        P: Problem<Encoding = Vec<D>>,
        D: Clone + PartialEq + 'static,
    {
        Self::new(pc, points, true)
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
        state: &mut State<P>,
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

                if self.keep_both {
                    offspring.push(child2);
                }
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
    use crate::state::Random;

    use super::*;

    #[test]
    fn all_recombined() {
        let problem = BenchmarkFunction::sphere(3);
        let comp = NPointCrossover {
            pc: 1.0,
            points: 3,
            keep_both: true,
        };
        let mut state = State::new();
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
#[derive(Serialize, Deserialize, Clone)]
pub struct UniformCrossover {
    /// Probability of recombining the solutions.
    pub pc: f64,
    /// If false, the second child is discarded.
    pub keep_both: bool,
}
impl UniformCrossover {
    pub fn new<P, D>(pc: f64, keep_both: bool) -> Box<dyn Component<P>>
    where
        P: Problem<Encoding = Vec<D>>,
        D: Clone + PartialEq + 'static,
    {
        Box::new(Recombinator(Self { pc, keep_both }))
    }

    pub fn new_single<P, D>(pc: f64) -> Box<dyn Component<P>>
    where
        P: Problem<Encoding = Vec<D>>,
        D: Clone + PartialEq + 'static,
    {
        Self::new(pc, false)
    }

    pub fn new_both<P, D>(pc: f64) -> Box<dyn Component<P>>
    where
        P: Problem<Encoding = Vec<D>>,
        D: Clone + PartialEq + 'static,
    {
        Self::new(pc, true)
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
        state: &mut State<P>,
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
    use crate::state::Random;

    use super::*;

    #[test]
    fn all_recombined() {
        let problem = BenchmarkFunction::sphere(3);
        let comp = UniformCrossover {
            pc: 1.0,
            keep_both: true,
        };
        let mut state = State::new();
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
#[derive(Serialize, Deserialize, Clone)]
pub struct ArithmeticCrossover {
    /// Probability of recombining the solutions.
    pub pc: f64,
    /// If false, the second child is discarded.
    pub keep_both: bool,
}
impl ArithmeticCrossover {
    pub fn new<P>(pc: f64, keep_both: bool) -> Box<dyn Component<P>>
    where
        P: Problem<Encoding = Vec<f64>>,
    {
        Box::new(Recombinator(Self { pc, keep_both }))
    }

    pub fn new_single<P>(pc: f64) -> Box<dyn Component<P>>
    where
        P: Problem<Encoding = Vec<f64>>,
    {
        Self::new(pc, false)
    }

    pub fn new_both<P>(pc: f64) -> Box<dyn Component<P>>
    where
        P: Problem<Encoding = Vec<f64>>,
    {
        Self::new(pc, true)
    }
}
impl<P> Recombination<P> for ArithmeticCrossover
where
    P: Problem<Encoding = Vec<f64>>,
{
    fn recombine_solutions(
        &self,
        parents: Vec<Vec<f64>>,
        offspring: &mut Vec<Vec<f64>>,
        _problem: &P,
        state: &mut State<P>,
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
                        let alpha = rng.gen::<f64>();
                        let x_1 = pairs[0][i];
                        let x_2 = pairs[1][i];
                        child1.push(x_1 * alpha + x_2 * (1. - alpha));
                        child2.push(x_2 * alpha + x_1 * (1. - alpha));
                    } else if i >= pairs[0].len() {
                        child2.push(pairs[1][i]);
                    } else if i >= pairs[1].len() {
                        child1.push(pairs[0][i]);
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

/// Applies a cycle crossover to two parent solutions depending on crossover probability.
///
/// Usually exclusive to combinatorial problems.
///
/// If pc = 1, the solutions are recombined.
#[derive(Serialize, Deserialize, Clone)]
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
        state: &mut State<P>,
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
    use crate::state::Random;

    use super::*;

    #[test]
    fn all_recombined() {
        let problem = BenchmarkFunction::sphere(3);
        let comp = CycleCrossover { pc: 1.0 };
        let mut state = State::new();
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

/// Performs a binomial crossover, combining two individuals from two populations at the same index.
///
/// Requires at least two populations on the stack, where the top population is modified.
/// Note that this crossover only has an effect if the two populations differ from each other.
#[derive(Serialize, Deserialize, Clone)]
pub struct DEBinomialCrossover {
    /// Probability of recombining the solutions.
    pc: f64,
}
impl DEBinomialCrossover {
    pub fn new<P: Problem<Encoding = Vec<f64>> + VectorProblem>(pc: f64) -> Box<dyn Component<P>> {
        Box::new(Self { pc })
    }
}

impl<P> Component<P> for DEBinomialCrossover
where
    P: Problem<Encoding = Vec<f64>> + VectorProblem,
{
    fn execute(&self, problem: &P, state: &mut State<P>) {
        let mut mut_state = state.get_states_mut();
        let populations = mut_state.populations_mut();

        let mut mutations = populations.pop();
        let bases = populations.current();

        let rng = mut_state.random_mut();

        for (mutation, base) in mutations.iter_mut().zip(bases) {
            let mutation = mutation.solution_mut();
            let base = base.solution();

            let index = rng.gen_range(0..problem.dimension());

            for i in 0..problem.dimension() {
                if rng.gen::<f64>() <= self.pc || i == index {
                    mutation[i] = base[i];
                }
            }
        }

        populations.push(mutations);
    }
}
#[cfg(test)]
mod de_binomial_crossover {
    use crate::framework::Individual;
    use crate::problems::bmf::BenchmarkFunction;
    use crate::state::common::Populations;
    use crate::state::Random;

    use super::*;

    #[test]
    fn all_recombined() {
        let problem = BenchmarkFunction::sphere(3);
        let comp = DEBinomialCrossover { pc: 1.0 };
        let mut state = State::new();
        state.insert(Random::testing());

        let mut stack = Populations::<BenchmarkFunction>::new();
        stack.push(
            vec![
                vec![8.0, 4.0, 7.0, 3.0, 6.0, 2.0, 5.0, 1.0, 9.0, 0.0],
                vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0],
            ]
            .into_iter()
            .map(Individual::new_unevaluated)
            .collect(),
        );
        stack.push(
            vec![
                vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0],
                vec![8.0, 4.0, 7.0, 3.0, 6.0, 2.0, 5.0, 1.0, 9.0, 0.0],
            ]
            .into_iter()
            .map(Individual::new_unevaluated)
            .collect(),
        );

        state.insert(stack);

        comp.initialize(&problem, &mut state);
        comp.execute(&problem, &mut state);

        let stack = state.populations_mut();

        let offspring = stack.pop();
        let parents = stack.current();

        assert_eq!(offspring.len(), parents.len());
    }
}

/// Performs an exponential crossover, combining two individuals from two populations at the same index.
///
/// Requires at least two populations on the stack, where the top population is modified.
/// Note that this crossover only has an effect if the two populations differ from each other.
#[derive(Serialize, Deserialize, Clone)]
pub struct DEExponentialCrossover {
    /// Probability of recombining the solutions.
    pc: f64,
}
impl DEExponentialCrossover {
    pub fn new<P: Problem<Encoding = Vec<f64>> + VectorProblem>(pc: f64) -> Box<dyn Component<P>> {
        Box::new(Self { pc })
    }
}

impl<P> Component<P> for DEExponentialCrossover
where
    P: Problem<Encoding = Vec<f64>> + VectorProblem,
{
    fn execute(&self, problem: &P, state: &mut State<P>) {
        let mut mut_state = state.get_states_mut();
        let populations = mut_state.populations_mut();

        let mut mutations = populations.pop();
        let bases = populations.current();

        let rng = mut_state.random_mut();

        for (mutation, base) in mutations.iter_mut().zip(bases) {
            let mutation = mutation.solution_mut();
            let base = base.solution();

            let index = rng.gen_range(0..problem.dimension());
            let mut i = index;

            loop {
                mutation[i] = base[i];
                i = (i + 1) % problem.dimension();

                if rng.gen::<f64>() > self.pc || i == index {
                    break;
                }
            }
        }

        populations.push(mutations);
    }
}
#[cfg(test)]
mod de_exponential_crossover {
    use crate::framework::Individual;
    use crate::problems::bmf::BenchmarkFunction;
    use crate::state::common::Populations;
    use crate::state::Random;

    use super::*;

    #[test]
    fn all_recombined() {
        let problem = BenchmarkFunction::sphere(3);
        let comp = DEExponentialCrossover { pc: 1.0 };
        let mut state = State::new();
        state.insert(Random::testing());

        let mut stack = Populations::<BenchmarkFunction>::new();
        stack.push(
            vec![
                vec![8.0, 4.0, 7.0, 3.0, 6.0, 2.0, 5.0, 1.0, 9.0, 0.0],
                vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0],
            ]
            .into_iter()
            .map(Individual::new_unevaluated)
            .collect(),
        );
        stack.push(
            vec![
                vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0],
                vec![8.0, 4.0, 7.0, 3.0, 6.0, 2.0, 5.0, 1.0, 9.0, 0.0],
            ]
            .into_iter()
            .map(Individual::new_unevaluated)
            .collect(),
        );

        state.insert(stack);

        comp.initialize(&problem, &mut state);
        comp.execute(&problem, &mut state);

        let stack = state.populations_mut();

        let offspring = stack.pop();
        let parents = stack.current();

        assert_eq!(offspring.len(), parents.len());
    }
}
