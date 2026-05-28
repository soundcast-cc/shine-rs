use criterion::{black_box, criterion_group, criterion_main, Criterion};

use rand::{Rng, SeedableRng};
use shine_rs::bitstream::BitstreamWriter;

fn bench_put_bits(c: &mut Criterion) {
    let bit_widths: [(i32, &str); 5] = [
        (1, "1bit"),
        (4, "4bit"),
        (8, "8bit"),
        (12, "12bit"),
        (16, "16bit"),
    ];

    for &(width, label) in &bit_widths {
        let mut values: Vec<u32> = Vec::with_capacity(1000);
        let max_val = if width == 32 { u32::MAX } else { (1u32 << width) - 1 };
        let mut rng = rand::rngs::StdRng::seed_from_u64(5);
        for _ in 0..1000 {
            values.push(rng.gen_range(0..=max_val));
        }

        let name = format!("bitstream/put_bits_{}", label);
        c.bench_function(&name, |b| {
            b.iter(|| {
                let mut bs = BitstreamWriter::new(4096);
                for v in &values {
                    bs.put_bits(black_box(*v), black_box(width)).unwrap();
                }
                black_box(bs.get_bits_count());
            })
        });
    }
}

fn bench_put_bits_mixed(c: &mut Criterion) {
    let mut rng = rand::rngs::StdRng::seed_from_u64(6);
    let operations: Vec<(u32, i32)> = (0..1000)
        .map(|_| {
            let width: i32 = rng.gen_range(1..=17);
            let max_val = (1u32 << width) - 1;
            (rng.gen_range(0..=max_val), width)
        })
        .collect();

    c.bench_function("bitstream/put_bits_mixed", |b| {
        b.iter(|| {
            let mut bs = BitstreamWriter::new(4096);
            for (val, width) in &operations {
                bs.put_bits(black_box(*val), black_box(*width)).unwrap();
            }
            black_box(bs.get_bits_count());
        })
    });
}

criterion_group!(benches, bench_put_bits, bench_put_bits_mixed);
criterion_main!(benches);
