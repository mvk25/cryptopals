use num_bigint::BigInt;
use std::str;

pub fn inverse_xor() {
    let all_chars: Vec<u8> = (0..255 as u32)
        .filter_map(std::char::from_u32)
        .map(|c| c as u8)
        .collect();
    println!("{:?}", all_chars);

    let hex_string = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let hex_bigint = BigInt::parse_bytes(hex_string.as_bytes(), 16).unwrap();
    println!("{:?}", str::from_utf8(&hex_bigint.clone().to_bytes_be().1));

    for char_byte in all_chars {
        let key_bytes = vec![char_byte; hex_string.len() / 2]; // Create a key with the same length as the hex_string bytes
        let key_bigint = BigInt::from_bytes_be(num_bigint::Sign::Plus, &key_bytes);
        let word_bytes = hex_bigint.clone() ^ key_bigint;

        let result_bytes = word_bytes.to_bytes_be().1;
        match str::from_utf8(&result_bytes) {
            Ok(result_str) => println!("Key '{}': {}", char_byte as char, result_str),
            Err(e) => println!("Key '{}': [Invalid UTF-8]", char_byte as char),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inverse_xor() {
        todo!();
    }
    
}
