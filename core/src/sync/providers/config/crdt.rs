use ed25519_dalek::{Keypair, PublicKey, SecretKey};
use rand_core::OsRng;

#[derive(Debug, Default)]
pub struct CrdtConfig {
    pub pk: [u8; 32],
    secret: [u8; 32],
    pub enabled: bool,
}


impl CrdtConfig {
    pub fn new() -> Self {
        let mut csprng = OsRng{};
        let keypair: Keypair = Keypair::generate(&mut csprng);
        Self { pk: keypair.public.to_bytes(),secret: keypair.secret.to_bytes(), enabled: true }
    }

    pub fn default() -> Self {
        CrdtConfig::new()
    }
}