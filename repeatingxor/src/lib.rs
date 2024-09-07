pub fn repeating_xor(text: &str) -> String {
    let mut vec_string = text.as_bytes().to_vec();
    let key = [73u8, 67u8, 69u8]; // Key: "ICE"

    for (i, byte) in vec_string.iter_mut().enumerate() {
        *byte ^= key[i % key.len()];
    }

    hex::encode(&vec_string)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repeating_xor() {
        assert_eq!(repeating_xor("Burning 'em, if you ain't quick and nimble
I go crazy when I hear a cymbal"), String::from("0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f") )
    }
}
