//! Functional population subset selection.

use rand::distributions::{Distribution, WeightedError, WeightedIndex};

use crate::{problems::SingleObjectiveProblem, state::random::Random, Individual, Problem};

pub fn objective_bounds<P: SingleObjectiveProblem>(
    population: &[Individual<P>],
) -> Option<(f64, f64)> {
    let mut max_fitness = population.first().map(|i| i.objective().value())?;
    let mut min_fitness = max_fitness;

    for individual in population.iter().skip(1) {
        let fitness = individual.objective().value();
        if fitness > max_fitness {
            max_fitness = fitness;
        } else if fitness < min_fitness {
            min_fitness = fitness;
        }
    }

    Some((max_fitness, min_fitness))
}

pub fn proportional_weights<P: SingleObjectiveProblem>(
    population: &[Individual<P>],
    offset: f64,
) -> Option<Vec<f64>> {
    let (worst, best) = objective_bounds(population)?;
    if !worst.is_finite() || best.is_sign_negative() {
        return None;
    }
    let reversed_objective_values: Vec<_> = population
        .iter()
        .map(|i| worst - i.objective().value() + offset)
        .collect();
    let total: f64 = reversed_objective_values.iter().sum();
    let weights: Vec<f64> = reversed_objective_values
        .iter()
        .map(|f| f / total)
        .collect();
    Some(weights)
}

pub fn reverse_rank<P: SingleObjectiveProblem>(population: &[Individual<P>]) -> Vec<usize> {
    let n = population.len();
    // First descending argsort with fitness
    let mut positions: Vec<_> = (1..=n).collect();
    positions.sort_unstable_by_key(|&i| -*population[i - 1].objective());

    // Second argsort with positions obtains ranking
    let mut ranking: Vec<_> = (1..=n).collect();
    ranking.sort_unstable_by_key(|&i| positions[i - 1]);

    ranking
}

pub fn sample_population_weighted<'a, P, X, I>(
    population: &'a [Individual<P>],
    weights: I,
    n: u32,
    rng: &mut Random,
) -> Result<Vec<&'a Individual<P>>, WeightedError>
where
    P: Problem,
    I: IntoIterator,
    I::Item: rand::distributions::uniform::SampleBorrow<X>,
    X: rand::distributions::uniform::SampleUniform
        + PartialOrd
        + for<'b> ::core::ops::AddAssign<&'b X>
        + Clone
        + Default,
{
    let wheel = WeightedIndex::new(weights)?;
    let selection = wheel
        .sample_iter(rng)
        .take(n as usize)
        .map(|i| &population[i])
        .collect();
    Ok(selection)
}

pub(crate) fn best<P: SingleObjectiveProblem>(
    population: &[Individual<P>],
) -> Option<&Individual<P>> {
    population.iter().min_by_key(|i| i.objective())
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;
    use crate::testing::*;

    #[test_case(&[-5., 0., 5.] => Some((5., -5.)); "when fitness negative")]
    #[test_case(&[0., 1., 2., 3., 4., 5.] => Some((5., 0.)); "when fitness positive")]
    #[test_case(&[5., 0., 2., 100., 4., 5.] => Some((100., 0.)); "when fitness unordered")]
    #[test_case(&[0.] => Some((0., 0.)); "when single individual in population")]
    fn objective_bounds_returns_some_with_correct_bounds(
        objective_values: &[f64],
    ) -> Option<(f64, f64)> {
        objective_bounds(&single_test_population(objective_values))
    }

    #[test]
    fn objective_bounds_returns_none_for_empty() {
        assert_eq!(objective_bounds(&single_test_population(&[])), None);
    }

    #[test_case(&[0., f64::INFINITY] => None; "when fitness infinite")]
    #[test_case(&[0., -1.] => None; "when fitness negative")]
    fn proportional_weights_returns_none_for_invalid_objective_values(
        objective_values: &[f64],
    ) -> Option<Vec<f64>> {
        proportional_weights(&single_test_population(objective_values), 0.1)
    }

    #[test_case(&[1., 2., 3.] => vec![3, 2, 1]; "when fitness ordered")]
    #[test_case(&[3., -5., 10., 0.] => vec![2, 4, 1, 3]; "when fitness unordered")]
    fn reverse_rank_returns_correct_reversed_ranking(objective_values: &[f64]) -> Vec<usize> {
        reverse_rank(&single_test_population(objective_values))
    }
}
