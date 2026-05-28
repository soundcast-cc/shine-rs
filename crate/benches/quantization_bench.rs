use criterion::{black_box, criterion_group, criterion_main, Criterion, BatchSize};

mod bench_utils;
use bench_utils::*;

use shine_rs::quantization::{
    calc_runlen, count1_bitcount, count_bit, ix_max, quantize_with_l3loop,
};
use shine_rs::types::L3Loop;

fn make_fresh_l3loop(xr: &mut [i32; 576]) -> L3Loop {
    let mut l3loop = L3Loop::default();
    init_l3loop_tables(&mut l3loop);
    l3loop.xr = xr.as_mut_ptr();
    for i in 0..576 {
        let xr_val = xr[i];
        l3loop.xrsq[i] = shine_rs::quantization::mulsr(xr_val, xr_val);
        l3loop.xrabs[i] = shine_rs::quantization::labs(xr_val);
        if l3loop.xrabs[i] > l3loop.xrmax {
            l3loop.xrmax = l3loop.xrabs[i];
        }
    }
    l3loop
}

fn bench_quantize_with_l3loop(c: &mut Criterion) {
    let stepsizes = [-60, -30, 0, 30, 60, 90, 120];

    for &stepsize in &stepsizes {
        let name = format!("quantization/quantize_with_l3loop_step{}", stepsize);
        let xr_template = make_test_xr(10000);
        let mut ix_buf = [0i32; 576];

        c.bench_function(&name, |b| {
            b.iter_batched(
                || {
                    let mut xr = xr_template.clone();
                    let l3loop = make_fresh_l3loop(&mut xr);
                    (xr, l3loop)
                },
                |(_xr, mut l3loop)| {
                    let result = quantize_with_l3loop(
                        black_box(&mut ix_buf),
                        black_box(stepsize),
                        black_box(&mut l3loop),
                    );
                    black_box(result);
                    black_box(&ix_buf);
                },
                BatchSize::SmallInput,
            );
        });
    }
}

fn bench_count_bit(c: &mut Criterion) {
    let (ix, big_values, _) = make_test_ix(0.8);
    let range_sizes: [(u32, u32); 3] = [(0, 100), (0, 300), (0, big_values * 2)];

    for (table, label) in &[(0u32, "table0"), (15u32, "table15")] {
        for &(start, end) in &range_sizes {
            if end == 0 {
                continue;
            }
            let name = format!("quantization/count_bit_{}_range_{}", label, end);
            c.bench_function(&name, |b| {
                b.iter(|| {
                    let bits = count_bit(
                        black_box(&*ix),
                        black_box(start),
                        black_box(end),
                        black_box(*table),
                    );
                    black_box(bits);
                })
            });
        }
    }
}

fn bench_ix_max(c: &mut Criterion) {
    let (ix, big_values, _) = make_test_ix(0.8);
    let end = (big_values * 2).max(1);

    c.bench_function("quantization/ix_max", |b| {
        b.iter(|| {
            let m = ix_max(black_box(&*ix), black_box(0), black_box(end));
            black_box(m);
        })
    });
}

fn bench_calc_runlen(c: &mut Criterion) {
    let (ix_base, big_values, _) = make_test_ix(0.8);
    let cod_info = make_test_cod_info(big_values, 0);

    c.bench_function("quantization/calc_runlen", |b| {
        b.iter_batched(
            || {
                let ix = *ix_base;
                (ix, cod_info.clone())
            },
            |(mut ix, mut ci)| {
                calc_runlen(black_box(&mut ix), black_box(&mut ci));
                black_box(ci.count1);
            },
            BatchSize::SmallInput,
        );
    });
}

fn bench_count1_bitcount(c: &mut Criterion) {
    let count1s = [0u32, 5, 20, 50];
    for &count1 in &count1s {
        let (ix, big_values, _) = make_test_ix_for_count1(count1);
        let mut cod_info = make_test_cod_info(big_values, count1);

        let name = format!("quantization/count1_bitcount_c1_{}", count1);
        c.bench_function(&name, |b| {
            b.iter(|| {
                let bits = count1_bitcount(black_box(&*ix), black_box(&mut cod_info));
                black_box(bits);
            })
        });
    }
}

criterion_group!(
    benches,
    bench_quantize_with_l3loop,
    bench_count_bit,
    bench_ix_max,
    bench_calc_runlen,
    bench_count1_bitcount,
);
criterion_main!(benches);
