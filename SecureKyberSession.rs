// Cargo.toml dependencies
/*
[dependencies]
pqc-kyber = "0.7.1"
zeroize = { version = "1.7", features = ["zeroize_derive"] }
rand_core = "0.6"
aes-gcm = "0.10"
hkdf = "0.12"
sha2 = "0.10"
*/

use pqc_kyber::*;
use zeroize::{Zeroize, ZeroizeOnDrop};
use rand_core::OsRng;
use aes_gcm::{Aes256Gcm, Key, Nonce, aead::Aead};
use hkdf::Hkdf;
use sha2::Sha256;

/// Secure Kyber-512 session with automatic memory cleanup
#[derive(Zeroize, ZeroizeOnDrop)]
pub struct SecureKyberSession {
    #[zeroize(skip)] // Public key doesn't need zeroization
    public_key: [u8; KYBER_PUBLICKEYBYTES],
    secret_key: [u8; KYBER_SECRETKEYBYTES],
    shared_secret: Option<[u8; KYBER_SSBYTES]>,
}

impl SecureKyberSession {
    /// Generate new keypair for client
    pub fn new_client() -> Result<Self, KyberError> {
        let mut rng = OsRng;
        let keys = keypair(&mut rng)?;
        
        Ok(Self {
            public_key: *keys.public.as_bytes(),
            secret_key: *keys.secret.as_bytes(),
            shared_secret: None,
        })
    }

    /// Get public key for transmission to server
    pub fn public_key(&self) -> &[u8; KYBER_PUBLICKEYBYTES] {
        &self.public_key
    }

    /// Server: encapsulate shared secret using client's public key
    pub fn server_encapsulate(client_pk: &[u8; KYBER_PUBLICKEYBYTES]) 
        -> Result<(Vec<u8>, [u8; KYBER_SSBYTES]), KyberError> {
        let mut rng = OsRng;
        let pk = PublicKey::from_bytes(client_pk)?;
        let (ciphertext, shared_secret) = encapsulate(&pk, &mut rng)?;
        
        Ok((ciphertext.as_bytes().to_vec(), *shared_secret.as_bytes()))
    }

    /// Client: decapsulate shared secret from server's ciphertext
    pub fn client_decapsulate(&mut self, ciphertext: &[u8]) 
        -> Result<(), KyberError> {
        let ct = Ciphertext::from_bytes(ciphertext)?;
        let sk = SecretKey::from_bytes(&self.secret_key)?;
        let shared_secret = decapsulate(&ct, &sk)?;
        
        self.shared_secret = Some(*shared_secret.as_bytes());
        Ok(())
    }

    /// Derive AES-256-GCM key from shared secret using HKDF
    pub fn derive_session_key(&self, info: &[u8]) -> Result<[u8; 32], &'static str> {
        let shared_secret = self.shared_secret
            .ok_or("No shared secret available")?;
            
        let hk = Hkdf::<Sha256>::new(None, &shared_secret);
        let mut okm = [0u8; 32];
        hk.expand(info, &mut okm)
            .map_err(|_| "HKDF expansion failed")?;
            
        Ok(okm)
    }

    /// Create authenticated encryption cipher
    pub fn create_cipher(&self, info: &[u8]) -> Result<Aes256Gcm, Box<dyn std::error::Error>> {
        let key_bytes = self.derive_session_key(info)?;
        let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
        Ok(Aes256Gcm::new(key))
    }
}

/// Secure message encryption/decryption with Kyber-derived keys
pub struct SecureMessaging {
    cipher: Aes256Gcm,
    nonce_counter: u64,
}

impl SecureMessaging {
    pub fn new(session: &SecureKyberSession, context: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        let cipher = session.create_cipher(context)?;
        Ok(Self {
            cipher,
            nonce_counter: 0,
        })
    }

    /// Encrypt message with automatic nonce management
    pub fn encrypt(&mut self, plaintext: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        self.nonce_counter += 1;
        let mut nonce_bytes = [0u8; 12];
        nonce_bytes[4..].copy_from_slice(&self.nonce_counter.to_be_bytes());
        let nonce = Nonce::from_slice(&nonce_bytes);
        
        let mut ciphertext = self.cipher.encrypt(nonce, plaintext)?;
        
        // Prepend nonce to ciphertext
        let mut result = Vec::with_capacity(12 + ciphertext.len());
        result.extend_from_slice(&nonce_bytes);
        result.append(&mut ciphertext);
        
        Ok(result)
    }

    /// Decrypt message with nonce extraction
    pub fn decrypt(&self, ciphertext_with_nonce: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        if ciphertext_with_nonce.len() < 12 {
            return Err("Ciphertext too short".into());
        }
        
        let (nonce_bytes, ciphertext) = ciphertext_with_nonce.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);
        
        let plaintext = self.cipher.decrypt(nonce, ciphertext)?;
        Ok(plaintext)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kyber_key_exchange() {
        // Client generates keypair
        let mut client = SecureKyberSession::new_client().unwrap();
        
        // Server encapsulates shared secret
        let (ciphertext, server_secret) = 
            SecureKyberSession::server_encapsulate(client.public_key()).unwrap();
        
        // Client decapsulates
        client.client_decapsulate(&ciphertext).unwrap();
        
        // Both derive same session key
        let info = b"test-session-key";
        let client_key = client.derive_session_key(info).unwrap();
        
        // Verify server can derive same key
        let mut server_session = SecureKyberSession {
            public_key: [0u8; KYBER_PUBLICKEYBYTES],
            secret_key: [0u8; KYBER_SECRETKEYBYTES],
            shared_secret: Some(server_secret),
        };
        let server_key = server_session.derive_session_key(info).unwrap();
        
        assert_eq!(client_key, server_key);
    }

    #[test]
    fn test_secure_messaging() {
        let mut client = SecureKyberSession::new_client().unwrap();
        let (ciphertext, server_secret) = 
            SecureKyberSession::server_encapsulate(client.public_key()).unwrap();
        client.client_decapsulate(&ciphertext).unwrap();

        let mut messaging = SecureMessaging::new(&client, b"test-context").unwrap();
        
        let message = b"Hello, quantum-secure world!";
        let encrypted = messaging.encrypt(message).unwrap();
        let decrypted = messaging.decrypt(&encrypted).unwrap();
        
        assert_eq!(message.to_vec(), decrypted);
    }
}
