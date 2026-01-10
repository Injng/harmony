use aes_gcm::{
    Aes256Gcm,
    aead::{Aead, AeadCore, KeyInit, OsRng},
};
use anyhow::{Result, anyhow};
use base64::{Engine as _, engine::general_purpose};
use sha2::{Digest, Sha256, digest::generic_array::GenericArray};

pub fn auth_check_and_decode_hex(password: &str) -> Result<String> {
    if &password[0..4] == "enc:" {
        if let Ok(v) = hex::decode(&password[4..]) {
            if let Ok(p) = String::from_utf8(v) {
                return Ok(p);
            } else {
                return Err(anyhow!("[ERROR] Invalid hex-encoded password"));
            }
        }
    }
    return Ok(password.to_owned());
}

fn auth_key_from_string(key_string: &str) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(key_string.as_bytes());
    hasher.finalize().into()
}

pub fn auth_encrypt(password: &str, key_str: &str) -> Result<(String, String)> {
    // derive the proper key from the key string
    let key_bytes = auth_key_from_string(key_str);
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

pub fn auth_decrypt(enc_password: &str, key_str: &str, nonce_str: &str) -> String {
    // decode all of the inputs into bytes and create the cipher
    let nonce_vec = &general_purpose::STANDARD.decode(&nonce_str).unwrap();
    let nonce = GenericArray::from_slice(&nonce_vec);
    let ciphertext = general_purpose::STANDARD.decode(&enc_password).unwrap();
    let key_bytes = auth_key_from_string(key_str);
    let cipher = Aes256Gcm::new(&key_bytes.into());

    // decrypt the password
    String::from_utf8(cipher.decrypt(nonce, ciphertext.as_slice()).unwrap()).unwrap()
}

pub fn auth_verify(
    encoded_password: &str,
    token: &str,
    salt: &str,
    key: &str,
    nonce: &str,
) -> bool {
    let password = auth_decrypt(encoded_password, key, nonce);
    let digest = md5::compute(format!("{}{}", password, salt).as_bytes());
    format!("{:x}", digest) == token
}
