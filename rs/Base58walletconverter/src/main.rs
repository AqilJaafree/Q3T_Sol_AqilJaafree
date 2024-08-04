use std::io::{self, BufRead};
use bs58;

fn main() {
    println!("Welcome to the Base58 Wallet Converter!");
    println!("Choose an option:");
    println!("1. Convert base58 to wallet");
    println!("2. Convert wallet to base58");
    println!("Enter your choice:");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();

    match choice.trim() {
        "1" => base58_to_wallet(),
        "2" => wallet_to_base58(),
        _ => println!("Invalid choice"),
    }
}

fn base58_to_wallet() {
    println!("Enter your base58 string:");
    let stdin = io::stdin();
    let base58 = stdin.lock().lines().next().unwrap().unwrap();

    match bs58::decode(&base58).into_vec() {
        Ok(wallet) => println!("Decoded wallet (byte array): {:?}", wallet),
        Err(e) => println!("Error decoding base58: {}", e),
    }
}

fn wallet_to_base58() {
    let wallet: Vec<u8> = vec![];
    let base58 = bs58::encode(wallet).into_string();
    println!("Encoded base58 string: {}", base58);
}
