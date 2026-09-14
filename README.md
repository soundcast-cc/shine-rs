# Shine-RS

[中文文档](README_CN.md)

A pure Rust MP3 encoder implementation based on the Shine library. This project strictly follows the Shine C reference implementation, providing complete MP3 Layer III encoding with support for various sample rates, bitrates, and channel configurations.

**Repository**: https://github.com/wshon/shine-rs

[![License: LGPL-2.0](https://img.shields.io/badge/License-LGPL%202.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)

## Features

- 🦀 **Pure Rust** — memory-safe and performant
- 🎯 **Shine-compatible** — algorithms match Shine C implementation exactly for identical output
- 🎵 **Full MP3 Layer III** — complete encoding pipeline
- ⚡ **Fast** — average **114.1x** real-time encoding speed
- 🔧 **Flexible** — multiple sample rates, bitrates, and channel modes
- 📊 **Standard-compliant** — ISO/IEC 11172-3
- 🧪 **Well-tested** — unit tests, integration tests, and reference validation
- 🛠️ **CLI tool** — WAV to MP3 conversion
- 📋 **Debug support** — optional logging and frame limit feature

## Supported Formats

### Sample Rates
- **MPEG-1**: 32000, 44100, 48000 Hz
- **MPEG-2**: 16000, 22050, 24000 Hz
- **MPEG-2.5**: 8000, 11025, 12000 Hz

### Bitrates
- **MPEG-1**: 32-320 kbps
- **MPEG-2**: 8-160 kbps
- **MPEG-2.5**: 8-64 kbps

### Channel Modes
- Mono
- Stereo
- Joint Stereo
- Dual Channel

## Quick Start

### CLI Tool

```bash
# Basic: WAV to MP3
cargo run tests/audio/inputs/basic/sample-3s.wav output.mp3

# Specify bitrate and stereo mode
cargo run input.wav output.mp3 128 stereo

# Debug mode: limit frame count
cargo run input.wav output.mp3 --max-frames 10

# Verbose output
cargo run input.wav output.mp3 --verbose
```

### As a Library

```toml
[dependencies]
shine-rs = { git = "https://github.com/wshon/shine-rs" }
```

```rust
use shine_rs::{Mp3Encoder, Mp3EncoderConfig, StereoMode};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Mp3EncoderConfig::new()
        .sample_rate(44100)
        .bitrate(128)
        .channels(2)
        .stereo_mode(StereoMode::Stereo);

    let mut encoder = Mp3Encoder::new(config)?;
    let pcm_data = vec![0i16; encoder.samples_per_frame()];
    let mp3_frames = encoder.encode_interleaved(&pcm_data)?;
    let final_data = encoder.finish()?;

    println!("Done! Generated {} bytes of MP3 data", final_data.len());
    Ok(())
}
```

> 💡 See [High-Level API Guide](docs/HIGH_LEVEL_API.md) for advanced usage.

## Project Structure

```
shine-rs/
├── 📁 crate/                    # Published library
│   ├── src/                     # Core encoder
│   │   ├── bitstream.rs         # Bitstream handling
│   │   ├── encoder.rs           # Low-level API
│   │   ├── mp3_encoder.rs       # High-level API
│   │   ├── huffman.rs           # Huffman coding
│   │   ├── mdct.rs              # MDCT transform
│   │   ├── quantization.rs      # Quantization loop
│   │   ├── subband.rs           # Subband analysis
│   │   ├── tables.rs            # Lookup tables
│   │   ├── reservoir.rs         # Bit reservoir
│   │   ├── error.rs             # Error handling
│   │   ├── types.rs             # Type definitions
│   │   └── lib.rs               # Library entry
│   └── Cargo.toml
├── 📁 src/                      # CLI tool
├── 📁 tests/                    # Integration tests & benchmarks
├── 📁 ref/shine/                # Shine C reference
├── 📁 docs/                     # Documentation
└── 📁 scripts/                  # Helper scripts
```

### Encoding Pipeline

```
PCM input → Subband Filter → MDCT → Quantization Loop → Huffman → Bitstream output
```

## Development Status

Complete MP3 encoding implementation:

- [x] Project structure and infrastructure
- [x] Configuration management
- [x] Lookup tables and constants
- [x] Bitstream writer
- [x] Subband filter (32-band analysis)
- [x] MDCT (Modified Discrete Cosine Transform)
- [x] Quantization loop (bitrate control)
- [x] Huffman encoder
- [x] Main encoder integration
- [x] Output verified against Shine (SHA256 match)
- [x] Performance optimization (~1.7x speedup)
- [x] Zero compiler warnings, all clippy checks pass
- [x] Full benchmark suite

## Build & Test

```bash
# Build
cargo build --release

# Run all tests
cargo test

# Run benchmarks
python scripts/benchmark_encoders.py

# Run integration tests
cargo test --test integration_full_pipeline_validation

# Run validation tests
cargo test encoder_validation_cicd

# CLI tool
cargo run --release tests/audio/inputs/basic/sample-3s.wav output.mp3
```

## Performance

### 性能优势

#### Benchmark Results (Shine-RS vs Shine C)

Using high-precision internal timing (excludes process startup and I/O):

| Bitrate | Shine-RS | Shine C | Ratio |
|---------|----------|---------|-------|
| 128kbps | 106.2x  | 138.1x  | 0.77x |
| 192kbps | 116.0x  | 129.8x  | 0.89x |
| 320kbps | 120.0x  | 123.5x  | **0.97x** |

**Overall**: Shine-RS 114.1x vs Shine 130.4x — only **14%** gap.

## Compatibility

Generated MP3 files are compatible with:
- FFmpeg/libmp3lame
- Windows Media Player
- VLC Media Player
- All standard MP3 players

## Documentation

- [Project Structure](docs/PROJECT_STRUCTURE.md)
- [Test Data Framework](docs/TEST_DATA_FRAMEWORK.md)
- [Frame Limit Feature](docs/FRAME_LIMIT_FEATURE.md)
- [Frame Limit Quick Ref](docs/FRAME_LIMIT_QUICK_REFERENCE.md)
- [High-Level API Guide](docs/HIGH_LEVEL_API.md)
- [Logging System](docs/LOGGING_SYSTEM.md)
- [Audio File Standardization](docs/AUDIO_FILES_STANDARDIZATION.md)
- [Verification Record](docs/VERIFICATION_RECORD.md)
- [Performance Benchmark](tests/encoder_benchmark.rs)

## License

LGPL-2.0. See [LICENSE](LICENSE).

## Contributing

1. Follow Shine's algorithm implementation exactly
2. All tests must pass, including Shine reference validation
3. Code must pass `cargo clippy` with no warnings
4. Add tests for new features

## Acknowledgments

Based on [Shine](https://github.com/toots/shine) MP3 encoder. Thanks to Gabriel Bouvigne (original core), Pete Everett (fixed-point port), Patrick Roberts (multi-platform library), and the Savonet team for long-term maintenance.
