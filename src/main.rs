// src/main.rs
mod cli;
mod core;
mod keys;
mod signing;
mod verification;
mod crypto;
mod utils;
mod errors;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    GenerateKeypair {
        #[arg(long)]
        private_key: Option<String>,
    },
    Sign {
        file_path: String,
        private_key_path: String,
        #[arg(long)]
        output: Option<String>,
    },
    Verify {
        file_path: String,
        public_key_path: String,
        signature_path: String,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::GenerateKeypair { private_key } => {
            keys::generate_keypair(private_key)?;
        }
        Commands::Sign { file_path, private_key_path, output } => {
            signing::sign_file(&file_path, &private_key_path, output)?;
        }
        Commands::Verify { file_path, public_key_path, signature_path } => {
            verification::verify_file(&file_path, &public_key_path, &signature_path)?;
        }
    }
    
    Ok(())
}