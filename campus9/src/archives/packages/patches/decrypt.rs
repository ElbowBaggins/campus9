use crate::archives::Id;
use crate::archives::packages::patches::Version;
use aes_gcm_siv::{Aes256GcmSiv};
use aes_gcm_siv::aead::{Aead, NewAead, generic_array::GenericArray};

pub const KEY_LENGTH: usize = 16;
pub const NONCE_LENGTH: usize = 12;

pub type Key = [u8; KEY_LENGTH];
pub type Nonce = [u8; NONCE_LENGTH];//GenericArray<u8, NONCE_LENGTH>;

const KEY: Key = [
    0xD6, 0x2A, 0xB2, 0xC1, 0x0C, 0xC0, 0x1B, 0xC5, 0x35, 0xDB, 0x7B, 0x86, 0x55, 0xC7, 0xDC, 0x3B
];
const KEY_ALT: Key = [
    0x3A, 0x4A, 0x5D, 0x36, 0x73, 0xA6, 0x60, 0x58, 0x7E, 0x63, 0xE6, 0x76, 0xE4, 0x08, 0x92, 0xB5
];

fn get_cipher(alternate: bool) -> Aes256GcmSiv {
    return if alternate {
        Aes256GcmSiv::new(GenericArray::from_slice(&KEY))
    } else {
        Aes256GcmSiv::new(GenericArray::from_slice(&KEY_ALT))
    }
}

fn make_nonce(package_id: Id, version: Version) -> Nonce {
    return [
        0x84 ^ (package_id >> 8 & 0xFF) as u8,
        0xDF ^ (version as u8),
        0x11,
        0xC0,
        0xAC,
        0xAB,
        0xFA,
        0x20,
        0x33,
        0x11,
        0x26,
        0x99 ^ (package_id as u8 & 0xFF)
    ];
}

pub fn decrypt(package_id: Id, version: Version, alternate_key: bool, data: Vec<u8>) -> Vec<u8> {
    let raw_nonce = make_nonce(package_id, version);
    let nonce = GenericArray::from_slice(&raw_nonce);
    let cipher = get_cipher(alternate_key);
    let result = cipher.decrypt(nonce, data.as_slice());
    if result.is_err() {
        panic!("Decryption failed!")
    } else {
        return result.unwrap();
    }
}