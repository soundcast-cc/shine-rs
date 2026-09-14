[English Documentation](README.md)

# MP3编码器参考数据生成脚本

## 概述

`generate_reference_data.py` 是一个完整的参考数据生成脚本，用于为MP3编码器集成测试生成真实的Shine参考数据。

## 功能

该脚本自动完成以下步骤：

1. **运行Shine编码器** - 使用指定的音频文件和参数运行Shine编码器
2. **捕获调试输出** - 提取MDCT系数、量化参数和比特流数据
3. **解析调试数据** - 从Shine的调试输出中提取关键算法参数
4. **生成JSON测试数据** - 创建包含真实参考值的JSON测试文件
5. **计算文件哈希** - 生成MP3文件的SHA256哈希用于验证

## 使用方法

```bash
# 生成所有参考数据
python scripts/generate_reference_data.py
```

## 生成的数据

脚本会在 `testing/fixtures/data/` 目录下生成以下文件：

- `sample-3s_128k_3f_real.json` - 3秒样本，128kbps，3帧
- `voice_recorder_128k_3f_real.json` - 语音录音，128kbps，3帧  
- `free_test_data_128k_3f_real.json` - 免费测试数据，128kbps，3帧
- `sample-3s_192k_3f_real.json` - 3秒样本，192kbps，3帧

## 数据结构

每个JSON文件包含：

### 元数据 (metadata)
- `name`: 测试用例名称
- `input_file`: 输入音频文件路径
- `expected_output_size`: 预期MP3文件大小
- `expected_hash`: 预期SHA256哈希值
- `created_at`: 创建时间
- `description`: 描述信息
- `generated_by`: 生成工具信息

### 配置 (config)
- `sample_rate`: 采样率
- `channels`: 声道数
- `bitrate`: 比特率
- `stereo_mode`: 立体声模式 (0=立体声, 3=单声道)
- `mpeg_version`: MPEG版本 (3=MPEG-I)

### 帧数据 (frames)
每帧包含：

#### MDCT系数 (mdct_coefficients)
- `coefficients`: MDCT系数 [k17, k16, k15]
- `l3_sb_sample`: 子带样本数据

#### 量化参数 (quantization)
- `xrmax`: 最大频谱值
- `max_bits`: 最大比特数
- `part2_3_length`: Part2/3长度
- `quantizer_step_size`: 量化步长
- `global_gain`: 全局增益

#### 比特流参数 (bitstream)
- `padding`: 填充位
- `bits_per_frame`: 每帧比特数
- `written`: 写入字节数
- `slot_lag`: 时隙延迟

## 依赖要求

- Python 3.6+
- Shine编码器 (`ref/shine/shineenc.exe`)
- 测试音频文件在 `tests/audio/` 目录

## 配置

要添加新的测试配置，修改脚本中的 `TEST_CONFIGS` 列表：

```python
TEST_CONFIGS = [
    {
        "name": "my_test_128k_3f_real",
        "audio_file": "tests/audio/my_audio.wav",
        "bitrate": 128,
        "frames": 3,
        "description": "My custom test case"
    }
]
```

## 验证

生成的参考数据可用于集成测试：

```bash
# 运行集成测试验证Rust实现与Shine的一致性
cargo test test_complete_encoding_pipeline
```

## 注意事项

1. **Shine调试输出**: 脚本依赖于Shine编码器的调试输出，确保使用包含调试信息的Shine版本
2. **路径处理**: 脚本会自动处理相对路径和绝对路径
3. **帧限制**: 使用环境变量 `SHINE_MAX_FRAMES` 限制编码帧数
4. **数据精度**: 所有数值都与Shine的输出完全一致，确保测试的准确性

## 故障排除

### 常见问题

1. **"Shine encoder not found"**
   - 确保 `ref/shine/shineenc.exe` 存在
   - 检查Shine是否正确编译

2. **"Audio file not found"**
   - 确保音频文件存在于指定路径
   - 检查文件路径是否正确

3. **"No debug data extracted"**
   - 确保Shine版本包含调试输出
   - 检查环境变量 `SHINE_MAX_FRAMES` 是否设置

### 调试技巧

- 查看Shine的标准输出和错误输出
- 检查生成的MP3文件是否存在
- 验证WAV文件格式是否正确 (16位PCM)

---

# MP3 Encoder Reference Data Generation Script

## Overview

`generate_reference_data.py` is a complete reference data generation script for generating real Shine reference data for MP3 encoder integration testing.

## Features

The script automatically performs the following steps:

1. **Run Shine encoder** - Run Shine encoder with specified audio files and parameters
2. **Capture debug output** - Extract MDCT coefficients, quantization parameters, and bitstream data
3. **Parse debug data** - Extract key algorithm parameters from Shine's debug output
4. **Generate JSON test data** - Create JSON test files containing real reference values
5. **Calculate file hash** - Generate SHA256 hash of MP3 files for verification

## Usage

```bash
# Generate all reference data
python scripts/generate_reference_data.py
```

## Generated Data

The script generates the following files in the `testing/fixtures/data/` directory:

- `sample-3s_128k_3f_real.json` - 3-second sample, 128kbps, 3 frames
- `voice_recorder_128k_3f_real.json` - Voice recording, 128kbps, 3 frames  
- `free_test_data_128k_3f_real.json` - Free test data, 128kbps, 3 frames
- `sample-3s_192k_3f_real.json` - 3-second sample, 192kbps, 3 frames

## Data Structure

Each JSON file contains:

### Metadata
- `name`: Test case name
- `input_file`: Input audio file path
- `expected_output_size`: Expected MP3 file size
- `expected_hash`: Expected SHA256 hash value
- `created_at`: Creation time
- `description`: Description
- `generated_by`: Generator tool information

### Configuration
- `sample_rate`: Sample rate
- `channels`: Number of channels
- `bitrate`: Bitrate
- `stereo_mode`: Stereo mode (0=stereo, 3=mono)
- `mpeg_version`: MPEG version (3=MPEG-I)

### Frame Data
Each frame contains:

#### MDCT Coefficients
- `coefficients`: MDCT coefficients [k17, k16, k15]
- `l3_sb_sample`: Subband sample data

#### Quantization Parameters
- `xrmax`: Maximum spectral value
- `max_bits`: Maximum bits
- `part2_3_length`: Part2/3 length
- `quantizer_step_size`: Quantizer step size
- `global_gain`: Global gain

#### Bitstream Parameters
- `padding`: Padding bits
- `bits_per_frame`: Bits per frame
- `written`: Bytes written
- `slot_lag`: Slot lag

## Requirements

- Python 3.6+
- Shine encoder (`ref/shine/shineenc.exe`)
- Test audio files in `tests/audio/` directory

## Configuration

To add new test configurations, modify the `TEST_CONFIGS` list in the script:

```python
TEST_CONFIGS = [
    {
        "name": "my_test_128k_3f_real",
        "audio_file": "tests/audio/my_audio.wav",
        "bitrate": 128,
        "frames": 3,
        "description": "My custom test case"
    }
]
```

## Verification

Generated reference data can be used for integration tests:

```bash
# Run integration tests to verify Rust implementation matches Shine
cargo test test_complete_encoding_pipeline
```

## Notes

1. **Shine debug output**: The script depends on Shine encoder's debug output; ensure you use a version with debug info
2. **Path handling**: The script automatically handles relative and absolute paths
3. **Frame limit**: Use environment variable `SHINE_MAX_FRAMES` to limit encoding frames
4. **Data precision**: All values are identical to Shine's output to ensure test accuracy

## Troubleshooting

### Common Issues

1. **"Shine encoder not found"**
   - Ensure `ref/shine/shineenc.exe` exists
   - Check if Shine is compiled correctly

2. **"Audio file not found"**
   - Ensure audio files exist at the specified path
   - Check if file paths are correct

3. **"No debug data extracted"**
   - Ensure Shine version includes debug output
   - Check if environment variable `SHINE_MAX_FRAMES` is set

### Debugging Tips

- View Shine's stdout and stderr
- Check if generated MP3 files exist
- Verify WAV file format is correct (16-bit PCM)
