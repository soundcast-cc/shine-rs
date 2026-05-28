use criterion::{black_box, criterion_group, criterion_main, Criterion};

use rand::{Rng, SeedableRng};

use shine_rs::quantization::{labs, mulr, mulsr};
use shine_rs::bitstream::abs_and_sign;

fn bench_mulr(c: &mut Criterion) {
    let mut rng = rand::rngs::StdRng::seed_from_u64(1);
    let pairs: Vec<(i32, i32)> = (0..1000).map(|_| (rng.gen(), rng.gen())).collect();

    c.bench_function("primitives/mulr", |b| {
        b.iter(|| {
            let mut sum = 0i32;
            for (a, b_val) in &pairs {
                sum = sum.wrapping_add(mulr(black_box(*a), black_box(*b_val)));
            }
            black_box(sum);
        })
    });
}

fn bench_mulsr(c: &mut Criterion) {
    let mut rng = rand::rngs::StdRng::seed_from_u64(2);
    let pairs: Vec<(i32, i32)> = (0..1000).map(|_| (rng.gen(), rng.gen())).collect();

    c.bench_function("primitives/mulsr", |b| {
        b.iter(|| {
            let mut sum = 0i32;
            for (a, b_val) in &pairs {
                sum = sum.wrapping_add(mulsr(black_box(*a), black_box(*b_val)));
            }
            black_box(sum);
        })
    });
}

fn bench_labs(c: &mut Criterion) {
    let mut rng = rand::rngs::StdRng::seed_from_u64(3);
    let values: Vec<i32> = (0..1000).map(|_| rng.gen()).collect();

    c.bench_function("primitives/labs", |b| {
        b.iter(|| {
            let mut sum = 0i32;
            for v in &values {
                sum = sum.wrapping_add(labs(black_box(*v)));
            }
            black_box(sum);
        })
    });
}

fn bench_labs_edge_cases(c: &mut Criterion) {
    let edge_values = [0i32, 1, -1, i32::MAX, i32::MIN, 42, -42, i32::MAX - 1];

    c.bench_function("primitives/labs_edge_cases", |b| {
        b.iter(|| {
            let mut sum = 0i32;
            for v in &edge_values {
                sum = sum.wrapping_add(labs(black_box(*v)));
            }
            black_box(sum);
        })
    });
}

fn bench_abs_and_sign(c: &mut Criterion) {
    let mut rng = rand::rngs::StdRng::seed_from_u64(4);
    let mut values: Vec<i32> = (0..1000).map(|_| rng.gen_range(-15i32..=15)).collect();

    c.bench_function("primitives/abs_and_sign", |b| {
        b.iter(|| {
            let mut sign_sum = 0u32;
            for v in values.iter_mut() {
                sign_sum = sign_sum.wrapping_add(abs_and_sign(black_box(v)));
            }
            black_box(sign_sum);
        })
    });
}

criterion_group!(
    benches,
    bench_mulr,
    bench_mulsr,
    bench_labs,
    bench_labs_edge_cases,
    bench_abs_and_sign,
);
criterion_main!(benches);
