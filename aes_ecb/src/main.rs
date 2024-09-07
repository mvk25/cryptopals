use reqwest;
use base64;

use aes_ecb::decrypt_aes_128_ecb;

#[tokio::main]
async fn main() {
    let resp = reqwest::get("https://cryptopals.com/static/challenge-data/7.txt").await.expect("Request failed");
    let body = resp.text().await.expect("Body invalid");
    println!("{}", body.len());
    let encoded_text_cleaned = body.replace("\n", "");
    println!("{}", encoded_text_cleaned.len());
    match base64::decode(&encoded_text_cleaned) {
        Ok(decoded_bytes) => {
            let key = String::from("YELLOW SUBMARINE");
            let decrypted = decrypt_aes_128_ecb(&decoded_bytes, key.as_ref());
            println!("{}", decrypted.len());
            println!("Decrypted (bytes): {}", String::from_utf8_lossy(decrypted.as_ref()));
        }
        Err(e) => println!("Failed to decode base64: {}", e),
    }

}