use criterion::{black_box, criterion_group, criterion_main, Criterion};
use mahf::components::mutation::functional::{
    circular_swap, circular_swap2, translocate_slice, translocate_slice2,
};

pub fn circular_swap_benchmark(c: &mut Criterion) {
    let mut permutation = vec![0usize, 1, 2, 3, 4];
    c.bench_function("swap 1", |b| {
        b.iter(|| circular_swap(black_box(&mut permutation), black_box(&[0, 1, 2, 4])))
    });
    let mut permutation = vec![0usize, 1, 2, 3, 4];
    c.bench_function("swap 2", |b| {
        b.iter(|| circular_swap2(black_box(&mut permutation), black_box(&[0, 1, 2, 4])))
    });
}

pub fn translocate_slice_benchmark(c: &mut Criterion) {
    let mut permutation = vec![1usize, 2, 3, 4, 5, 6, 7, 8, 9];
    c.bench_function("translocate 1", |b| {
        b.iter(|| translocate_slice(black_box(&mut permutation), black_box(3..6), black_box(6)))
    });
    let mut permutation = vec![1usize, 2, 3, 4, 5, 6, 7, 8, 9];
    c.bench_function("translocate 2", |b| {
        b.iter(|| translocate_slice2(black_box(&mut permutation), black_box(3..6), black_box(6)))
    });
}

// criterion_group!(benches, circular_swap_benchmark);
criterion_group!(benches, translocate_slice_benchmark);
criterion_main!(benches);
