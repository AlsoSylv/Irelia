//! This decoder is largely taking from this article. <https://dev.to/tiemen/implementing-base64-from-scratch-in-rust-kb1>
//! It goes into detail about the entire thing, and why it works the way it does, and I hightly reccomend reading it
//! Very big thanks to Tiemen for writing it!

const BASE64_ALPHABET: [char; 64] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', '+', '/',
];

/// Fast enough encoder for what is a very small string, used once
pub(super) fn encode(string: String) -> String {
    string
        .as_bytes()
        .chunks(3)
        .flat_map(|chunk| {
            let bit_1 = &chunk[0] >> 2;
            let bit_2 = (&chunk[0] & 0b00000011) << 4;
            let char_1 = BASE64_ALPHABET[bit_1 as usize];
            match chunk.len() {
                1 => [char_1, BASE64_ALPHABET[bit_2 as usize], '=', '='],
                2 => {
                    let bit_2 = bit_2 | &chunk[1] >> 4;
                    let bit_3 = (&chunk[1] & 0b00001111) << 2;
                    [
                        char_1,
                        BASE64_ALPHABET[bit_2 as usize],
                        BASE64_ALPHABET[bit_3 as usize],
                        '=',
                    ]
                }
                3 => {
                    let bit_2 = bit_2 | &chunk[1] >> 4;
                    let bit_3 = (&chunk[1] & 0b00001111) << 2 | &chunk[2] >> 6;
                    let bit_4 = &chunk[2] & 0b00111111;
                    [
                        char_1,
                        BASE64_ALPHABET[bit_2 as usize],
                        BASE64_ALPHABET[bit_3 as usize],
                        BASE64_ALPHABET[bit_4 as usize],
                    ]
                }
                _ => unreachable!(),
            }
        })
        .collect()
}

#[cfg(test)]
mod test {
    use base64::{self, engine::general_purpose, Engine};
    use rand::{
        distributions::{Alphanumeric, DistString},
        thread_rng,
    };

    use super::encode;

    #[test]
    fn b64() {
        for _ in 0..100000 {
            let mut rng = thread_rng();
            let string = Alphanumeric.sample_string(&mut rng, 20);
            let b64_encoded = general_purpose::STANDARD.encode(string.clone());
            let my_encoded = encode(string);

            if !my_encoded.eq(&b64_encoded) {
                panic!("{my_encoded:?} != {b64_encoded:?}")
            }
        }
    }
}
