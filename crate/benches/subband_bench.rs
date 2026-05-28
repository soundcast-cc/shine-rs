use criterion::{black_box, criterion_group, criterion_main, Criterion};

mod bench_utils;
use bench_utils::*;

use shine_rs::types::{Subband, HAN_SIZE, MAX_CHANNELS, SBLIMIT};
use shine_rs::subband::{shine_subband_initialise, shine_window_filter_subband};
use std::sync::OnceLock;

fn get_subband_template() -> &'static Subband {
    static TEMPLATE: OnceLock<Subband> = OnceLock::new();
    TEMPLATE.get_or_init(|| {
        let mut s = Subband::default();
        shine_subband_initialise(&mut s);
        s
    })
}

fn bench_window_filter_subband(c: &mut Criterion) {
    let strides: [usize; 2] = [1, 2];

    for &stride in &strides {
        let label = if stride == 1 { "mono" } else { "stereo_stride" };
        let samples_needed = 32 * stride;
        let buf_data: Vec<i16> = generate_white_noise(samples_needed + 64);

        let name = format!("subband/window_filter_{}", label);

        c.bench_function(&name, |b| {
            b.iter_batched(
                || {
                    let template = get_subband_template();
                    Subband {
                        off: template.off,
                        fl: template.fl,
                        x: [[0i32; HAN_SIZE]; MAX_CHANNELS],
                    }
                },
                |mut subband| {
                    let mut s = [0i32; SBLIMIT];
                    let mut buf_ref: &[i16] = &buf_data;
                    shine_window_filter_subband(
                        black_box(&mut buf_ref),
                        black_box(&mut s),
                        black_box(0),
                        black_box(&mut subband),
                        black_box(stride),
                    );
                    black_box(&s);
                },
                criterion::BatchSize::SmallInput,
            );
        });
    }
}

criterion_group!(benches, bench_window_filter_subband);
criterion_main!(benches);
