use std::time::Instant;
use shine_rs::{Mp3EncoderConfig, StereoMode};

fn make_config(sr: u32, br: u32, ch: u8, use_simd: bool) -> Mp3EncoderConfig {
    Mp3EncoderConfig {
        sample_rate: sr,
        bitrate: br,
        channels: ch,
        stereo_mode: if ch == 1 { StereoMode::Mono } else { StereoMode::Stereo },
        copyright: false,
        original: true,
        use_simd,
    }
}

// Simple deterministic PRNG (xorshift)
fn next_rand(state: &mut u64) -> i16 {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    ((*state >> 32) as i32 % 2000 - 1000) as i16
}

fn main() {
    let mut state = 123456789u64;

    for &sr in &[8000, 22050, 44100] {
        for &br in &[64, 128, 320] {
            for &ch in &[1u8, 2] {
                let samples_per_granule = if sr >= 32000 { 1152 } else { 576 };
                let n = samples_per_granule * ch as usize;
                let samples: Vec<i16> = (0..n).map(|_| next_rand(&mut state)).collect();

                // warmup
                for _ in 0..100 {
                    let _ = shine_rs::encode_pcm_to_mp3(make_config(sr, br, ch, false), &samples);
                    let _ = shine_rs::encode_pcm_to_mp3(make_config(sr, br, ch, true), &samples);
                }

                // bench scalar
                let iters = 500;
                let start = Instant::now();
                for _ in 0..iters {
                    let _ = shine_rs::encode_pcm_to_mp3(make_config(sr, br, ch, false), &samples);
                }
                let scalar_time = start.elapsed() / iters;

                // bench simd
                let start = Instant::now();
                for _ in 0..iters {
                    let _ = shine_rs::encode_pcm_to_mp3(make_config(sr, br, ch, true), &samples);
                }
                let simd_time = start.elapsed() / iters;

                let speedup = scalar_time.as_secs_f64() / simd_time.as_secs_f64();
                let indicator = if speedup > 1.05 { "✅" } else if speedup < 0.95 { "❌" } else { "≈" };
                println!(
                    "{:>6}Hz {:>3}kbps {}ch | scalar: {:>8?} | simd: {:>8?} | {:.2}x {}",
                    sr, br, ch, scalar_time, simd_time, speedup, indicator
                );
            }
        }
    }
}
