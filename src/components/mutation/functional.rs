//! Functional mutation of solutions.
//!
//! The functions in this module can be used to simplify implementation of mutation component behaviour.

use std::{cmp::Ordering, ops::Range};

use itertools::Itertools;

/// Swaps all `indices` in the `permutation` circularly.
#[contracts::requires(indices.len() > 1, "swapping less than two indices is not possible")]
pub fn circular_swap<D: 'static>(permutation: &mut [D], indices: &[usize]) {
    for (&i, &j) in indices
        .iter()
        .rev()
        .circular_tuple_windows::<(_, _)>()
        .skip(1)
    {
        permutation.swap(i, j);
    }
}

/// Swaps all `indices` in the `permutation` circularly.
#[contracts::requires(indices.len() > 1, "swapping less than two indices is not possible")]
pub fn circular_swap2<D: 'static>(permutation: &mut [D], indices: &[usize]) {
    let mut buffer = indices.to_owned();
    let n = buffer.len();

    permutation.swap(buffer[n - 1], buffer[0]);
    if n > 2 {
        for _ in 0..n - 2 {
            permutation.swap(buffer[buffer.len() - 1], buffer[buffer.len() - 2]);
            buffer.remove(buffer.len() - 1);
        }
    }
}

/// Removes the slice specified by the `range` from the `permutation` and inserts it at `index`.
#[contracts::requires(index < permutation.len())]
#[contracts::requires(range.start < permutation.len())]
#[contracts::requires(range.end < permutation.len())]
pub fn translocate_slice<D: 'static>(permutation: &mut [D], range: Range<usize>, index: usize) {
    let chunk_size = range.end - range.start;
    assert!(
        index + chunk_size <= permutation.len(),
        "moving the slice {:?} to index {} results in out of bounds access",
        range,
        index
    );

    match index.cmp(&range.start) {
        Ordering::Less => {
            permutation[index..range.end].rotate_right(chunk_size);
        }
        Ordering::Greater => {
            permutation[range.start..index + chunk_size].rotate_left(chunk_size);
        }
        Ordering::Equal => {}
    }
}

/// Removes the slice specified by the `range` from the `permutation` and inserts it at `index`.
#[contracts::requires(index < permutation.len())]
#[contracts::requires(range.start < permutation.len())]
#[contracts::requires(range.end < permutation.len())]
pub fn translocate_slice2<D: Clone + 'static>(
    permutation: &mut [D],
    range: Range<usize>,
    index: usize,
) {
    assert!(
        index + range.end - range.start <= permutation.len(),
        "moving the slice {:?} to index {} results in out of bounds access",
        range,
        index
    );

    let mut copy = permutation.to_vec();
    let slice: Vec<_> = copy.drain(range).collect();
    copy.splice(index..index, slice);
    permutation.clone_from_slice(&copy);
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;

    #[test_case(&[0, 1, 2, 3, 4], &[0, 3] => vec![3, 1, 2, 0, 4]; "when two indices")]
    #[test_case(&[0, 1, 2, 3, 4], &[0, 3, 4] => vec![4, 1, 2, 0, 3]; "when three indices ordered")]
    #[test_case(&[0, 1, 2, 3, 4], &[3, 4, 0] => vec![4, 1, 2, 0, 3]; "when three indices unordered")]
    #[test_case(&[0, 1, 2, 3, 4], &[0, 1, 2, 4] => vec![4, 0, 1, 3, 2]; "when four indices ordered")]
    #[test_case(&[0, 1, 2, 3, 4], &[1, 0, 4, 2] => vec![1, 2, 4, 3, 0]; "when four indices unordered")]
    fn circular_swap_swaps_correct_indices(permutation: &[usize], indices: &[usize]) -> Vec<usize> {
        let mut permutation = permutation.to_owned();
        circular_swap(&mut permutation, indices);
        permutation
    }

    #[test_case(&[0, 1, 2, 3, 4], &[0, 3] => vec![3, 1, 2, 0, 4]; "when two indices")]
    #[test_case(&[0, 1, 2, 3, 4], &[0, 3, 4] => vec![4, 1, 2, 0, 3]; "when three indices ordered")]
    #[test_case(&[0, 1, 2, 3, 4], &[3, 4, 0] => vec![4, 1, 2, 0, 3]; "when three indices unordered")]
    #[test_case(&[0, 1, 2, 3, 4], &[0, 1, 2, 4] => vec![4, 0, 1, 3, 2]; "when four indices ordered")]
    #[test_case(&[0, 1, 2, 3, 4], &[1, 0, 4, 2] => vec![1, 2, 4, 3, 0]; "when four indices unordered")]
    fn circular_swap2_swaps_correct_indices(
        permutation: &[usize],
        indices: &[usize],
    ) -> Vec<usize> {
        let mut permutation = permutation.to_owned();
        circular_swap2(&mut permutation, indices);
        permutation
    }

    #[test_case(&[1, 2, 3, 4, 5, 6, 7, 8, 9], 3..6, 1 => vec![1, 4, 5, 6, 2, 3, 7, 8, 9]; "example one")]
    #[test_case(&[1, 4, 5, 6, 2, 3, 7, 8, 9], 3..6, 6 => vec![1, 4, 5, 7, 8, 9, 6, 2, 3]; "example two")]
    fn translocate_slice_inserts_slice_at_correct_index(
        permutation: &[usize],
        range: Range<usize>,
        index: usize,
    ) -> Vec<usize> {
        let mut permutation = permutation.to_owned();
        translocate_slice(&mut permutation, range, index);
        permutation
    }

    #[test_case(&[1, 2, 3, 4, 5, 6, 7, 8, 9], 3..6, 1 => vec![1, 4, 5, 6, 2, 3, 7, 8, 9]; "example one")]
    #[test_case(&[1, 4, 5, 6, 2, 3, 7, 8, 9], 3..6, 6 => vec![1, 4, 5, 7, 8, 9, 6, 2, 3]; "example two")]
    fn translocate_slice2_inserts_slice_at_correct_index(
        permutation: &[usize],
        range: Range<usize>,
        index: usize,
    ) -> Vec<usize> {
        let mut permutation = permutation.to_owned();
        translocate_slice2(&mut permutation, range, index);
        permutation
    }
}
