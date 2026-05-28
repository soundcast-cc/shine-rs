use criterion::{black_box, criterion_group, criterion_main, Criterion};

mod bench_utils;
use bench_utils::*;

use shine_rs::encoder::shine_encode_buffer_interleaved;

fn bench_encode_one_frame(c: &mut Criterion) {
    let configs: [(u32, u32, u8, &str); 4] = [
        (44100, 128, 2, "128k_stereo"),
        (44100, 320, 2, "320k_stereo"),
        (44100, 128, 1, "128k_mono"),
        (44100, 64, 1, "64k_mono"),
    ];

    for (sample_rate, bitrate, channels, label) in &configs {
        let samples_per_frame = if *channels == 2 { 1152 * 2 } else { 1152 };
        let pcm_silence = generate_silence(samples_per_frame);
        let pcm_noise = generate_white_noise(samples_per_frame);
        let pcm_sine = generate_sine(1000.0, *sample_rate, 1152.0 / *sample_rate as f64, *channels);

        for (signal_name, pcm_data) in &[
            ("silence", &pcm_silence),
            ("noise", &pcm_noise),
            ("sine1k", &pcm_sine),
        ] {
            let name = format!(
                "encode_pipeline/one_frame_{}_{}_{}",
                label, signal_name, bitrate
            );

            let pcm = (*pcm_data).clone();
            let sr = *sample_rate;
            let br = *bitrate;
            let ch = *channels;

            c.bench_function(&name, |b| {
                b.iter(|| {
                    let mut encoder = create_encoder(sr, br, ch);
                    let (data, written) = unsafe {
                        shine_encode_buffer_interleaved(
                            black_box(&mut encoder),
                            black_box(pcm.as_ptr()),
                        )
                    }
                    .unwrap();
                    black_box((data, written));
                })
            });
        }
    }
}

criterion_group!(benches, bench_encode_one_frame);
criterion_main!(benches);
