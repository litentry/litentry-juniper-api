extern crate serde;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate hyper;
#[macro_use]
extern crate ureq;
extern crate hex;

//use ureq::body::Payload;

use serde::{Serialize, Deserialize};
use litentry_substrate_utils::{get_public_from_address, decode_balance};
use substrate_primitives::blake2_256;

#[derive(Serialize, Deserialize)]
struct PostResult {
    jsonrpc: String,
    result: String,
    id: i32,
}

const URI: &str = "http://192.168.2.158:9933";
// const URI: &str = "http://127.0.0.1:9933";

pub fn get_balance(address: &str) -> Option<i128> {
    let public_key = get_public_from_address(address);
    let method = "Balances FreeBalance".as_bytes();
    let mut input = [0u8; 52];
    let mut index = 0;
    for letter in method {
        input[index] = *letter;
        index = index + 1;
    };
    //TODO if address is invalid. should return None.
    for letter in &public_key.unwrap().0 {
        input[index] = *letter;
        index = index + 1;
    };

    let result = blake2_256(&input);
    let mut hex_str = String::from("0x");
    hex_str.push_str(&hex::encode(&result));
    let resp = ureq::post(URI)
        .set("Content-Type", "application/json")
        .send_json(json!({
            "jsonrpc": "2.0".to_owned(),
            "method": "state_getStorage".to_owned(),
            "params": (hex_str, ),
            "id": 1,
    }));

    if resp.ok() {
        let result_string = &resp.into_string().unwrap();
        println!("get_balance result is {}", result_string.to_owned());
        let result: PostResult = serde_json::from_str(&result_string).unwrap();
        let balance = decode_balance(&result.result);
        Some(balance)
    } else {
        None
    }
}

//pub fn get<A>(method_name: &str, params_tuple: A) -> Result<String, String> {
//    let resp = ureq::post(URI)
//        .set("Content-Type", "application/json")
//        .send_json(json!({
//            "jsonrpc": "2.0".to_owned(),
//            "method": method_name.to_owned(),
//            "params": params_tuple,
//            "id": 1,
//    }));
//
//    if resp.ok() {
//        let result: PostResult = serde_json::from_str(&resp.into_string().unwrap()).unwrap();
//        Ok(result.result.to_owned())
//    } else {
//        Err("post result is nok.".to_owned())
//    }
//}
//
//
//pub fn get_one<A>(method_name: &str, params: (A)) -> Result<String, String> {
//    let resp = ureq::post(URI)
//        .set("Content-Type", "application/json")
//        .send_json(json!({
//            "jsonrpc": "2.0".to_owned(),
//            "method": method_name.to_owned(),
//            "params": params,
//            "id": 1,
//    }));
//
//    if resp.ok() {
//        let result: PostResult = serde_json::from_str(&resp.into_string().unwrap()).unwrap();
//        Ok(result.result.to_owned())
//    } else {
//        Err("post result is nok.".to_owned())
//    }
//}
//
//pub fn get_two<A, B>(method_name: &str, params: (A, B)) -> Result<String, String> {
//    let resp = ureq::post(URI)
//        .set("Content-Type", "application/json")
//        .send_json(json!({
//            "jsonrpc": "2.0".to_owned(),
//            "method": method_name.to_owned(),
//            "params": params,
//            "id": 1,
//    }));
//
//    if resp.ok() {
//        let result: PostResult = serde_json::from_str(&resp.into_string().unwrap()).unwrap();
//        Ok(result.result.to_owned())
//    } else {
//        Err("post result is nok.".to_owned())
//    }
//}
