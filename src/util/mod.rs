extern crate num_bigint;
pub mod event;
use num_bigint::BigUint;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use serde::Serialize;
// use serde_json::Value;
// use serde_json::map::Map;
use openssl::{
    hash::{hash, MessageDigest},
    symm::{encrypt, Cipher},
};
use serde_urlencoded;

static MODULUS:&str = "00e0b509f6259df8642dbc35662901477df22677ec152b5ff68ace615bb7b725152b3ab17a876aea8a5aa76d2e417629ec4ee341f56135fccf695280104e0312ecbda92557c93870114af6c9d05c4f7f0c3685b7a46bee255932575cce10b424d813cfe4875d3e82047b97ddef52741d546b8e289dc6935b3ece0462db0a22b8e7";
static NONCE: &str = "0CoJUm6Qyw8W8jud";
static PUBKEY: &str = "010001";

pub struct Encrypt;

#[allow(unused)]
impl Encrypt {
    pub fn encrypt_id(id: String) -> String {
        let magic = b"3go8&$8*3*3h0k(2)2";
        let magic_len = magic.len();
        let id = id;
        let mut song_id = id.clone().into_bytes();
        id.as_bytes().iter().enumerate().for_each(|(i, sid)| {
            song_id[i] = *sid ^ magic[i % magic_len];
        });
        let result = hash(MessageDigest::md5(), &song_id).unwrap();
        base64::encode_config(&hex::encode(result), base64::URL_SAFE)
            .replace("/", "_")
            .replace("+", "-")
    }

    pub fn encrypt_login(text: impl Serialize + std::fmt::Debug) -> String {
        let data = serde_json::to_string(&text).unwrap();
        let secret = Self.create_key(16);
        let secret = "e0e80547fa3ecd5a".to_owned();
        let params = Self.aes(Self.aes(data, NONCE), &secret);
        #[allow(non_snake_case)]
        let encSecKey = Self.rsa(secret);
        // let mut res = Map::new();
        // res.insert("params".to_owned(), params.into());
        // res.insert("encSecKey".to_owned(), encSecKey.into());
        // Value::Object(res)
        let meal = &[("params", params), ("encSecKey", encSecKey)];
        serde_urlencoded::to_string(&meal).unwrap_or("".to_owned())
    }

    fn aes(&self, text: String, key: &str) -> String {
        let pad = 16 - text.len() % 16;
        let p = pad as u8 as char;
        let mut text = text;
        for _ in 0..pad {
            text.push(p);
        }
        let text = text.as_bytes();
        let cipher = Cipher::aes_128_cbc();
        let ciphertext = encrypt(cipher, key.as_bytes(), Some(b"0102030405060708"), text).unwrap();
        base64::encode(&ciphertext)
    }

    fn rsa(&self, text: String) -> String {
        let text = text.chars().rev().collect::<String>();
        let text = BigUint::parse_bytes(hex::encode(text).as_bytes(), 16).unwrap();
        let pubkey = BigUint::parse_bytes(PUBKEY.as_bytes(), 16).unwrap();
        let modulus = BigUint::parse_bytes(MODULUS.as_bytes(), 16).unwrap();
        let pow = text.modpow(&pubkey, &modulus);
        pow.to_str_radix(16)
    }

    fn create_key(&self, len: usize) -> String {
        return hex::encode(
            String::from_utf8(
                thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(len)
                    .collect::<Vec<u8>>(),
            )
            .unwrap(),
        )[..16]
            .to_string();
    }
}
