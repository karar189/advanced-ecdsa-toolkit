
use secp256k1::{Message, Secp256k1, SecretKey};
use sha3::{Digest, Keccak256};
use std::fs::{self, File};
use std::io::{Read, Write};

pub fn sign_file(
    file_path: &str, 
    private_key_path: &str,
    output_path: Option<String>
) -> Result<(), Box<dyn std::error::Error>> {

    let mut file_content = Vec::new();
    let mut file = File::open(file_path)?;
    file.read_to_end(&mut file_content)?;
    

    let mut hasher = Keccak256::new();
    hasher.update(&file_content);
    let hash = hasher.finalize();
    
    let private_key_hex = fs::read_to_string(private_key_path)?;
    let private_key_bytes = hex::decode(private_key_hex.trim())?;
    let secret_key = SecretKey::from_slice(&private_key_bytes)?;
    
    let message = Message::from_digest_slice(hash.as_slice())?;
    
    let secp = Secp256k1::new();
    let signature = secp.sign_ecdsa(&message, &secret_key);

    let signature_bytes = signature.serialize_compact();
    

    let output_file = output_path.unwrap_or_else(|| "ECDSA_Signature".to_string());
    let mut signature_file = File::create(output_file.clone())?;
    signature_file.write_all(&signature_bytes)?;
    
    println!("File signed successfully. Signature saved to: {}", output_file);
    
    Ok(())
}