//! SIMD utility functions using the `wide` crate.
//! Provides vectorized versions of fixed-point arithmetic primitives
//! operating on 4 i32 lanes at a time.

use wide::{i32x4, i64x4};

/// SIMD version of `labs`: absolute value of 4 i32 lanes.
#[inline]
pub fn labs_i32x4(x: i32x4) -> i32x4 {
    x.abs()
}

/// SIMD version of `mulr`: `(((a * b) + 0x80000000) >> 32) as i32` for 4 lanes.
/// Widening multiply i32→i64, add rounding, shift right 32, narrow to i32.
#[inline]
pub fn mulr_i32x4(a: i32x4, b: i32x4) -> i32x4 {
    let wide: i64x4 = a.mul_widen(b);
    let r: i64x4 = (wide + i64x4::splat(0x80000000i64)) >> 32;
    narrow_i64x4_to_i32x4(r)
}

/// SIMD version of `mulsr`: `(((a * b) + 0x40000000) >> 31) as i32` for 4 lanes.
#[inline]
pub fn mulsr_i32x4(a: i32x4, b: i32x4) -> i32x4 {
    let wide: i64x4 = a.mul_widen(b);
    let r: i64x4 = (wide + i64x4::splat(0x40000000i64)) >> 31;
    narrow_i64x4_to_i32x4(r)
}

/// Narrow `i64x4` to `i32x4` by truncating each lane.
/// wide v1.4 has no built-in narrow operation for i64x4→i32x4.
#[inline]
fn narrow_i64x4_to_i32x4(v: i64x4) -> i32x4 {
    let arr: [i64; 4] = v.to_array();
    i32x4::new([arr[0] as i32, arr[1] as i32, arr[2] as i32, arr[3] as i32])
}
