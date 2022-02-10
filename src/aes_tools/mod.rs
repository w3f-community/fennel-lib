use openssl::aes::{aes_ige, AesKey};
use openssl::rand::rand_bytes;
use openssl::symm::Mode;

#[cfg(test)]
mod bench;
#[cfg(test)]
mod tests;

mod padding;

use padding::pad;

struct AESCipher {
    encrypt_key: AesKey,
    decrypt_key: AesKey,
    iv: Vec<u8>,
}

impl AESCipher {
    fn new() -> AESCipher {
        let keys = generate_keys();

        AESCipher {
            encrypt_key: keys.0,
            decrypt_key: keys.1,
            iv: generate_buffer(32),
        }
    }
}

trait Cipher {
    fn encrypt<T: AsRef<str>>(&self, plaintext: T) -> Vec<u8>;
    fn decrypt(&self, ciphertext: Vec<u8>) -> String;
}

impl Cipher for AESCipher {
    fn encrypt<T: AsRef<str>>(&self, plaintext: T) -> Vec<u8> {
        aes_encrypt(&self.encrypt_key, self.iv.clone(), plaintext)
    }

    fn decrypt(&self, ciphertext: Vec<u8>) -> String {
        aes_decrypt(&self.decrypt_key, self.iv.clone(), ciphertext)
    }
}

pub fn generate_keys() -> (AesKey, AesKey) {
    let buf = generate_buffer(16); // 128, 192, 256 bits or 16, 24, 32 bytes
    let e_aeskey = AesKey::new_encrypt(&buf).expect("failed to generate encrypt key");
    let d_aeskey = AesKey::new_decrypt(&buf).expect("failed to generate decrypt key");
    (e_aeskey, d_aeskey)
}

pub fn aes_encrypt<T: AsRef<str>>(key: &AesKey, iv: Vec<u8>, plaintext: T) -> Vec<u8> {
    let buffer = pad(plaintext.as_ref().as_bytes(), None);
    aes_crypt(key, iv, buffer, Mode::Encrypt)
}

pub fn aes_decrypt(key: &AesKey, iv: Vec<u8>, ciphertext: Vec<u8>) -> String {
    let plaintext = aes_crypt(key, iv, ciphertext, Mode::Decrypt);
    String::from_utf8(plaintext)
        .expect("invalid utf8 byte array")
        .trim_end()
        .to_owned()
}

fn aes_crypt(key: &AesKey, mut iv: Vec<u8>, input: Vec<u8>, mode: Mode) -> Vec<u8> {
    let mut output = vec![0u8; input.len()];
    aes_ige(&input, &mut output, &key, &mut iv, mode);
    output
}

fn generate_buffer(length: usize) -> Vec<u8> {
    let mut buf = vec![0; length]; // 128, 192, 256 bits or 16, 24, 32 bytes
    rand_bytes(&mut buf).unwrap();
    buf
}
