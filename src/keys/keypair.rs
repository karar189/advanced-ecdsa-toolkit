
use rand::rngs::OsRng;
use secp256k1::{PublicKey, SecretKey, Secp256k1};
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn generate_keypair(private_key_hex: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    let secp = Secp256k1::new();
    
    let secret_key = match private_key_hex {
        Some(hex_key) => {
            let key_bytes = hex::decode(&hex_key)?;
            SecretKey::from_slice(&key_bytes)?
        },
        None => {
            let mut rng = OsRng::default();
            SecretKey::new(&mut rng)
        }
    };
    
    let public_key = PublicKey::from_secret_key(&secp, &secret_key);
    
    let private_key_hex = hex::encode(secret_key.secret_bytes());
    let mut private_key_file = File::create("private_key.hex")?;
    private_key_file.write_all(private_key_hex.as_bytes())?;
    
    let public_key_hex = hex::encode(public_key.serialize_uncompressed());
    let mut public_key_file = File::create("public_key.hex")?;
    public_key_file.write_all(public_key_hex.as_bytes())?;

    let ethereum_address = crate::keys::ethereum::generate_ethereum_address(&public_key)?;
    println!("Successfully generated keypair:");
    println!("Private key: {}", private_key_hex);
    println!("Public key: {}", public_key_hex);
    println!("Ethereum address: {}", ethereum_address);
    
    Ok(())
}