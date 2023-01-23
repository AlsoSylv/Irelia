trait Alphabet {
    fn get_char_for_index(&self, index: u8) -> Option<char>;
    fn get_index_for_char(&self, character: char) -> Option<u8>;
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

    fn get_index_for_char(&self, character: char) -> Option<u8> {
        let character = character as i8;
        let base64_index = match character {
            65..=90 => character - UPPERCASEOFFSET,  // A-Z
            97..=122 => character - LOWERCASEOFFSET, // a-z
            48..=57 => character - DIGITOFFSET,      // 0-9
            43 => 62,                                // +
            47 => 63,                                // /

            _ => return None,
        } as u8;

        Some(base64_index)
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

fn encode_using_alphabet<T: Alphabet>(alphabet: &T, data: &[u8]) -> String {
    let encoded = data
        .chunks(3)
        .map(split)
        .flat_map(|chunk| encode_chunk(alphabet, chunk));

    String::from_iter(encoded)
}

fn encode_chunk<T: Alphabet>(alphabet: &T, chunk: Vec<u8>) -> Vec<char> {
    let mut out = vec![alphabet.get_padding_char(); 4];

    for i in 0..chunk.len() {
        if let Some(chr) = alphabet.get_char_for_index(chunk[i]) {
            out[i] = chr;
        }
    }

    out
}

pub(crate) fn encode(input: &str) -> String {
    encode_using_alphabet(&Classic, input.as_bytes())
}
