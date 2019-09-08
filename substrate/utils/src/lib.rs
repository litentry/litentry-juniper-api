extern crate substrate_primitives;
extern crate substrate_application_crypto;
extern crate hex;
extern crate byteorder;
//#[macro_use]
//extern crate hex_literal;
// extern crate parity_scale_codec;

use byteorder::{BigEndian, LittleEndian, ReadBytesExt, WriteBytesExt};
use substrate_primitives::{sr25519, sr25519::{Signature}, Pair, crypto, crypto::{Ss58Codec}};
use std::io::Cursor;
use std::vec::Vec;
use substrate_primitives::{blake2_256, twox_128};

// use hex_literal::hex;
// use parity_scale_codec::DecodeLength;

pub fn sign(message: &str) -> Signature {
    let seed = "0x235c69907d33b85f27bd78e73ff5d0c67bd4894515cc30c77f4391859bc1a3f2";
    let pair = sr25519::Pair::from_seed(&hex_str_to_hash(seed));

    let signature = pair.sign(&decode_hex_hash(message)[..]);
    println!("Pair public key is {:?}", &pair.public());
    println!("signature result is {:?}", hex::encode(&signature));
    signature
}

pub fn verify_signature(public_key_str: &str, signature_str: &str, message: &str) -> bool {
    sign(message);
    println!("verify_signature sender public key is {}", public_key_str);
//    let a = sr25519::Public::from_string_with_version();
    let public_key: sr25519::Public = sr25519::Public::from_raw(hex_str_to_hash(public_key_str));
    println!("after decode public key is {:?}", &public_key);
    let raw_data = &decode_hex_hash(message)[..];
    println!("raw data is {:?}, {}", &raw_data, message);
    // check signature length to avoid panic.
    let signature_vec = &decode_hex_hash(signature_str);
    if signature_vec.len() != 64 {
        println!("signature from rpc with wrong length.");
        return false;
    }
    let signature = Signature::from_slice(&signature_vec[..]);
    println!("go to verify method.");
    <sr25519::Pair as crypto::Pair>::verify(&signature, &raw_data, &public_key)
}

pub fn get_public_from_address(address: &str) -> Result<sr25519::Public, &str> {
    let result = sr25519::Public::from_string_with_version(address);
    match result {
        Ok((key, _)) => Ok(key),
        Err(_) => Err("invalid address"),
    }
}

pub fn hex_str_to_hash(hex_str: &str) -> [u8; 32] {
    let _unhex = decode_hex_hash(hex_str);
    let mut a = [0u8; 32];
    a.copy_from_slice(&_unhex.as_slice()[0..32]);
    a
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

pub fn hexstr_to_vec(mut hexstr: String) -> Vec<u8> {
    if hexstr.starts_with("0x") {
        hex::decode(hexstr.split_off(2)).unwrap()
    } else {
        hex::decode(&hexstr).unwrap()
    }
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
                let account = hex::encode(&data[cursor+1..cursor+33]);
                let id_hash = hex::encode(&data[cursor+33..cursor+65]);
                println!("account is {:?}, id hash is {:?}", account, id_hash);
                cursor += 64;
            } else if event_index == 1 {
                let account = hex::encode(&data[cursor+1..cursor+33]);
                let id_hash = hex::encode(&data[cursor+33..cursor+65]);
                let token_hash = hex::encode(&data[cursor+65..cursor+97]);
                println!("account is {:?}, id hash is {:?}, token hash {:?} ", account, id_hash, token_hash);
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

pub fn twox_storage_key_hash(module: &str, storage_key_name: &str) -> String {
    let mut key = [module, storage_key_name].join(" ").as_bytes().to_vec();
    let mut keyhash;
    keyhash = hex::encode(twox_128(&key));
    keyhash.insert_str(0, "0x");
    keyhash
}