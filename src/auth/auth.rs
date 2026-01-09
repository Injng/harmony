use aes_gcm::{
    Aes256Gcm,
    aead::{Aead, AeadCore, KeyInit, OsRng},
};
use anyhow::{Result, anyhow};
use base64::{Engine as _, engine::general_purpose};
use sha2::{Digest, Sha256};

fn auth_key_from_string(key_string: &str) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(key_string.as_bytes());
    hasher.finalize().into()
}

pub fn auth_encrypt(password: &str, key_string: &str) -> Result<(String, String)> {
    // derive the proper key from the key string
    let key_bytes = auth_key_from_string(key_string);
    let cipher = Aes256Gcm::new(&key_bytes.into());

    // encrypt the plaintext password with the key and a randomly generated nonce
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let ciphertext = cipher
        .encrypt(&nonce, password.as_bytes().as_ref())
        .map_err(|_| anyhow!("[ERROR] AEAD encryption of password failed"))?;

    // convert to base64 strings for database storage
    let encrypted_password = general_purpose::STANDARD.encode(&ciphertext);
    let nonce_string = general_purpose::STANDARD.encode(&nonce);
    Ok((encrypted_password, nonce_string))
}
