use base64::{self, engine::general_purpose, Engine};
use rand::{
    distributions::{Alphanumeric, DistString},
    thread_rng,
};

use crate::utils::encoder::encode;

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
