extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate hyper;
#[macro_use]
extern crate ureq;
extern crate sr_io;
extern crate hex;
extern crate substrate_primitives;
//use ureq::body::Payload;

use serde::{Serialize, Deserialize};
use std::str;
use std::os;
use hyper::{Client, Request};
use hyper::Body;
use substrate_primitives::{
    ed25519, sr25519, hexdisplay::HexDisplay, Pair, Public, blake2_256,
    crypto::{Ss58Codec, set_default_ss58_version, Ss58AddressFormat},
};

#[derive(Serialize, Deserialize)]
struct PostResult {
    jsonrpc: String,
    result: String,
    id: i32,
}

pub fn get_public_from_address(address: &str) -> Result<sr25519::Public, &str> {
    let result = sr25519::Public::from_string_with_version(address);

    match result {
        Ok((key, format)) => Ok(key),
        Err(_) => Err("invalid address"),
    }
}

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


fn main() {
    // address from Alice
    let address = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY";
    let public_key = get_public_from_address(address);
    println!("public key is {:?}", public_key.clone().unwrap().0);
    let method = "Balances FreeBalance".as_bytes();
    let mut input = [0u8; 52];
    let mut index = 0;
    for letter in method {
        input[index] = *letter;
        print!("{} ", input[index]);
        index = index + 1;
    };
    println!("");
    for letter in &public_key.unwrap().0 {
        input[index] = *letter;
        print!("{} ", input[index]);
        index = index + 1;
    };
    println!("");


    let result = blake2_256(&input);
    println!("result is {:?}", &result);
    let mut hex_str = String::from("0x");
    hex_str.push_str(&hex::encode(&result));
    println!("{:?}", &hex_str);
    let resp = ureq::post("http://192.168.2.158:9933/")
        .set("Content-Type", "application/json")
        .send_json(json!({
            "jsonrpc": "2.0".to_owned(),
            "method": "state_getStorage".to_owned(),
            "params": (hex_str.to_owned(),),
            "id": 1,
    }));

    if resp.ok() {
        let result: PostResult = serde_json::from_str(&resp.into_string().unwrap()).unwrap();
        let balance = decode_balance(&result.result);


        println!("{}", balance);
    } else {
        println!("");
    }
}



