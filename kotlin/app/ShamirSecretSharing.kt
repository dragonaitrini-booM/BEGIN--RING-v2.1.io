// Advanced authenticated encryption with additional data (AEAD)
use aes_gcm::{Aes256Gcm, Key, Nonce, aead::{Aead, NewAead}};
use chacha20poly1305::ChaCha20Poly1305;

pub struct MultiCipherAEAD {
    aes_cipher: Aes256Gcm,
    chacha_cipher: ChaCha20Poly1305,
    cipher_type: CipherType,
}

#[derive(Clone, Copy)]
pub enum CipherType {
    AES256GCM = 0,
    ChaCha20Poly1305 = 1,
}

impl MultiCipherAEAD {
    pub fn new(key: &[u8; 32], cipher_type: CipherType) -> Self {
        let key = Key::from_slice(key);
        Self {
            aes_cipher: Aes256Gcm::new(key),
            chacha_cipher: ChaCha20Poly1305::new(key),
            cipher_type,
        }
    }

    pub fn encrypt_with_aad(&self, nonce: &[u8; 12], plaintext: &[u8], aad: &[u8]) -> Result<Vec<u8>, String> {
        let nonce = Nonce::from_slice(nonce);
        
        match self.cipher_type {
            CipherType::AES256GCM => {
                self.aes_cipher.encrypt(nonce, aead::Payload { msg: plaintext, aad })
                    .map_err(|e| format!("AES encryption failed: {:?}", e))
            },
            CipherType::ChaCha20Poly1305 => {
                self.chacha_cipher.encrypt(nonce, aead::Payload { msg: plaintext, aad })
                    .map_err(|e| format!("ChaCha20 encryption failed: {:?}", e))
            }
        }
    }
