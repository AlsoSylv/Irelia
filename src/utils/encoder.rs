// This decoder is largely taking from this article. https://dev.to/tiemen/implementing-base64-from-scratch-in-rust-kb1
// It goes into detail about the entire thing, and why it works the way it does, and I hightly reccomend reading it
// Very big thanks to Tiemen for writing it!

const UPPERCASEOFFSET: u8 = 65;
const LOWERCASEOFFSET: u8 = 71;
const DIGITOFFSET: u8 = 4;

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

fn encode_chunk(chunk: Vec<u8>) -> [char; 4] {
    let mut out = ['=', '=', '=', '='];

    chunk.iter().enumerate().for_each(|(i, index)| {
        out[i] = match index {
            0..=25 => (*index + UPPERCASEOFFSET) as char,
            26..=51 => (*index + LOWERCASEOFFSET) as char,
            52..=61 => (*index - DIGITOFFSET) as char,
            62 => 43 as char,
            63 => 47 as char,

            _ => unreachable!(),
        };
    });

    out
}

pub(crate) fn encode(input: String) -> String {
    String::from_iter(input.as_bytes().chunks(3).map(split).flat_map(encode_chunk))
}
