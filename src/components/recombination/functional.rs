use itertools::multizip;

use crate::encoding::valid_permutation;

pub fn multi_point_crossover<D>(parent1: &[D], parent2: &[D], indices: &[usize]) -> [Vec<D>; 2]
where
    D: Clone,
{
    debug_assert!(!indices.is_empty());
    debug_assert!(indices.len() < parent1.len());
    debug_assert!(indices.len() < parent2.len());

    let mut child1 = parent1.to_owned();
    let mut child2 = parent2.to_owned();

    for (i, &idx) in indices.iter().enumerate() {
        if parent1.len() != parent2.len() {
            if i < indices.len() - 1 {
                child2[..idx].swap_with_slice(&mut child1[..idx]);
            } else {
                child1.truncate(idx);
                child1.extend_from_slice(&parent2[idx..]);
                child2.truncate(idx);
                child2.extend_from_slice(&parent1[idx..]);
            }
        } else {
            child2[idx..].swap_with_slice(&mut child1[idx..]);
        }
    }

    [child1, child2]
}

pub fn uniform_crossover<D>(parent1: &[D], parent2: &[D], mask: &[bool]) -> [Vec<D>; 2]
where
    D: Clone,
{
    debug_assert!(mask.len() >= parent1.len());
    debug_assert!(mask.len() >= parent2.len());

    let mut child1 = parent1.to_owned();
    let mut child2 = parent2.to_owned();

    for (i, &value) in mask.iter().enumerate() {
        if value {
            std::mem::swap(&mut child1[i], &mut child2[i]);
        }
    }

    [child1, child2]
}

pub fn arithmetic_crossover(parent1: &[f64], parent2: &[f64], alphas: &[f64]) -> [Vec<f64>; 2] {
    debug_assert!(alphas.len() >= parent1.len());
    debug_assert!(alphas.len() >= parent2.len());

    let mut child1 = parent1.to_owned();
    let mut child2 = parent2.to_owned();

    for (i, (&p1, &p2, &alpha)) in multizip((parent1, parent2, alphas)).enumerate() {
        child1[i] = alpha * p1 + (1. - alpha) * p2;
        child2[i] = alpha * p2 + (1. - alpha) * p1;
    }

    [child1, child2]
}

pub fn cycle_crossover<D: Clone + Ord + PartialEq>(parent1: &[D], parent2: &[D]) -> [Vec<D>; 2] {
    debug_assert_eq!(parent1.len(), parent2.len());
    debug_assert!(valid_permutation(parent1));
    debug_assert!(valid_permutation(parent2));

    let mut child1 = Vec::new();
    let mut child2 = Vec::new();

    let mut cycles = vec![-1; parent1.len()];
    let mut cycle_number = 1;
    let cycle_start: Vec<_> = (0..cycles.len()).collect();

    for mut pos in cycle_start {
        while cycles[pos] < 0 {
            cycles[pos] = cycle_number;
            pos = parent1.iter().position(|r| r == &parent2[pos]).unwrap();
        }
        cycle_number += 1;
    }

    for (p1, p2, n) in multizip((parent1, parent2, cycles)) {
        if n % 2 != 0 {
            child1.push(p1.clone());
            child2.push(p2.clone());
        } else {
            child1.push(p2.clone());
            child2.push(p1.clone());
        }
    }

    [child1, child2]
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;
    use crate::testing::*;

    fn assert_array_floats_eq(expected: [Vec<f64>; 2]) -> impl Fn([Vec<f64>; 2]) {
        move |actual| {
            assert_floats_eq(&expected[0], &actual[0]);
            assert_floats_eq(&expected[1], &actual[1]);
        }
    }

    #[test_case(&[0, 0, 0, 0, 0], &[1, 1, 1, 1, 1], &[2] => [vec![0, 0, 1, 1, 1], vec![1, 1, 0, 0, 0]]; "when single index")]
    #[test_case(&[0, 0, 0, 0, 0], &[1, 1, 1, 1, 1], &[2, 4] => [vec![0, 0, 1, 1, 0], vec![1, 1, 0, 0, 1]]; "when multiple indices ordered")]
    #[test_case(&[0, 0, 0, 0, 0], &[1, 1, 1, 1, 1], &[4, 2] => [vec![0, 0, 1, 1, 0], vec![1, 1, 0, 0, 1]]; "when multiple indices unordered")]
    #[test_case(&[0, 0, 0, 0, 0], &[1, 1, 1, 1, 1], &[1, 2, 3, 4] => [vec![0, 1, 0, 1, 0], vec![1, 0, 1, 0, 1]]; "when all possible indices ordered")]
    #[test_case(&[0, 0, 0, 0, 0], &[1, 1, 1, 1, 1], &[2, 4, 1, 3] => [vec![0, 1, 0, 1, 0], vec![1, 0, 1, 0, 1]]; "when all possible indices unordered")]
    fn multi_point_crossover_returns_correct_children(
        parent1: &[usize],
        parent2: &[usize],
        indices: &[usize],
    ) -> [Vec<usize>; 2] {
        multi_point_crossover(parent1, parent2, indices)
    }

    #[test_case(&[0, 0, 0, 0, 0], &[1, 1, 1, 1, 1], &[false, false, false, false, false] => [vec![0, 0, 0, 0, 0], vec![1, 1, 1, 1, 1]]; "when all false")]
    #[test_case(&[0, 0, 0, 0, 0], &[1, 1, 1, 1, 1], &[true, true, true, true, true] => [vec![1, 1, 1, 1, 1], vec![0, 0, 0, 0, 0]]; "when all true")]
    #[test_case(&[0, 0, 0, 0, 0], &[1, 1, 1, 1, 1], &[true, true, false, false, false] => [vec![1, 1, 0, 0, 0], vec![0, 0, 1, 1, 1]]; "when some true")]
    #[test_case(&[0, 0, 0, 0, 0], &[1, 1, 1, 1, 1], &[true, false, true, false, true] => [vec![1, 0, 1, 0, 1], vec![0, 1, 0, 1, 0]]; "when alternating mask")]
    fn uniform_crossover_returns_correct_children(
        parent1: &[usize],
        parent2: &[usize],
        mask: &[bool],
    ) -> [Vec<usize>; 2] {
        uniform_crossover(parent1, parent2, mask)
    }

    #[test_case(&[0., 0., 0., 0., 0.], &[1., 1., 1., 1., 1.], &[0.5, 0.5, 0.5, 0.5, 0.5] => using assert_array_floats_eq([vec![0.5, 0.5, 0.5, 0.5, 0.5], vec![0.5, 0.5, 0.5, 0.5, 0.5]]); "when alpha uniform")]
    #[test_case(&[0., 0., 0., 0., 0.], &[1., 1., 1., 1., 1.], &[0.3, 0.3, 0.3, 0.3, 0.3] => using assert_array_floats_eq([vec![0.7, 0.7, 0.7, 0.7, 0.7], vec![0.3, 0.3, 0.3, 0.3, 0.3]]); "when alpha not uniform")]
    #[test_case(&[0., 0., 0., 0., 0.], &[1., 1., 1., 1., 1.], &[0.5, 0.3, 0.7, 0.1, 0.9] => using assert_array_floats_eq([vec![0.5, 0.7, 0.3, 0.9, 0.1], vec![0.5, 0.3, 0.7, 0.1, 0.9]]); "when alpha different")]
    fn arithmetic_crossover_returns_correct_children(
        parent1: &[f64],
        parent2: &[f64],
        alphas: &[f64],
    ) -> [Vec<f64>; 2] {
        arithmetic_crossover(parent1, parent2, alphas)
    }

    #[test_case(&[8, 4, 7, 3, 6, 2, 5, 1, 9, 0], &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9] => [vec![8, 1, 2, 3, 4, 5, 6, 7, 9, 0], vec![0, 4, 7, 3, 6, 2, 5, 1, 8, 9]]; "when one is ascending")]
    fn cycle_crossover_returns_correct_children(
        parent1: &[usize],
        parent2: &[usize],
    ) -> [Vec<usize>; 2] {
        cycle_crossover(parent1, parent2)
    }
}
