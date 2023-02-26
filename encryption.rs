use aes::Aes128;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;

type Aes128Cbc = Cbc<Aes128, Pkcs7>;

const BLOCK_SIZE: usize = 16;

fn encrypt_file(file_path: &str, key: &[u8]) -> Result<(), String> {
    // Open the input file for reading
    let mut input_file = match File::open(file_path) {
        Ok(file) => file,
        Err(err) => return Err(format!("Error opening input file: {}", err)),
    };
    
    // Create the output file for writing
    let output_file_path = format!("{}.enc", file_path);
    let mut output_file = match OpenOptions::new().write(true).create_new(true).open(&output_file_path) {
        Ok(file) => file,
        Err(err) => return Err(format!("Error creating output file: {}", err)),
    };
    
    // Initialize the block cipher
    let cipher = Aes128Cbc::new_var(key, &[0; BLOCK_SIZE]).unwrap();
    
    // Read the input file in blocks and encrypt each block
    let mut buffer = [0; BLOCK_SIZE];
    let mut iv = [0; BLOCK_SIZE];
    while let Ok(n) = input_file.read(&mut buffer) {
        if n == 0 {
            break;
        }
        if n < BLOCK_SIZE {
            return Err("Input file is not a multiple of the block size".to_owned());
        }
        
        // Encrypt the block with CBC mode
        let ciphertext = cipher.encrypt_iv(&mut iv, &buffer);
        
        // Write the encrypted block to the output file
        output_file.write_all(&ciphertext).unwrap();
    }
    
    Ok(())
}

fn generate_key() -> [u8; BLOCK_SIZE] {
    let mut key = [0; BLOCK_SIZE];
    let mut rng = rand::thread_rng();
    rng.fill_bytes(&mut key);
    key
}

fn main() -> Result<(), String> {
    // Parse the command-line argument
    let file_path = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Usage: {} <file>", std::env::args().next().unwrap());
        std::process::exit(1);
    });
    
    // Generate the encryption key
    let key = generate_key();
    
    // Encrypt the file
    encrypt_file(&file_path, &key)?;
    
    println!("File encrypted with key: {:?}", key);
    Ok(())
}
