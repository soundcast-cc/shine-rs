#![allow(dead_code)]

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use shine_rs::encoder::{NONE, ShineConfig, ShineMpeg, ShineWave};
use shine_rs::types::{GrInfo, L3Loop, ShineGlobalConfig, GRANULE_SIZE};
use std::f64::consts::PI;

pub fn init_l3loop_tables(l3loop: &mut L3Loop) {
    for i in (0..128).rev() {
        l3loop.steptab[i] = (2.0_f64).powf((127 - i as i32) as f64 / 4.0);
        l3loop.steptabi[i] = if (l3loop.steptab[i] * 2.0) > 0x7fffffff as f64 {
            0x7fffffff
        } else {
            (l3loop.steptab[i] * 2.0 + 0.5) as i32
        };
    }
    for i in (0..10000).rev() {
        l3loop.int2idx[i] =
            ((i as f64).sqrt().sqrt() * (i as f64).sqrt() - 0.0946 + 0.5) as i32;
    }
}

pub fn create_encoder(sample_rate: u32, bitrate: u32, channels: u8) -> Box<ShineGlobalConfig> {
    let stereo_mode = if channels == 1 { 3 } else { 0 };
    let mpeg = ShineMpeg {
        mode: stereo_mode,
        bitr: bitrate as i32,
        emph: NONE,
        copyright: 0,
        original: 1,
    };
    let wave = ShineWave {
        channels: channels as i32,
        samplerate: sample_rate as i32,
    };
    shine_rs::encoder::shine_initialise(&ShineConfig { wave, mpeg }).unwrap()
}

pub fn generate_silence(samples: usize) -> Vec<i16> {
    vec![0i16; samples]
}

pub fn generate_sine(freq: f64, sample_rate: u32, duration_secs: f64, channels: u8) -> Vec<i16> {
    let total_samples = (sample_rate as f64 * duration_secs) as usize * channels as usize;
    let mut data = Vec::with_capacity(total_samples);
    for i in 0..(total_samples / channels as usize) {
        let t = i as f64 / sample_rate as f64;
        let sample = (32767.0 * (2.0 * PI * freq * t).sin()) as i16;
        for _ in 0..channels {
            data.push(sample);
        }
    }
    data
}

pub fn generate_white_noise(samples: usize) -> Vec<i16> {
    let mut rng = StdRng::seed_from_u64(42);
    (0..samples).map(|_| rng.gen::<i16>()).collect()
}

pub fn make_test_ix(nonzero_ratio: f64) -> (Box<[i32; GRANULE_SIZE]>, u32, u32) {
    let mut rng = StdRng::seed_from_u64(12345);
    let mut ix = Box::new([0i32; GRANULE_SIZE]);
    let nonzero_count = (GRANULE_SIZE as f64 * nonzero_ratio) as usize;
    for i in 0..nonzero_count.min(GRANULE_SIZE) {
        ix[i] = rng.gen_range(1..16);
    }
    let big_values = (nonzero_count.min(GRANULE_SIZE) / 2).min(288) as u32;
    (ix, big_values, 0)
}

pub fn make_test_ix_for_count1(count1_count: u32) -> (Box<[i32; GRANULE_SIZE]>, u32, u32) {
    let mut rng = StdRng::seed_from_u64(67890);
    let mut ix = Box::new([0i32; GRANULE_SIZE]);
    let start = (GRANULE_SIZE - count1_count as usize * 4).min(GRANULE_SIZE - 4);
    for i in (start..GRANULE_SIZE).step_by(4) {
        ix[i] = rng.gen_range(-1..=1i32);
        ix[i + 1] = rng.gen_range(-1..=1i32);
        ix[i + 2] = rng.gen_range(-1..=1i32);
        ix[i + 3] = rng.gen_range(-1..=1i32);
    }
    let big_values = 0u32;
    (ix, big_values, count1_count)
}

pub fn make_test_xr(max_abs: i32) -> Box<[i32; GRANULE_SIZE]> {
    let mut rng = StdRng::seed_from_u64(99999);
    let mut xr = Box::new([0i32; GRANULE_SIZE]);
    for i in 0..GRANULE_SIZE {
        xr[i] = rng.gen_range(-max_abs..=max_abs);
    }
    xr
}

pub fn make_test_cod_info(big_values: u32, count1: u32) -> GrInfo {
    let mut cod_info = GrInfo::default();
    cod_info.big_values = big_values;
    cod_info.count1 = count1;
    cod_info
}
