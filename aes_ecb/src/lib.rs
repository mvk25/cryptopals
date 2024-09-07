use crypto::aes::ecb_decryptor;
use crypto::buffer::{BufferResult, ReadBuffer, WriteBuffer};
use crypto::blockmodes::NoPadding;
use crypto::aes::KeySize::KeySize128;


pub fn decrypt_aes_128_ecb(ciphertext: &[u8], key: &[u8]) -> Vec<u8> {
    let mut decryptor = ecb_decryptor(KeySize128, key, NoPadding);
    
    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = crypto::buffer::RefReadBuffer::new(ciphertext);
    let mut buffer = [0; 4096];
    let mut write_buffer = crypto::buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let result = decryptor.decrypt(&mut read_buffer, &mut write_buffer, true).unwrap();
        final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));

        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => { }
        }
    }

    final_result
}