#![cfg_attr(feature = "nightly", feature(array_chunks))]
#![cfg_attr(feature = "simd", feature(portable_simd))]
#![cfg_attr(test, feature(core_intrinsics, test))]
#![no_std]

//! This decoder is largely taking from this article. <https://dev.to/tiemen/implementing-base64-from-scratch-in-rust-kb1>
//! It goes into detail about the entire thing, and why it works the way it does, and I hightly reccomend reading it
//! Very big thanks to Tiemen for writing it!
//!
//! The usage of u64s as byte arrays is taken from the base64 crate, which is under the MIT licsense

extern crate alloc;

#[cfg(feature = "simd")]
use core::{
    mem::{transmute, transmute_copy},
    simd::{Which::*, *},
    slice,
};

use alloc::{string::String, vec};

/// BASE64 encoder struct
pub struct Encoder {
    encode_table: [u8; 64],
}

impl Encoder {
    /// Creates a new instance of the encoder using the default base64 alphabet
    ///
    /// # Examples
    /// ```
    /// const ENCODER: Encoder = Encoder::new();
    /// ```
    pub const fn new() -> Self {
        Self {
            encode_table: [
                b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N',
                b'O', b'P', b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y', b'Z', b'a', b'b',
                b'c', b'd', b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p',
                b'q', b'r', b's', b't', b'u', b'v', b'w', b'x', b'y', b'z', b'0', b'1', b'2', b'3',
                b'4', b'5', b'6', b'7', b'8', b'9', b'+', b'/',
            ],
        }
    }

    #[cfg(not(feature = "simd"))]
    /// Creates a new instance of the encoder using a specified alphabet
    ///
    /// # Example:
    /// ```
    /// const ALPHABET: [u8; 64] = [
    ///     b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N',
    ///     b'O', b'P', b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y', b'Z', b'a', b'b',
    ///     b'c', b'd', b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p',
    ///     b'q', b'r', b's', b't', b'u', b'v', b'w', b'x', b'y', b'z', b'0', b'1', b'2', b'3',
    ///     b'4', b'5', b'6', b'7', b'8', b'9', b'+', b'/',
    /// ];
    ///
    /// const ENCODER: Encoder = Encoder::with_encode_table(ALPHABET);
    /// ```
    pub const fn with_encode_table(encode_table: [u8; 64]) -> Self {
        Self { encode_table }
    }

    #[rustfmt::skip]
    /// Converts the buffer to base64, uses an out paramater to avoid allocations
    fn internal_encode(&self, buf: &[u8], out: &mut [u8]) {
        #[cfg(feature = "simd")]
        let chunks = buf.array_chunks::<12>();
        #[cfg(feature = "simd")]
        let out_chunks = out.array_chunks_mut::<16>();

        #[cfg(all(feature = "nightly", not(feature = "simd")))]
        let chunks = buf.array_chunks::<24>();
        #[cfg(all(feature = "nightly", not(feature = "simd")))]
        let out_chunks = out.array_chunks_mut::<32>();

        #[cfg(not(feature = "nightly"))]
        let chunks = buf.chunks_exact(24);
        #[cfg(not(feature = "nightly"))]
        let out_chunks = out.chunks_exact_mut(32);

        let mut output_index = 0;
        let rem = chunks.remainder();

        #[cfg(not(feature = "simd"))]
        {
            chunks.zip(out_chunks).for_each(|(chunk, out)| {
                #[cfg(not(feature = "nightly"))]
                let out: &mut [u8; 32] = out.try_into().unwrap();

                let byte_array_1 = u64::from_be_bytes([
                    0, 0, chunk[0], chunk[1], chunk[2], chunk[3], chunk[4], chunk[5],
                ]);

                let byte_array_2 = u64::from_be_bytes([
                    0, 0, chunk[6], chunk[7], chunk[8], chunk[9], chunk[10], chunk[11],
                ]);

                let byte_array_3 = u64::from_be_bytes([
                    0, 0, chunk[12], chunk[13], chunk[14], chunk[15], chunk[16], chunk[17],
                ]);

                let byte_array_4 = u64::from_be_bytes([
                    0, 0, chunk[18], chunk[19], chunk[20], chunk[21], chunk[22], chunk[23],
                ]);

                *out = [
                    self.encode_table[(byte_array_1 >> 42 & 0b00111111) as usize],
                    self.encode_table[(byte_array_1 >> 36 & 0b00111111) as usize],
                    self.encode_table[(byte_array_1 >> 30 & 0b00111111) as usize],
                    self.encode_table[(byte_array_1 >> 24 & 0b00111111) as usize],
                    self.encode_table[(byte_array_1 >> 18 & 0b00111111) as usize],
                    self.encode_table[(byte_array_1 >> 12 & 0b00111111) as usize],
                    self.encode_table[(byte_array_1 >> 6 & 0b00111111) as usize],
                    self.encode_table[(byte_array_1 & 0b00111111) as usize],
                    self.encode_table[(byte_array_2 >> 42 & 0b00111111) as usize],
                    self.encode_table[(byte_array_2 >> 36 & 0b00111111) as usize],
                    self.encode_table[(byte_array_2 >> 30 & 0b00111111) as usize],
                    self.encode_table[(byte_array_2 >> 24 & 0b00111111) as usize],
                    self.encode_table[(byte_array_2 >> 18 & 0b00111111) as usize],
                    self.encode_table[(byte_array_2 >> 12 & 0b00111111) as usize],
                    self.encode_table[(byte_array_2 >> 6 & 0b00111111) as usize],
                    self.encode_table[(byte_array_2 & 0b00111111) as usize],
                    self.encode_table[(byte_array_3 >> 42 & 0b00111111) as usize],
                    self.encode_table[(byte_array_3 >> 36 & 0b00111111) as usize],
                    self.encode_table[(byte_array_3 >> 30 & 0b00111111) as usize],
                    self.encode_table[(byte_array_3 >> 24 & 0b00111111) as usize],
                    self.encode_table[(byte_array_3 >> 18 & 0b00111111) as usize],
                    self.encode_table[(byte_array_3 >> 12 & 0b00111111) as usize],
                    self.encode_table[(byte_array_3 >> 6 & 0b00111111) as usize],
                    self.encode_table[(byte_array_3 & 0b00111111) as usize],
                    self.encode_table[(byte_array_4 >> 42 & 0b00111111) as usize],
                    self.encode_table[(byte_array_4 >> 36 & 0b00111111) as usize],
                    self.encode_table[(byte_array_4 >> 30 & 0b00111111) as usize],
                    self.encode_table[(byte_array_4 >> 24 & 0b00111111) as usize],
                    self.encode_table[(byte_array_4 >> 18 & 0b00111111) as usize],
                    self.encode_table[(byte_array_4 >> 12 & 0b00111111) as usize],
                    self.encode_table[(byte_array_4 >> 6 & 0b00111111) as usize],
                    self.encode_table[(byte_array_4 & 0b00111111) as usize],
                ];
            });
        }

        // Original Github: https://github.com/lemire/fastbase64/tree/master

        // Copyright (c) 2015-2016, Wojciech Muła, Alfred Klomp,  Daniel Lemire
        // (Unless otherwise stated in the source code)
        // All rights reserved.

        // Redistribution and use in source and binary forms, with or without
        // modification, are permitted provided that the following conditions are
        // met:

        // 1. Redistributions of source code must retain the above copyright
        //    notice, this list of conditions and the following disclaimer.

        // 2. Redistributions in binary form must reproduce the above copyright
        //    notice, this list of conditions and the following disclaimer in the
        //    documentation and/or other materials provided with the distribution.

        // THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS
        // IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED
        // TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A
        // PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
        // HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
        // SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED
        // TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
        // PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
        // LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
        // NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
        // SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

        // Rust portable-simd port and additional optimizations 
        // authored by AlsoSylv and burgerindividual
        #[cfg(feature = "simd")]
        {
            pub fn unpack_with_bswap(input: u8x16) -> u8x16 {
                let in_u8 = simd_swizzle!(input, [1, 0, 2, 1, 4, 3, 5, 4, 7, 6, 8, 7, 10, 9, 11, 10,]);

                // # SAFETY: Transmute is only used to convert between same sized simd types
                unsafe {
                    let in_u32: u32x4 = transmute(in_u8);

                    let t0 = in_u32 & u32x4::splat(0x0fc0fc00);
                    let t0_u16 = transmute::<_, u16x8>(t0);

                    let t1 = simd_swizzle!(
                        t0_u16 >> Simd::splat(10),
                        t0_u16 >> Simd::splat(6),
                        [
                            First(0),
                            Second(1),
                            First(2),
                            Second(3),
                            First(4),
                            Second(5),
                            First(6),
                            Second(7),
                        ]
                    );

                    let t2 = in_u32 & u32x4::splat(0x003f03f0);

                    let t3 = transmute::<_, u16x8>(t2)
                        * u16x8::from_array([
                            0x0010, 0x0100, 0x0010, 0x0100, 0x0010, 0x0100, 0x0010, 0x0100,
                        ]);

                    transmute(t1 | t3)
                }
            }

            fn enc_translate(input: u8x16) -> u8x16 {
                let lut: Simd<i8, 16> = Simd::from_array([
                    65, 71, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -19, -16, 0, 0,
                ]);
                let indicies = input.saturating_sub(Simd::splat(51));
                let mask = input
                    .simd_gt(Simd::splat(25))
                    .to_int()
                    .cast::<u8>();

                let indicies = indicies - mask;

                let out = input.cast::<i8>() + swizzle_dyn(lut.cast::<u8>(), indicies).cast::<i8>();

                out.cast::<u8>()
            }

            chunks.zip(out_chunks).for_each(|(chunk, out)| {
                // # SAFETY: We ignore the last four values, which are the only ones that would be random
                let buf = unsafe { slice::from_raw_parts(chunk.as_ptr(), 16) };
                let vec: Simd<u8, 16> = Simd::from_slice(buf);

                let indicies = unpack_with_bswap(vec);

                let chars = enc_translate(indicies);

                *out = chars.to_array();

                output_index += 16;
            });
        }

        let rem_out = &mut out[output_index..];

        #[cfg(feature = "nightly")]
        let chunks = rem.array_chunks::<3>();
        #[cfg(feature = "nightly")]
        let out_chunks = rem_out.array_chunks_mut::<4>();

        #[cfg(not(feature = "nightly"))]
        let chunks = rem.chunks_exact(3);
        #[cfg(not(feature = "nightly"))]
        let out_chunks = rem_out.chunks_exact_mut(4);

        let rem = chunks.remainder();

        chunks.zip(out_chunks).for_each(|(chunk, out)| {
            #[cfg(not(feature = "nightly"))]
            let out: &mut [u8; 4] = out.try_into().unwrap();

            let byte_array = u32::from_be_bytes([chunk[0], chunk[1], chunk[2], 0]);
            let bit_1 = byte_array >> 26 & 0b00111111;
            let bit_2 = byte_array >> 20 & 0b00111111;
            let bit_3 = byte_array >> 14 & 0b00111111;
            let bit_4 = byte_array >> 8 & 0b00111111;

            *out = [
                self.encode_table[bit_1 as usize],
                self.encode_table[bit_2 as usize],
                self.encode_table[bit_3 as usize],
                self.encode_table[bit_4 as usize],
            ];

            output_index += 4;
        });

        let rem_out = &mut out[output_index..];

        #[cfg(feature = "nightly")]
        let chunks = rem.array_chunks::<2>();
        #[cfg(feature = "nightly")]
        let out_chunks = rem_out.array_chunks_mut::<3>();

        #[cfg(not(feature = "nightly"))]
        let chunks = rem.chunks_exact(2);
        #[cfg(not(feature = "nightly"))]
        let out_chunks = rem_out.chunks_exact_mut(3);

        let rem = chunks.remainder();

        chunks.zip(out_chunks).for_each(|(chunk, out)| {
            #[cfg(not(feature = "nightly"))]
            let out: &mut [u8; 3] = out.try_into().unwrap();

            let byte_array = u16::from_be_bytes([chunk[0], chunk[1]]);
            let bit_1 = byte_array >> 10 & 0b00111111;
            let bit_2 = byte_array >> 4 & 0b00111111;
            let bit_3 = byte_array << 2 & 0b00111111;

            *out = [
                self.encode_table[bit_1 as usize],
                self.encode_table[bit_2 as usize],
                self.encode_table[bit_3 as usize],
            ];

            output_index += 3;
        });


        let rem_out = &mut out[output_index..];

        #[cfg(feature = "nightly")]
        let chunks = rem.array_chunks::<1>();
        #[cfg(feature = "nightly")]
        let out_chunks = rem_out.array_chunks_mut::<2>();

        #[cfg(not(feature = "nightly"))]
        let chunks = rem.chunks_exact(1);
        #[cfg(not(feature = "nightly"))]
        let out_chunks = rem_out.chunks_exact_mut(2);

        chunks.zip(out_chunks).for_each(|(chunk, out)| {
            #[cfg(not(feature = "nightly"))]
            let out: &mut [u8; 2] = out.try_into().unwrap();

            let byte = chunk[0];
            let bit_1 = byte >> 2;
            let bit_2 = (byte & 0b00000011) << 4;

            *out = [
                self.encode_table[bit_1 as usize],
                self.encode_table[bit_2 as usize],
            ];

            output_index += 2;
        });
    }

    /// Converts the bytes to BASE64
    ///
    /// # Examples
    /// ```
    /// const ENCODER: Encoder = Encoder::new();
    ///
    /// let base64_encoded = ENCODER.encode("Hello, World!");
    /// ```
    pub fn encode<T>(&self, bytes: T) -> String
    where
        T: AsRef<[u8]>,
    {
        let buf = bytes.as_ref();
        let mut out = vec![b'='; div_ceil(buf.len(), 3) * 4];
        self.internal_encode(buf, &mut out);

        String::from_utf8(out).unwrap()
    }

    pub fn encode_with_ascii_check<T>(&self, bytes: T) -> String
    where
        T: AsRef<[u8]>,
    {
        let buf = bytes.as_ref();
        let mut out = vec![b'='; div_ceil(buf.len(), 3) * 4];
        self.internal_encode(buf, &mut out);

        assert!(is_ascii(&out));

        // Safety: This is checked to be valid ASCII
        // Which also makes it valid UTF-8
        unsafe { String::from_utf8_unchecked(out) }
    }

    /// Converts the bytes to BASE64, but doesn't check if the output is valid UTF-8
    ///
    /// # Example:
    /// ```
    /// const ENCODER: Encoder = Encoder::new();
    ///
    /// let base64_encoded = unsafe { ENCODER.encode_unchecked("Hello, World!") };
    /// ```
    ///
    /// # Safety
    ///
    /// The characters used for the encode table need to be valid UTF-8
    /// This is true with the default table, but might not be for custom ones.
    ///
    pub unsafe fn encode_unchecked<T>(&self, bytes: T) -> String
    where
        T: AsRef<[u8]>,
    {
        let buf = bytes.as_ref();
        let mut out = vec![b'='; div_ceil(buf.len(), 3) * 4];
        self.internal_encode(buf, &mut out);

        String::from_utf8_unchecked(out)
    }

    /// Converts the bytes to BASE64 without padding
    ///
    /// # Examples
    /// ```
    /// const ENCODER: Encoder = Encoder::new();
    ///
    /// let base64_encoded = ENCODER.encode_without_padding("Hello, World!");
    /// ```
    pub fn encode_without_padding<T>(&self, bytes: T) -> String
    where
        T: AsRef<[u8]>,
    {
        let buf = bytes.as_ref();
        let mut out = vec![0; div_ceil(buf.len(), 3) * 4];
        self.internal_encode(buf, &mut out);

        String::from_utf8(out).unwrap()
    }

    /// Converts the bytes to BASE64 without padding, but doesn't check if the output is valid UTF-8
    ///
    /// # Example:
    /// ```
    /// const ENCODER: Encoder = Encoder::new();
    ///
    /// let base64_encoded = unsafe { ENCODER.encode_unchecked("Hello, World!") };
    /// ```
    ///
    /// # Safety
    ///
    /// The characters used for the encode table need to be valid UTF-8
    /// This is true with the default table, but might not be for custom ones.
    ///
    pub unsafe fn encode_unchecked_without_padding<T>(&self, bytes: T) -> String
    where
        T: AsRef<[u8]>,
    {
        let buf = bytes.as_ref();
        let mut out = vec![0; div_ceil(buf.len(), 3) * 4];
        self.internal_encode(buf, &mut out);

        String::from_utf8_unchecked(out)
    }
}

#[cfg(feature = "simd")]
fn is_ascii(buffer: &[u8]) -> bool {
    let mut mask: Simd<i8, 16> = Simd::splat(0);
    let spans = buffer.array_chunks::<256>();
    let rem = spans.remainder();

    let mut has_error: Mask<i8, 16>;

    for span in spans {
        let chunks = span.array_chunks::<16>();

        for chunk in chunks {
            let current_bytes = Simd::from_slice(chunk);
            mask |= current_bytes.cast();
        }

        has_error = mask.simd_lt(Simd::splat(0));

        if has_error.any() {
            return false;
        }
    }

    let chunks = rem.array_chunks::<16>();
    let rem = chunks.remainder();

    for chunk in chunks {
        let current_bytes = Simd::from_slice(chunk);
        mask |= current_bytes.cast();
    }

    has_error = mask.simd_lt(Simd::splat(0));

    if has_error.any() {
        return false;
    }

    for x in rem {
        if x > &127 {
            return false;
        }
    }

    true
}

#[cfg(not(feature = "simd"))]
fn is_ascii(buffer: &[u8]) -> bool {
    #[cfg(feature = "nightly")]
    let spans = buffer.array_chunks::<256>();
    #[cfg(not(feature = "nightly"))]
    let spans = buffer.chunks_exact(256);
    let rem = spans.remainder();
    for span in spans {
        if span > &[127; 256] {
            return false;
        }
    }

    for x in rem {
        if x > &127 {
            return false;
        }
    }

    true
}

#[inline]
const fn div_ceil(divisor: usize, dividend: usize) -> usize {
    let quotient = divisor / dividend;
    let remainder = divisor % dividend;

    if remainder > 0 {
        quotient + 1
    } else {
        quotient
    }
}

/// Swizzle a vector of bytes according to the index vector.
/// Indices within range select the appropriate byte.
/// Indices "out of bounds" instead select 0.
///
/// Note that the current implementation is selected during build-time
/// of the standard library, so `cargo build -Zbuild-std` may be necessary
/// to unlock better performance, especially for larger vectors.
/// A planned compiler improvement will enable using `#[target_feature]` instead.
#[cfg(feature = "simd")]
#[inline]
fn swizzle_dyn<const N: usize>(val: Simd<u8, N>, idxs: Simd<u8, N>) -> Simd<u8, N>
where
    LaneCount<N>: SupportedLaneCount,
{
    #![allow(unused_imports, unused_unsafe)]
    #[cfg(all(target_arch = "aarch64", target_endian = "little"))]
    use core::arch::aarch64::{uint8x8_t, vqtbl1q_u8, vtbl1_u8};
    #[cfg(all(target_arch = "arm", target_feature = "v7", target_endian = "little"))]
    use core::arch::arm::{uint8x8_t, vtbl1_u8};
    #[cfg(target_arch = "wasm32")]
    use core::arch::wasm32 as wasm;
    #[cfg(target_arch = "x86")]
    use core::arch::x86;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64 as x86;
    // SAFETY: Intrinsics covered by cfg
    unsafe {
        #[cfg(target_feature = "ssse3")]
        return transize(x86::_mm_shuffle_epi8, val, idxs);
        #[cfg(target_feature = "simd128")]
        return transize(wasm::i8x16_swizzle, val, idxs);
        #[cfg(all(
            target_arch = "aarch64",
            target_feature = "neon",
            target_endian = "little"
        ))]
        return transize(vqtbl1q_u8, val, idxs);
    }
}

/// This sets up a call to an architecture-specific function, and in doing so
/// it persuades rustc that everything is the correct size. Which it is.
/// This would not be needed if one could convince Rust that, by matching on N,
/// N is that value, and thus it would be valid to substitute e.g. 16.
///
/// # Safety
/// The correctness of this function hinges on the sizes agreeing in actuality.
#[cfg(feature = "simd")]
#[allow(dead_code)]
#[inline(always)]
unsafe fn transize<T, const N: usize>(
    f: unsafe fn(T, T) -> T,
    bytes: Simd<u8, N>,
    idxs: Simd<u8, N>,
) -> Simd<u8, N>
where
    LaneCount<N>: SupportedLaneCount,
{
    let idxs = zeroing_idxs(idxs);
    // SAFETY: Same obligation to use this function as to use transmute_copy.
    unsafe { transmute_copy(&f(transmute_copy(&bytes), transmute_copy(&idxs))) }
}

/// Make indices that yield 0 for this architecture
#[cfg(feature = "simd")]
#[inline(always)]
fn zeroing_idxs<const N: usize>(idxs: Simd<u8, N>) -> Simd<u8, N>
where
    LaneCount<N>: SupportedLaneCount,
{
    // On x86, make sure the top bit is set.
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    let idxs = {
        idxs.simd_lt(Simd::splat(N as u8))
            .select(idxs, Simd::splat(u8::MAX))
    };
    // Simply do nothing on most architectures.
    idxs
}

#[cfg(test)]
/*
These are the current benchmark results running on an Ryzen 7 5700u

Simd Feature Enabled:
base64_crate               ... bench: 8,935,681 ns/iter (+/- 289,240)
irelia_encoder             ... bench: 4,270,517 ns/iter (+/- 61,791)
irelia_encoder_ascii_check ... bench: 3,595,404 ns/iter (+/- 79,627)
irelia_encoder_unchecked   ... bench: 3,306,281 ns/iter (+/- 39,131)

Nightly Feature Enabled:
base64_crate               ... bench: 8,896,750 ns/iter (+/- 148,658)
irelia_encoder             ... bench: 7,312,479 ns/iter (+/- 24,627)
irelia_encoder_ascii_check ... bench: 6,871,569 ns/iter (+/- 18,158)
irelia_encoder_unchecked   ... bench: 6,589,772 ns/iter (+/- 55,076)

Stable Toolchain:
base64_crate               ... bench: 9,042,472 ns/iter (+/- 261,678)
irelia_encoder             ... bench: 7,382,325 ns/iter (+/- 87,712)
irelia_encoder_ascii_check ... bench: 6,951,625 ns/iter (+/- 21,298)
irelia_encoder_unchecked   ... bench: 6,578,281 ns/iter (+/- 21,827)

Stable Toolchain no rustflags:
base64_crate               ... bench: 8,532,239 ns/iter (+/- 31,178)
irelia_encoder             ... bench: 8,069,776 ns/iter (+/- 95,354)
irelia_encoder_ascii_check ... bench: 7,568,010 ns/iter (+/- 23,848)
irelia_encoder_unchecked   ... bench: 7,190,625 ns/iter (+/- 58,626)
*/
mod test {
    extern crate test;

    use alloc::{string::String, vec};
    use core::intrinsics::black_box;
    use test::Bencher;

    use base64::{engine::general_purpose, Engine};
    use rand::{
        distributions::{Alphanumeric, DistString},
        thread_rng,
    };

    use super::Encoder;

    #[test]
    fn b64_validity_check() {
        for _ in 0..100000 {
            let mut rng = thread_rng();
            let string = Alphanumeric.sample_string(&mut rng, 20);
            let b64_encoded = general_purpose::STANDARD.encode(string.clone());
            let encoder = Encoder::new();
            let my_encoded = encoder.encode(string);

            if !my_encoded.eq(&b64_encoded) {
                panic!("{my_encoded:?} != {b64_encoded:?}")
            }
        }
    }

    #[bench]
    fn irelia_encoder(b: &mut Bencher) {
        let mut strings = vec![String::new(); 10000];

        let mut rng = black_box(thread_rng());

        for x in 0..10000 {
            strings[x] = black_box(Alphanumeric.sample_string(&mut rng, 1924));
        }

        let encoder = Encoder::new();

        b.iter(|| {
            black_box({
                for x in 0..10000 {
                    black_box(encoder.encode(&strings[x]));
                }
            });
        })
    }

    #[bench]
    fn irelia_encoder_ascii_check(b: &mut Bencher) {
        let mut strings = vec![String::new(); 10000];

        let mut rng = black_box(thread_rng());

        for x in 0..10000 {
            strings[x] = black_box(Alphanumeric.sample_string(&mut rng, 1924));
        }

        let encoder = Encoder::new();

        b.iter(|| {
            black_box({
                for x in 0..10000 {
                    black_box(encoder.encode_with_ascii_check(&strings[x]));
                }
            });
        })
    }

    #[bench]
    fn irelia_encoder_unchecked(b: &mut Bencher) {
        let mut strings = vec![String::new(); 10000];

        let mut rng = black_box(thread_rng());

        for x in 0..10000 {
            strings[x] = black_box(Alphanumeric.sample_string(&mut rng, 1924));
        }

        let encoder = Encoder::new();

        b.iter(|| {
            black_box({
                for x in 0..10000 {
                    black_box(unsafe { encoder.encode_unchecked(&strings[x]) });
                }
            });
        })
    }

    #[bench]
    fn base64_crate(b: &mut Bencher) {
        let mut strings = vec![String::new(); 10000];

        let mut rng = black_box(thread_rng());

        for x in 0..10000 {
            strings[x] = black_box(Alphanumeric.sample_string(&mut rng, 1924));
        }

        b.iter(|| {
            black_box({
                for x in 0..10000 {
                    general_purpose::STANDARD.encode(&strings[x]);
                }
            });
        })
    }
}
