use crate::random::Random;
use aes::cipher::{BlockDecryptMut, BlockEncryptMut, KeyInit, block_padding::Pkcs7};
use std::error::Error;

type Aes128EcbEnc = ecb::Encryptor<aes::Aes128>;
type Aes128EcbDec = ecb::Decryptor<aes::Aes128>;

const KEY_SIZE: usize = 16;
const DEFAULT_SEED: u32 = 7;

#[derive(Clone)]
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
            // generate default key [0, 1, 2, ..., 15]
            for (i, byte) in key.iter_mut().enumerate() {
                *byte = i as u8
            }
        } else {
            let mut rng = Random::new(DEFAULT_SEED);
            let new_seed = rng.next().wrapping_add(seed);
            rng.set_seed(new_seed);

            for (_, byte) in key.iter_mut().enumerate() {
                *byte = rng.next() as u8
            }
        }

        // println!("new key: {:?}", Crypto::byte_array_to_string(&key));
        println!("new key: {:?}", (&key));

        self.key = key;
    }

    fn encrypt_string_to_bytes(
        plain_text: String,
        key: [u8; KEY_SIZE],
    ) -> Result<Vec<u8>, Box<dyn Error>> {
        if plain_text.is_empty() {
            return Err("plain_text cannot be empty".into());
        }

        let res =
            Aes128EcbEnc::new(&key.into()).encrypt_padded_vec_mut::<Pkcs7>(plain_text.as_bytes());

        Ok(res)
    }

    pub fn encrypt(&self, plain_text: String) -> Result<Vec<u8>, Box<dyn Error>> {
        Self::encrypt_string_to_bytes(plain_text, self.key)
    }

    fn decrypt_string_from_bytes(
        cipher_text: Vec<u8>,
        key: [u8; KEY_SIZE],
    ) -> Result<String, Box<dyn Error>> {
        if cipher_text.is_empty() {
            return Err("cipher_text cannot be empty".into());
        }

        let res = Aes128EcbDec::new(&key.into())
            .decrypt_padded_vec_mut::<Pkcs7>(&cipher_text)
            .unwrap();

        String::from_utf8(res).map_err(|e| e.into())
    }

    pub fn decrypt(&self, cipher_text: Vec<u8>) -> Result<String, Box<dyn Error>> {
        Self::decrypt_string_from_bytes(cipher_text, self.key)
    }

    pub fn bytes_to_hex(buf: &[u8]) -> String {
        let mut string = String::with_capacity(buf.len() * 2);
        for &byte in buf {
            string.push_str(&format!("{:02x}", byte));
        }
        string
    }

    pub fn hex_to_bytes(str: &str) -> Result<Vec<u8>, Box<dyn Error>> {
        if str.len() % 2 != 0 {
            return Err("Hex string must have even length".into());
        }
        let mut vec = Vec::with_capacity(str.len() / 2);
        let bytes = str.as_bytes();
        for i in (0..str.len()).step_by(2) {
            let hex_str = std::str::from_utf8(&bytes[i..i + 2])?;
            let byte = u8::from_str_radix(hex_str, 16)?;
            vec.push(byte);
        }
        Ok(vec)
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
