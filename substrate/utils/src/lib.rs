extern crate substrate_primitives;
extern crate substrate_application_crypto;
extern crate hex;
extern crate byteorder;
// extern crate parity_scale_codec;

use byteorder::{BigEndian, LittleEndian, ReadBytesExt, WriteBytesExt};
use substrate_primitives::{sr25519, sr25519::{Pair, Signature}, crypto, crypto::{Ss58Codec}};
use std::io::Cursor;
use std::vec::Vec;
// use parity_scale_codec::DecodeLength;

pub fn verify_signature(address: &str, signature_str: &str, message: &str) -> bool {
    let public_key: sr25519::Public = get_public_from_address(address).unwrap();
    let raw_data = &decode_hex_hash(message)[..];
    let signature = Signature::from_slice(&decode_hex_hash(signature_str)[..]);
    <Pair as crypto::Pair>::verify(&signature, &raw_data, &public_key)
}

pub fn get_public_from_address(address: &str) -> Result<sr25519::Public, &str> {
    let result = sr25519::Public::from_string_with_version(address);
    match result {
        Ok((key, _)) => Ok(key),
        Err(_) => Err("invalid address"),
    }
}

// hex string to i128
pub fn decode_balance(result: &str) -> i128 {
    let without: String = result.chars().skip(2).collect();
    let number_types = hex::decode(without).unwrap();

    let mut iter = number_types.iter().rev();
    let mut balance: i128 = 0;
    loop {
        match iter.next() {
            Some(x) => {
                balance = balance * 256 + *x as i128;
            }
            None => break,
        }
    }
    balance
}

// hex string to u64
pub fn decode_hex_to_u64(result: &str) -> u64 {
    let without: String = result.chars().skip(2).collect();
    let number_types = hex::decode(without).unwrap();

    let mut iter = number_types.iter().rev();
    let mut result: u64 = 0;
    loop {
        match iter.next() {
            Some(x) => {
                result = result * 256 + *x as u64;
            }
            None => break,
        }
    }
    result
}

// hex string to u64
pub fn decode_bytes_to_i128(byte_vec: &[u8]) -> i128 {
    let mut result = 0_i128;
    for i in byte_vec {
        result = result * 256;
        result = result + *i as i128;
    }
    result
}

// hex string to u64
pub fn decode_bytes_to_u64(byte_vec: &[u8]) -> u64 {
    let mut result = 0u64;
    for i in byte_vec {
        result = result * 256;
        result = result + *i as u64;
    }
    result
}


// hex string to bytes
pub fn decode_hex_hash(data: &str) -> Vec<u8> {
    if data.starts_with("0x") {
        let without: String = data.chars().skip(2).collect();
        hex::decode(without).unwrap()
    } else {
        hex::decode(data).unwrap()
    }
}

// hex string to bytes
pub fn decode_token(data: &str) -> Option<(String, String, String, String, String)> {
    if data.starts_with("0x") {
        let without: String = data.chars().skip(2).collect();
        decode_token_no_0x(&without)
    } else {
        decode_token_no_0x(data)
    }
}

// hex string to bytes
pub fn decode_token_no_0x(data: &str) -> Option<(String, String, String, String, String)> {
    let token_vec = hex::decode(data).unwrap();
    // (4 + 16 + 8 + 8 + 8)
    if token_vec.len() == 72 {
        let token_hash = hex::encode(&token_vec[0..32]);
        let balance = Cursor::new(&token_vec[32..48]).read_i128::<LittleEndian>().unwrap();
        // let balance = decode_bytes_to_i128(&token_vec[32..48]);
        let data = Cursor::new(&token_vec[48..56]).read_u64::<LittleEndian>().unwrap();
        let data_type = Cursor::new(&token_vec[56..64]).read_u64::<LittleEndian>().unwrap();
        let expired = Cursor::new(&token_vec[64..72]).read_u64::<LittleEndian>().unwrap();
        Some((token_hash, balance.to_string(), data.to_string(), data_type.to_string(), expired.to_string()),)
    } else {
        println!("decode_token_no_0x failed.");
        None
    }
}

pub fn u64_to_little_vec(data: u64) -> Vec::<u8> {
    let mut result = vec![];
    result.write_u64::<LittleEndian>(data).unwrap();
    result
}

pub fn decode_events(bytes: &[u8]) {
    // TODO we just deal with events number less than 64 now.
    let data = bytes.to_vec();
    let event_length = data[0] / 4;
    println!("include {} events in system events storage.", event_length);
    let mut cursor = 0;
    for index in 0..event_length {
        cursor += 1; // skip first zero since its hardcoded in phase
        cursor += 4; // skip four bytes as phase it is little endian, 1 means
        cursor += 1;
        let module_index = data[cursor];
        cursor += 1;
        let event_index = data[cursor];

        if module_index == 5 {
            if event_index == 0 {
                cursor += 64;
            } else if event_index == 1 {
                cursor += 96;
            }
        }
        cursor += 1; // skip inputs
        println!("module {} and event {}", module_index, event_index);
        let if_inputs = data[cursor];
        if if_inputs != 0 {
            cursor += 32; // skip logs
        }
    }
}

