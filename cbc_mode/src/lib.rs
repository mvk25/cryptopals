use crypto::aes::KeySize;
use crypto::blockmodes::NoPadding;
use crypto::buffer::{RefReadBuffer, RefWriteBuffer, BufferResult};
use crypto::symmetriccipher::BlockEncryptor;

pub fn custom_cbc_mode(key: &[u8], plaintext: Vec<u8>) -> Vec<u8> {
    let chunk_size = 16;
    let mut cipher_data = Vec::with_capacity(plaintext.len());
    let mut iv = vec![0u8; chunk_size];
    let mut encryptor = crypto::aes::ecb_encryptor(
        KeySize::KeySize128,
        key,
        NoPadding
    );

    for chunk in plaintext.chunks(chunk_size) {
        let mut block = [0u8; 16];
        block[..chunk.len()].copy_from_slice(chunk);

        // XOR with IV (or previous ciphertext block)
        for (b, &iv_byte) in block.iter_mut().zip(iv.iter()) {
            *b ^= iv_byte;
        }

        let mut read_buffer = RefReadBuffer::new(&block);
        let mut buffer = [0; 16];
        let mut write_buffer = RefWriteBuffer::new(&mut buffer);

        encryptor.encrypt(&mut read_buffer, &mut write_buffer, true).unwrap();

        cipher_data.extend_from_slice(&buffer);
        iv.copy_from_slice(&buffer);
    }

    cipher_data
}