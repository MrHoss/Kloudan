use sha2::{Digest, Sha256};
use rand::RngCore;
use rand::rngs::OsRng;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write, Seek, SeekFrom};

pub fn hash_password(password: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(password);
    let hash = hasher.finalize();
    let mut result = String::new();
    for byte in hash.iter() {
        result.push_str(&format!("{:02x}", byte));
    }
    result
}
pub fn rand_key() -> String {
    let mut rng = OsRng;
    let mut key = [0u8; 32];
    rng.fill_bytes(&mut key);
    let mut hasher = Sha256::new();
    hasher.update(&key);
    let hash = hasher.finalize();
    let mut result = String::new();
    for byte in hash.iter() {
        result.push_str(&format!("{:02x}", byte));
    }
    result
}
pub fn verify_password(password: &str, hashed_password: &str) -> bool {
    let mut hasher = Sha256::new();
    hasher.update(password);
    let hash = hasher.finalize();
    let mut result = String::new();
    for byte in hash.iter() {
        result.push_str(&format!("{:02x}", byte));
    }
    result == hashed_password
}

pub fn encrypt_file(file_path: &str, password: &str) -> std::io::Result<()> {
    // Ler o arquivo inteiro na memória
    let mut file = File::open(file_path)?;
    let mut file_content = Vec::new();
    file.read_to_end(&mut file_content)?;

    // Gerar uma chave a partir da senha usando SHA256
    let mut key = [0; 32];
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    key.copy_from_slice(&hasher.finalize()[..32]);

    // Criptografar o conteúdo do arquivo
    for i in 0..file_content.len() {
        file_content[i] ^= key[i % 32];
    }

    // Sobrescrever o arquivo com o conteúdo criptografado
    let mut file = OpenOptions::new().write(true).truncate(true).open(file_path)?;
    file.write_all(&file_content)?;

    Ok(())
}


pub fn decrypt_file(file_path: &str, password: &str) -> std::io::Result<()> {
    // Ler o arquivo inteiro na memória
    let mut file = File::open(file_path)?;
    let mut file_content = Vec::new();
    file.read_to_end(&mut file_content)?;

    // Gerar uma chave a partir da senha usando SHA256
    let mut key = [0; 32];
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    key.copy_from_slice(&hasher.finalize()[..32]);

    // Descriptografar o conteúdo do arquivo
    for i in 0..file_content.len() {
        file_content[i] ^= key[i % 32];
    }

    // Sobrescrever o arquivo com o conteúdo descriptografado
    let mut file = OpenOptions::new().write(true).truncate(true).open(file_path)?;
    file.write_all(&file_content)?;

    Ok(())
}



