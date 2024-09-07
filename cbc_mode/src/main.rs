// // We are going to implement CBC mode from using ebc mode 
// use aes_ecb::decrypt_aes_128_ecb;
// use reqwest;
// use base64;

// use cbc_mode::custom_cbc_mode;

// #[tokio::main]
// async fn main() {
//     let resp = reqwest::get("https://cryptopals.com/static/challenge-data/7.txt").await.expect("Requsst failed");
//     let body = resp.text().await.expect("Body invalid");

//     let encoded_text_cleaned = body.replace("\n", "");
//     let decoded_bytes = base64::decode(&encoded_text_cleaned).expect("Failed to decode base64");
    
//     let mut decrypted = decrypt_aes_128_ecb(&decoded_bytes, b"YELLOW SUBMARINE");

//     let ciphered_data = custom_cbc_mode(b"YELLOW SUBMARINE", decrypted);
//     println!("{}", ciphered_data.len());
//     println!("{:?}", ciphered_data);
// }

use aes::Aes128;
use aes::cipher::{BlockEncrypt, KeyInit};
use aes::cipher::generic_array::GenericArray;

pub fn ecb_encrypt_block(key: &[u8], block: &mut [u8]) {
    let cipher = Aes128::new(GenericArray::from_slice(key));
    let mut block = GenericArray::clone_from_slice(block);
    cipher.encrypt_block(&mut block);
    block.copy_from_slice(&block);
}

pub fn xor_blocks(block1: &[u8], block2: &[u8], output: &mut [u8]) {
    for (i, (b1, b2)) in block1.iter().zip(block2.iter()).enumerate() {
        output[i] = b1 ^ b2;
    }
}

pub fn cbc_encrypt(key: &[u8], iv: &[u8], plaintext: &[u8]) -> Vec<u8> {
    let block_size = 16; // For AES-128
    let mut ciphertext = Vec::new();
    let mut prev_block = iv.to_vec(); // Start with IV

    for chunk in plaintext.chunks(block_size) {
        let mut block = vec![0u8; block_size];
        
        // XOR the plaintext block with the previous ciphertext block
        xor_blocks(chunk, &prev_block, &mut block);

        // Encrypt the XORed block using ECB
        ecb_encrypt_block(key, &mut block);

        // Append the encrypted block to the ciphertext
        ciphertext.extend_from_slice(&block);

        // Update the previous block to the current ciphertext block
        prev_block = block;
    }

    ciphertext
}

pub fn cbc_decrypt(key: &[u8], iv: &[u8], ciphertext: &[u8]) -> Vec<u8> {
    let block_size = 16; // For AES-128
    let mut plaintext = Vec::new();
    let mut prev_block = iv.to_vec(); // Start with IV

    for chunk in ciphertext.chunks(block_size) {
        let mut block = chunk.to_vec();

        // Decrypt the block using ECB
        ecb_encrypt_block(key, &mut block); // Remember to use the ECB decryption function

        // XOR the decrypted block with the previous ciphertext block
        xor_blocks(&block, &prev_block, &mut block);

        // Append the decrypted block to the plaintext
        plaintext.extend_from_slice(&block);

        // Update the previous block to the current ciphertext block
        prev_block = chunk.to_vec();
    }

    plaintext
}

fn main() {
    let key = b"YELLOW SUBMARINE"; // 16 bytes for AES-128
    let iv = b"1234567890123456"; // 16 bytes IV
    let plaintext = b"Hello, world! This is a test for CBC mode!";

    let ciphertext = cbc_encrypt(key, iv, plaintext);
    println!("Ciphertext: {:?}", ciphertext);

    let decrypted_text = cbc_decrypt(key, iv, &ciphertext);
    println!("Decrypted: {:?}", String::from_utf8_lossy(&decrypted_text));
}


