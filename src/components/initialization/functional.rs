use std::{iter::repeat_with, ops::Range};

use rand::{
    distributions::{uniform::SampleUniform, Bernoulli},
    seq::SliceRandom,
    Rng,
};

use crate::state::random::Random;

pub fn random_spread<D>(
    domain: &[Range<D>],
    population_size: usize,
    rng: &mut Random,
) -> Vec<Vec<D>>
where
    D: SampleUniform + Clone + PartialOrd + 'static,
{
    repeat_with(|| {
        domain
            .iter()
            .map(|range| rng.gen_range(range.clone()))
            .collect()
    })
    .take(population_size)
    .collect()
}

pub fn random_permutation(
    dimension: usize,
    population_size: usize,
    rng: &mut Random,
) -> Vec<Vec<usize>> {
    repeat_with(|| {
        let mut solution: Vec<_> = (0..dimension).collect();
        solution.shuffle(rng);
        solution
    })
    .take(population_size)
    .collect()
}

pub fn random_bitstring(
    dimension: usize,
    p: f64,
    population_size: usize,
    rng: &mut Random,
) -> Vec<Vec<bool>> {
    repeat_with(|| {
        rng.sample_iter(Bernoulli::new(p).unwrap())
            .take(dimension)
            .collect()
    })
    .take(population_size)
    .collect()
}
