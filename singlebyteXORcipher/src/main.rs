use std::collections::HashMap;

use singlebyteXORcipher::inverse_xor;

fn hex_to_bytes(hex: &str) -> Vec<u8> {
    (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap())
        .collect()
}

fn single_byte_xor(bytes: &[u8], key: u8) -> Vec<u8> {
    bytes.iter().map(|&b| b ^ key).collect()
}

fn score_text(text: &str) -> f64 {
    let english_freq = [
        ('a', 0.0651738), ('b', 0.0124248), ('c', 0.0217339), ('d', 0.0349835), ('e', 0.1041442),
        ('f', 0.0197881), ('g', 0.0158610), ('h', 0.0492888), ('i', 0.0558094), ('j', 0.0009033),
        ('k', 0.0050529), ('l', 0.0331490), ('m', 0.0202124), ('n', 0.0564513), ('o', 0.0596302),
        ('p', 0.0137645), ('q', 0.0008606), ('r', 0.0497563), ('s', 0.0515760), ('t', 0.0729357),
        ('u', 0.0225134), ('v', 0.0082903), ('w', 0.0171272), ('x', 0.0013692), ('y', 0.0145984),
        ('z', 0.0007836), (' ', 0.1918182)
    ].iter().cloned().collect::<HashMap<char, f64>>();

    text.chars().fold(0.0, |acc, c| acc + *english_freq.get(&c.to_ascii_lowercase()).unwrap_or(&0.0))
}

fn decrypt_hex_string(hex_string: &str) -> (u8, String) {
    let bytes = hex_to_bytes(hex_string);
    let mut best_score = 0.0;
    let mut best_key = 0;
    let mut best_message = String::new();

    for key in 0..=255 {
        let decoded_bytes = single_byte_xor(&bytes, key);
        if let Ok(decoded_text) = String::from_utf8(decoded_bytes) {
            let score = score_text(&decoded_text);
            if score > best_score {
                best_score = score;
                best_key = key;
                best_message = decoded_text;
            }
        }
    }

    (best_key, best_message)
}

fn main() {
    let hex_string = "7b5a4215415d544115415d5015455447414c155c46155f4058455c5b523f";
    let (key, decrypted_message) = decrypt_hex_string(hex_string);
    println!("Key: {}", key);
    println!("Decrypted message: {}", decrypted_message);
    // inverse_xor()
}
