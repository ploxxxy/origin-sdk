use crate::random::Random;
use aes::cipher::{
    BlockDecryptMut, BlockEncryptMut, KeyInit,
    block_padding::{Pkcs7, UnpadError},
};
use std::{error::Error, fmt};

type EcbEncryptor = ecb::Encryptor<aes::Aes128>;
type EcbDecryptor = ecb::Decryptor<aes::Aes128>;

const KEY_SIZE: usize = 16;
const DEFAULT_SEED: u32 = 7;

#[derive(Debug)]
pub enum CryptoError {
    EmptyInput,
    DecryptionFailed(String),
    Utf8Error(std::string::FromUtf8Error),
}

impl fmt::Display for CryptoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CryptoError::EmptyInput => write!(f, "Input cannot be empty"),
            CryptoError::DecryptionFailed(msg) => write!(f, "Decryption failed: {}", msg),
            CryptoError::Utf8Error(err) => write!(f, "UTF-8 conversion error: {}", err),
        }
    }
}

impl Error for CryptoError {}

impl From<std::string::FromUtf8Error> for CryptoError {
    fn from(err: std::string::FromUtf8Error) -> Self {
        CryptoError::Utf8Error(err)
    }
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

        self.key = key;
    }

    pub fn encrypt(&self, plain_text: String) -> Result<Vec<u8>, CryptoError> {
        if plain_text.is_empty() {
            return Err(CryptoError::EmptyInput);
        }

        let res = EcbEncryptor::new(&self.key.into())
            .encrypt_padded_vec_mut::<Pkcs7>(plain_text.as_bytes());

        Ok(res)
    }

    pub fn decrypt(&self, cipher_text: Vec<u8>) -> Result<String, CryptoError> {
        if cipher_text.is_empty() {
            return Err(CryptoError::EmptyInput);
        }

        let res = EcbDecryptor::new(&self.key.into())
            .decrypt_padded_vec_mut::<Pkcs7>(&cipher_text)
            .map_err(|e: UnpadError| CryptoError::DecryptionFailed(e.to_string()))?;

        String::from_utf8(res).map_err(CryptoError::from)
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
            [
                251, 135, 22, 197, 214, 181, 148, 115, 149, 93, 40, 78, 123, 141, 60, 108
            ]
        );
    }

    #[test]
    fn test_encrypt_decrypt() {
        let crypto = Crypto::new(1337);

        let plain_text = "hello world".to_string();
        let cipher_text = crypto
            .encrypt(plain_text.clone())
            .expect("Failed to encrypt");

        let decrypted_text = crypto.decrypt(cipher_text).expect("Failed to decrypt");

        assert_eq!(plain_text, decrypted_text);
    }
}
