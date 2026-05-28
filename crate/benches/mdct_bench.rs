use criterion::{black_box, criterion_group, criterion_main, Criterion};

mod bench_utils;
use bench_utils::*;

use shine_rs::mdct::shine_mdct_sub;

fn bench_mdct_sub(c: &mut Criterion) {
    let configs: [(u32, u32, u8, &str); 3] = [
        (44100, 128, 2, "128k_stereo"),
        (44100, 320, 2, "320k_stereo"),
        (44100, 128, 1, "128k_mono"),
    ];

    for (sample_rate, bitrate, channels, label) in &configs {
        let samples_needed = if *channels == 2 { 576 * 2 } else { 576 };
        let pcm = generate_white_noise(samples_needed);

        let name = format!("mdct/mdct_sub_{}", label);
        let sr = *sample_rate;
        let br = *bitrate;
        let ch = *channels;

        c.bench_function(&name, |b| {
            b.iter_batched(
                || (create_encoder(sr, br, ch), pcm.clone()),
                |(mut config, pcm_data)| {
                    config.buffer[0] = pcm_data.as_ptr() as *mut i16;
                    if ch == 2 {
                        config.buffer[1] = unsafe { pcm_data.as_ptr().offset(1) as *mut i16 };
                    }
                    shine_mdct_sub(black_box(&mut config), black_box(ch as i32));
                    black_box(&config.mdct_freq);
                },
                criterion::BatchSize::SmallInput,
            )
        });
    }
}

criterion_group!(benches, bench_mdct_sub);
criterion_main!(benches);
