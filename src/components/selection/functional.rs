//! Functional population subset selection.
//!
//! The functions in this module can be used to simplify implementation of selection component behaviour.

use itertools::Itertools;
use rand::distributions::{Distribution, WeightedError, WeightedIndex};

use crate::utils::all_eq;
use crate::{problems::SingleObjectiveProblem, state::random::Random, Individual, Problem};

/// Returns the `(max, min)` objective values of the population if it is non-empty, or `None` otherwise.
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

/// Calculates proportional weights for all [`Individual`]s using the objective value.
///
/// Greater weights are assigned to [`Individual`]s with lower objective value.
///
/// The returned weights are guaranteed to be `>= offset`.
///
/// The `offset` is required to be positive or zero.
///
/// If `normalize` is true, the weights sum to `1`.
/// Note that most sampling methods don't require normalized weights.
#[contracts::debug_requires(offset >= 0.0, "the `offset` is required to be positive or zero")]
pub fn proportional_weights<P: SingleObjectiveProblem>(
    population: &[Individual<P>],
    offset: f64,
    normalize: bool,
) -> Option<Vec<f64>> {
    let (max, min) = objective_bounds(population)?;

    // Weights don't work with infinite fitness values.
    if !max.is_finite() {
        return None;
    }

    let weights: Vec<_> = population.iter().map(|i| i.objective().value()).collect();

    // Positive objective values can be directly used as weights after reversing.
    if min > 0.0 {
        // Add an offset to allow the worst to have a weight of `offset`, and not zero.
        return Some(weights.iter().map(|o| max - o + offset).collect());
    }

    // Explicitly handle uniform weights here to avoid having a sum of 0 later.
    // When all weights are identical, subtracting the `min` otherwise produces
    // weights all zero, which is not supported by most sampling methods, and dividing
    // by the `total` for normalizing divides by zero.
    if all_eq(&weights) {
        return Some(
            std::iter::repeat(if normalize {
                1.0 / population.len() as f64
            } else {
                1.0
            })
            .take(population.len())
            .collect(),
        );
    }

    // Shift all values to be `>= 0` using `o - min`, where `min` is guaranteed to be `<= 0`.
    // Subtract from the shifted best `(max - min)` to reverse weights for minimization.
    // Add an offset to allow the worst to have a weight of `offset`, and not zero.
    let shifted_weights: Vec<_> = weights
        .iter()
        .map(|o| (max - min) - (o - min) + offset)
        .collect();

    if !normalize {
        return Some(shifted_weights);
    }

    // Normalize weights by dividing with the total.
    // The total is guaranteed to be `> 0`, because all weights are shifted to be positive
    // and weights all identical are already handled.
    let total: f64 = shifted_weights.iter().sum();
    let normalized_weights: Vec<f64> = shifted_weights.iter().map(|f| f / total).collect();

    Some(normalized_weights)
}

/// Returns the ranking of the population, giving the individual
/// with lowest objective value a rank of `1`.
///
/// Ties are assigned the same rank.
pub fn reverse_rank<P: SingleObjectiveProblem>(population: &[Individual<P>]) -> Vec<usize> {
    population
        .iter()
        .map(|i| *i.objective())
        // Example: -5, 10, -1, 0, -1
        .enumerate()
        // Example: (0, -5), (1, 10), (2, -1), (3, 0), (4, -1)
        .sorted_by_key(|(_, o)| *o)
        // Example: (0, -5), (2, -1), (4, -1), (3, 0), (1, 10)
        .group_by(|(_, o)| *o)
        // Example: (0, -5), ((2, -1), (4, -1)), (3, 0), (1, 10)
        .into_iter()
        .enumerate()
        // Example: (0, (0, -5)), (1, (2, -1), (4, -1)), (2, (3, 0)), (3, (1, 10))
        .flat_map(|(group_index, (_, group))| {
            group
                .map(|(index, _)| (index, group_index + 1))
                .collect_vec()
        })
        // Example: (0, 1), (2, 2), (4, 2), (3, 3), (1, 4)
        .sorted_by_key(|(index, _)| *index)
        // Example: (0, 1), (1, 4), (2, 2), (3, 3), (4, 2)
        .map(|(_, rank)| rank)
        // Example: 1, 4, 2, 3, 2
        .collect()
}

/// Returns `n` individuals sampled with replacement using the provided `weights`.
///
/// Note that the weights are not required to sum to `1`.
/// See [`WeightedIndex`] for more information.
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
        + for<'b> std::ops::AddAssign<&'b X>
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

    fn all_positive(slice: &[f64]) -> bool {
        slice.iter().all(|&v| v > 0.0)
    }

    #[test_case(&[3., 1., 2.]; "when fitness positive")]
    #[test_case(&[-3., -5., -10., -1000.]; "when fitness negative")]
    #[test_case(&[3., -5., 10., -123.]; "when fitness mixed")]
    fn proportional_weights_returns_positive_weights(objective_values: &[f64]) {
        assert!(all_positive(
            &proportional_weights(&single_test_population(objective_values), 0.1, false).unwrap()
        ));
    }

    #[test]
    fn proportional_weights_returns_none_for_invalid_objective_values() {
        assert_eq!(
            proportional_weights(&single_test_population(&[0., f64::INFINITY]), 0.1, false),
            None
        );
    }

    #[test_case(&[1., 2., 3.] => vec![1, 2, 3]; "when fitness positive")]
    #[test_case(&[10., 18., 0., 3., 2., 7.] => vec![5, 6, 1, 3, 2, 4]; "when fitness positive 2")]
    #[test_case(&[-3., -5., -10., -2.] => vec![3, 2, 1, 4]; "when fitness negative")]
    #[test_case(&[3., -5., -10., 11.] => vec![3, 2, 1, 4]; "when fitness mixed")]
    #[test_case(&[0., 0., -10., 11.] => vec![2, 2, 1, 3]; "when fitness tied")]
    #[test_case(&[-5., 10., -1., 0., -1.] => vec![1, 4, 2, 3, 2]; "when fitness tied 2")]
    fn reverse_rank_returns_correct_reversed_ranking(objective_values: &[f64]) -> Vec<usize> {
        reverse_rank(&single_test_population(objective_values))
    }
}
