#![feature(array_chunks, core_intrinsics, int_roundings, test)]
#![no_std]

//! This decoder is largely taking from this article. <https://dev.to/tiemen/implementing-base64-from-scratch-in-rust-kb1>
//! It goes into detail about the entire thing, and why it works the way it does, and I hightly reccomend reading it
//! Very big thanks to Tiemen for writing it!
//!
//! The usage of u64s as byte arrays is taken from the base64 crate, which is under the MIT licsense

extern crate alloc;

use alloc::{string::String, vec};

/// BASE64 encoder struct
pub struct Encoder {
    encode_table: [u8; 64],
}

impl Encoder {
    /// Creates a new instance of the encoder using the default base64 alphabet
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

    pub const fn with_encode_table(encode_table: [u8; 64]) -> Self {
        Self { encode_table }
    }

    #[rustfmt::skip]
    /// Converts the buffer to base64, uses an out paramater to avoid allocations
    fn internal_encode(&self, buf: &[u8], out: &mut [u8]) {
        let chunks = buf.array_chunks::<24>();
        let out_chunks = out.array_chunks_mut::<32>();

        let mut output_index = 0;
        let rem = chunks.remainder();

        chunks.zip(out_chunks).for_each(|(chunk, out)| {
            let byte_array_1 = u64::from_be_bytes([
                chunk[0], chunk[1], chunk[2], chunk[3], chunk[4], chunk[5], 0, 0,
            ]);

            let byte_array_2 = u64::from_be_bytes([
                chunk[6], chunk[7], chunk[8], chunk[9], chunk[10], chunk[11], 0, 0,
            ]);

            let byte_array_3 = u64::from_be_bytes([
                chunk[12], chunk[13], chunk[14], chunk[15], chunk[16], chunk[17], 0, 0,
            ]);

            let byte_array_4 = u64::from_be_bytes([
                chunk[18], chunk[19], chunk[20], chunk[21], chunk[22], chunk[23], 0, 0,
            ]);

            *out = [
                self.encode_table[(byte_array_1 >> 58 & 0b00111111) as usize],
                self.encode_table[(byte_array_1 >> 52 & 0b00111111) as usize],
                self.encode_table[(byte_array_1 >> 46 & 0b00111111) as usize],
                self.encode_table[(byte_array_1 >> 40 & 0b00111111) as usize],
                self.encode_table[(byte_array_1 >> 34 & 0b00111111) as usize],
                self.encode_table[(byte_array_1 >> 28 & 0b00111111) as usize],
                self.encode_table[(byte_array_1 >> 22 & 0b00111111) as usize],
                self.encode_table[(byte_array_1 >> 16 & 0b00111111) as usize],
                self.encode_table[(byte_array_2 >> 58 & 0b00111111) as usize],
                self.encode_table[(byte_array_2 >> 52 & 0b00111111) as usize],
                self.encode_table[(byte_array_2 >> 46 & 0b00111111) as usize],
                self.encode_table[(byte_array_2 >> 40 & 0b00111111) as usize],
                self.encode_table[(byte_array_2 >> 34 & 0b00111111) as usize],
                self.encode_table[(byte_array_2 >> 28 & 0b00111111) as usize],
                self.encode_table[(byte_array_2 >> 22 & 0b00111111) as usize],
                self.encode_table[(byte_array_2 >> 16 & 0b00111111) as usize],
                self.encode_table[(byte_array_3 >> 58 & 0b00111111) as usize],
                self.encode_table[(byte_array_3 >> 52 & 0b00111111) as usize],
                self.encode_table[(byte_array_3 >> 46 & 0b00111111) as usize],
                self.encode_table[(byte_array_3 >> 40 & 0b00111111) as usize],
                self.encode_table[(byte_array_3 >> 34 & 0b00111111) as usize],
                self.encode_table[(byte_array_3 >> 28 & 0b00111111) as usize],
                self.encode_table[(byte_array_3 >> 22 & 0b00111111) as usize],
                self.encode_table[(byte_array_3 >> 16 & 0b00111111) as usize],
                self.encode_table[(byte_array_4 >> 58 & 0b00111111) as usize],
                self.encode_table[(byte_array_4 >> 52 & 0b00111111) as usize],
                self.encode_table[(byte_array_4 >> 46 & 0b00111111) as usize],
                self.encode_table[(byte_array_4 >> 40 & 0b00111111) as usize],
                self.encode_table[(byte_array_4 >> 34 & 0b00111111) as usize],
                self.encode_table[(byte_array_4 >> 28 & 0b00111111) as usize],
                self.encode_table[(byte_array_4 >> 22 & 0b00111111) as usize],
                self.encode_table[(byte_array_4 >> 16 & 0b00111111) as usize],
            ];

            output_index += 32;
        });

        let rem_out = &mut out[output_index..];

        let chunks = rem.array_chunks::<3>();
        let out_chunks = rem_out.array_chunks_mut::<4>();

        let rem = chunks.remainder();

        chunks.zip(out_chunks).for_each(|(chunk, out)| {
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

        match (rem, rem_out) {
            ([], _) => {}
            ([a], [out_a, out_b, _, _]) => {
                let bit_1 = a >> 2;
                let bit_2 = (a & 0b00000011) << 4;

                *out_a = self.encode_table[bit_1 as usize];
                *out_b = self.encode_table[bit_2 as usize];
            }
            ([a, b], [out_a, out_b, out_c, _]) => {
                let byte_array = u16::from_be_bytes([*a, *b]);
                let bit_1 = byte_array >> 10 & 0b00111111;
                let bit_2 = byte_array >> 4 & 0b00111111;
                let bit_3 = byte_array << 2 & 0b00111111;

                *out_a = self.encode_table[bit_1 as usize];
                *out_b = self.encode_table[bit_2 as usize];
                *out_c = self.encode_table[bit_3 as usize];
            }
            _ => unreachable!(),
        }
    }

    /// Converts the bytes to BASE64
    pub fn encode<T>(&self, bytes: T) -> String
    where
        T: AsRef<[u8]>,
    {
        let buf = bytes.as_ref();
        let mut out = vec![b'='; buf.len().div_ceil(3) * 4];
        self.internal_encode(buf, &mut out);

        String::from_utf8(out).unwrap()
    }

    /// Converts the bytes to BASE64
    ///
    /// # Safety
    ///
    /// This does not check that all characters are valid UTF-8
    /// The behavoir of strings containing invalid UTF-8 bytes
    /// is undefined
    pub unsafe fn encode_unchecked<T>(&self, bytes: T) -> String
    where
        T: AsRef<[u8]>,
    {
        let buf = bytes.as_ref();
        let mut out = vec![b'='; buf.len().div_ceil(3) * 4];
        self.internal_encode(buf, &mut out);

        String::from_utf8_unchecked(out)
    }

    /// Converts the bytes to BASE64
    pub fn encode_without_padding<T>(&self, bytes: T) -> String
    where
        T: AsRef<[u8]>,
    {
        let buf = bytes.as_ref();
        let mut out = vec![0; buf.len().div_ceil(3) * 4];
        self.internal_encode(buf, &mut out);

        String::from_utf8(out).unwrap()
    }

    /// Converts the bytes to BASE64
    ///
    /// # Safety
    ///
    /// This does not check that all characters are valid UTF-8
    /// The behavoir of strings containing invalid UTF-8 bytes
    /// is undefined
    pub unsafe fn encode_unchecked_without_padding<T>(&self, bytes: T) -> String
    where
        T: AsRef<[u8]>,
    {
        let buf = bytes.as_ref();
        let mut out = vec![0; buf.len().div_ceil(3) * 4];
        self.internal_encode(buf, &mut out);

        String::from_utf8_unchecked(out)
    }
}

#[cfg(test)]
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
    fn b64() {
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
    fn my_b64(b: &mut Bencher) {
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
    fn real_b64(b: &mut Bencher) {
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
