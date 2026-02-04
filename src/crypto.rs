use crate::random::Random;
use aes::cipher::{
    block_padding::{Pkcs7, UnpadError},
    BlockDecryptMut, BlockEncryptMut, KeyInit,
};
use thiserror::Error;
use tracing::debug;

type EcbEncryptor = ecb::Encryptor<aes::Aes128>;
type EcbDecryptor = ecb::Decryptor<aes::Aes128>;

const KEY_SIZE: usize = 16;
const DEFAULT_SEED: u32 = 7;

#[derive(Debug, Error)]
pub enum CryptoError {
    #[error("Input cannot be empty")]
    EmptyInput,

    #[error("Decryption failed: {0}")]
    DecryptionFailed(String),

    #[error("UTF-8 conversion error")]
    Utf8Error(#[from] std::string::FromUtf8Error),
}
#[derive(Clone, Debug)]
pub struct Crypto {
    key: [u8; KEY_SIZE],
}

impl Crypto {
    pub fn new(seed: u32) -> Self {
        let mut crypto = Self {
            key: [0u8; KEY_SIZE],
        };

        crypto.set_key(seed);
        crypto
    }

    pub fn set_key(&mut self, seed: u32) {
        let mut key = [0u8; KEY_SIZE];

        if seed == 0 {
            for (i, byte) in key.iter_mut().enumerate() {
                *byte = i as u8
            }
        } else {
            let mut rng = Random::new(DEFAULT_SEED);
            let new_seed = rng.next().wrapping_add(seed);
            rng.set_seed(new_seed);

            for byte in key.iter_mut() {
                *byte = rng.next() as u8
            }
        }

        debug!("Setting new encryption key: {:?}", hex::encode(key));
        self.key = key;
    }

    pub fn encrypt(&self, plain_text: &str) -> Result<Vec<u8>, CryptoError> {
        if plain_text.is_empty() {
            return Err(CryptoError::EmptyInput);
        }

        let res = EcbEncryptor::new(&self.key.into())
            .encrypt_padded_vec_mut::<Pkcs7>(plain_text.as_bytes());

        Ok(res)
    }

    pub fn decrypt(&self, cipher_text: &[u8]) -> Result<String, CryptoError> {
        if cipher_text.is_empty() {
            return Err(CryptoError::EmptyInput);
        }

        let res = EcbDecryptor::new(&self.key.into())
            .decrypt_padded_vec_mut::<Pkcs7>(cipher_text)
            .map_err(|e: UnpadError| CryptoError::DecryptionFailed(e.to_string()))?;

        String::from_utf8(res).map_err(CryptoError::from)
    }

    pub fn prepare_challenge_response(&mut self, key: &str) -> Result<String, CryptoError> {
        let response_key = self.encrypt(key)?;
        let response_str = hex::encode(&response_key);
        let response_bytes = response_str.as_bytes();

        let seed = ((response_bytes[0] as u32) << 8) | (response_bytes[1] as u32);
        self.set_key(seed);

        Ok(response_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key() {
        let crypto = Crypto::new(0);

        assert_eq!(crypto.key.len(), KEY_SIZE);
        assert_eq!(
            &crypto.key as &[u8],
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]
        );

        let crypto = Crypto::new(1337);

        assert_eq!(crypto.key.len(), KEY_SIZE);
        assert_eq!(
            &crypto.key as &[u8],
            [251, 135, 22, 197, 214, 181, 148, 115, 149, 93, 40, 78, 123, 141, 60, 108]
        );
    }

    #[test]
    fn test_encrypt_decrypt() {
        let crypto = Crypto::new(1337);

        let plain_text = "hello world".to_string();
        let cipher_text = crypto.encrypt(&plain_text).expect("Failed to encrypt");
        let decrypted_text = crypto.decrypt(&cipher_text).expect("Failed to decrypt");

        assert_eq!(plain_text, decrypted_text);
    }
}
