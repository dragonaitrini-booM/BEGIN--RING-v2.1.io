// Digital signatures for authentication
use pqc_dilithium::*;

#[derive(Zeroize, ZeroizeOnDrop)]
pub struct DilithiumSigner {
    public_key: [u8; PUBLICKEYBYTES],
    secret_key: [u8; SECRETKEYBYTES],
}

impl DilithiumSigner {
    pub fn new() -> Result<Self, DilithiumError> {
        let mut rng = OsRng;
        let keys = keypair(&mut rng)?;
        
        Ok(Self {
            public_key: *keys.public.as_bytes(),
            secret_key: *keys.secret.as_bytes(),
        })
    }

    pub fn sign(&self, message: &[u8]) -> Result<Vec<u8>, DilithiumError> {
        let mut rng = OsRng;
        let sk = SecretKey::from_bytes(&self.secret_key)?;
        let signature = sign(message, &sk, &mut rng)?;
        Ok(signature.as_bytes().to_vec())
    }

    pub fn verify(&self, message: &[u8], signature: &[u8]) -> Result<bool, DilithiumError> {
        let pk = PublicKey::from_bytes(&self.public_key)?;
        let sig = Signature::from_bytes(signature)?;
        Ok(verify(&sig, message, &pk).is_ok())
    }

    pub fn public_key(&self) -> &[u8] {
        &self.public_key
    }
}// Digital signatures for authentication
use pqc_dilithium::*;

#[derive(Zeroize, ZeroizeOnDrop)]
pub struct DilithiumSigner {
    public_key: [u8; PUBLICKEYBYTES],
    secret_key: [u8; SECRETKEYBYTES],
}

impl DilithiumSigner {
    pub fn new() -> Result<Self, DilithiumError> {
        let mut rng = OsRng;
        let keys = keypair(&mut rng)?;
        
        Ok(Self {
            public_key: *keys.public.as_bytes(),
            secret_key: *keys.secret.as_bytes(),
        })
    }

    pub fn sign(&self, message: &[u8]) -> Result<Vec<u8>, DilithiumError> {
        let mut rng = OsRng;
        let sk = SecretKey::from_bytes(&self.secret_key)?;
        let signature = sign(message, &sk, &mut rng)?;
        Ok(signature.as_bytes().to_vec())
    }

    pub fn verify(&self, message: &[u8], signature: &[u8]) -> Result<bool, DilithiumError> {
        let pk = PublicKey::from_bytes(&self.public_key)?;
        let sig = Signature::from_bytes(signature)?;
        Ok(verify(&sig, message, &pk).is_ok())
    }

    pub fn public_key(&self) -> &[u8] {
        &self.public_key
    }
}
