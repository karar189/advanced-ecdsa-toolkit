
use secp256k1::{Message, PublicKey, Secp256k1, Signature};
use sha3::{Digest, Keccak256};
use std::fs::{self, File};
use std::io::Read;

pub fn verify_file(
    file_path: &str,
    public_key_path: &str,
    signature_path: &str
) -> Result<(), Box<dyn std::error::Error>> {
    let mut file_content = Vec::new();
    let mut file = File::open(file_path)?;
    file.read_to_end(&mut file_content)?;
    
    let mut hasher = Keccak256::new();
    hasher.update(&file_content);
    let hash = hasher.finalize();
    
    let public_key_hex = fs::read_to_string(public_key_path)?;
    let public_key_bytes = hex::decode(public_key_hex.trim())?;
    let public_key = PublicKey::from_slice(&public_key_bytes)?;
    
    let mut signature_bytes = [0u8; 64]; 
    let mut signature_file = File::open(signature_path)?;
    signature_file.read_exact(&mut signature_bytes)?;
    let signature = Signature::from_compact(&signature_bytes)?;
    
    let message = Message::from_digest_slice(hash.as_slice())?;
    
    let secp = Secp256k1::new();
    secp.verify_ecdsa(&message, &signature, &public_key)?;
    
    println!("Signature verification successful!");
    
    Ok(())
}