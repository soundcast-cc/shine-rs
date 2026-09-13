use std::time::Instant;
use shine_rs::{ShineConfig, StereoMode};
use rand::Rng;

fn encode_one(sr: u32, br: u32, ch: u8, samples: &[i16]) -> Vec<u8> {
    let config = ShineConfig::default()
        .sample_rate(sr)
        .bitrate(br)
        .channels(ch)
        .stereo_mode(match ch {
            1 => StereoMode::Mono,
            _ => StereoMode::Stereo,
        });
    shine_rs::encode_pcm_to_mp3(config, samples).unwrap()
}

fn bench<F: FnOnce() -> R, R>(name: &str, warmup: u32, iters: u32, f: F) {
    for _ in 0..warmup { let _ = f(); }
    let start = Instant::now();
    for _ in 0..iters { let _ = f(); }
    let elapsed = start.elapsed();
    let per_iter = elapsed / iters;
    println!("{:45} {:>12} ({} iter)", name, format!("{:?}", per_iter), iters);
}

fn main() {
    let mut rng = rand::thread_rng();

    println!("=== SIMD path enabled ===");
    for &sr in &[8000, 22050, 44100] {
        for &br in &[64, 128, 320] {
            for &ch in &[1u8, 2] {
                let n = if sr >= 32000 { 1152 } else { 576 };
                let samples: Vec<i16> = (0..n * ch as usize)
                    .map(|_| rng.gen_range(-1000..1000))
                    .collect();
                let name = format!("encode {}Hz {}kbps {}ch", sr, br, ch);
                bench(&name, 50, 200, || encode_one(sr, br, ch, &samples));
            }
        }
    }
}
