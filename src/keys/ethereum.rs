use secp256k1::PublicKey;
use sha3::{Digest, Keccak256};

pub fn generate_ethereum_address(public_key: &PublicKey) -> Result<String, Box<dyn std::error::Error>> {
    let public_key_bytes = public_key.serialize_uncompressed();
    
    let key_without_prefix = &public_key_bytes[1..];

    let mut hasher = Keccak256::new();
    hasher.update(key_without_prefix);
    let hash = hasher.finalize();
    
    let address_bytes = &hash[12..32];
    let address_hex = hex::encode(address_bytes);
    
    Ok(format!("0x{}", address_hex))
}