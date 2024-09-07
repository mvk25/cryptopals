use serde::{Deserialize, Serialize};
use serde_json;
use hex;
use base64;

#[derive(Serialize, Deserialize, Debug)]
struct HexData {
    hex: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Base64Data {
    base64: String,
}

fn hex_to_base64(hex_str: &str) -> Result<String, String> {
    // Decode the hex string to bytes
    let bytes = hex::decode(hex_str).map_err(|e| format!("Invalid hex string: {}", e))?;
    // Encode the bytes to a base64 string
    let base64_str = base64::encode(bytes);
    
    Ok(base64_str)
}

fn main() {
    // Example hex string
    let hex_data = HexData {
        hex: String::from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"), // "Hello World" in hex
    };

    println!("{:?}", hex_data);
    // Convert HexData to JSON string
    let hex_json = serde_json::to_string(&hex_data).unwrap();
    println!("Hex JSON: {}", hex_json);

    // Parse the JSON string back to HexData struct
    let parsed_hex_data: HexData = serde_json::from_str(&hex_json).unwrap();
    println!("Parsed Hex Data: {:?}", parsed_hex_data);

    // Convert the hex string to a base64 string
    match hex_to_base64(&parsed_hex_data.hex) {
        Ok(base64_str) => {
            let base64_data = Base64Data { base64: base64_str };
            // Convert Base64Data to JSON string
            let base64_json = serde_json::to_string(&base64_data).unwrap();
            println!("Base64 JSON: {}", base64_json);

            // Parse the JSON string back to Base64Data struct
            let parsed_base64_data: Base64Data = serde_json::from_str(&base64_json).unwrap();
            println!("Parsed Base64 Data: {:?}", parsed_base64_data);
        }
        Err(e) => println!("Error: {}", e),
    }
}
