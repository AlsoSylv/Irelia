// This decoder is largely taking from this article. https://dev.to/tiemen/implementing-base64-from-scratch-in-rust-kb1
// It goes into detail about the entire thing, and why it works the way it does, and I hightly reccomend reading it
// Very big thanks to Tiemen for writing it!

trait Alphabet {
    fn get_char_for_index(&self, index: u8) -> Option<char>;
    fn get_padding_char(&self) -> char;
}

struct Classic;

const UPPERCASEOFFSET: i8 = 65;
const LOWERCASEOFFSET: i8 = 71;
const DIGITOFFSET: i8 = -4;

impl Alphabet for Classic {
    fn get_char_for_index(&self, index: u8) -> Option<char> {
        let index = index as i8;

        let ascii_index = match index {
            0..=25 => index + UPPERCASEOFFSET,  // A-Z
            26..=51 => index + LOWERCASEOFFSET, // a-z
            52..=61 => index + DIGITOFFSET,     // 0-9
            62 => 43,                           // +
            63 => 47,                           // /

            _ => return None,
        } as u8;

        Some(ascii_index as char)
    }

    fn get_padding_char(&self) -> char {
        '='
    }
}

fn split(chunk: &[u8]) -> Vec<u8> {
    match chunk.len() {
        1 => vec![&chunk[0] >> 2, (&chunk[0] & 0b00000011) << 4],

        2 => vec![
            &chunk[0] >> 2,
            (&chunk[0] & 0b00000011) << 4 | &chunk[1] >> 4,
            (&chunk[1] & 0b00001111) << 2,
        ],

        3 => vec![
            &chunk[0] >> 2,
            (&chunk[0] & 0b00000011) << 4 | &chunk[1] >> 4,
            (&chunk[1] & 0b00001111) << 2 | &chunk[2] >> 6,
            &chunk[2] & 0b00111111,
        ],

        _ => unreachable!(),
    }
}

fn encode_chunk(chunk: Vec<u8>) -> Vec<char> {
    let mut out = vec![Classic.get_padding_char(); 4];

    for i in 0..chunk.len() {
        if let Some(chr) = Classic.get_char_for_index(chunk[i]) {
            out[i] = chr;
        }
    }

    out
}

pub(super) fn encode(input: String) -> String {
    let encoded = input
        .as_bytes()
        .chunks(3)
        .map(split)
        .flat_map(encode_chunk);

    String::from_iter(encoded)
}
