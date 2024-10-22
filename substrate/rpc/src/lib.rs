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
use litentry_substrate_utils::{get_public_from_address, decode_balance, decode_bytes_to_u64,
                               decode_hex_to_u64, decode_hex_hash, decode_token, u64_to_little_vec};
use substrate_primitives::{blake2_256, twox_128};

#[derive(Serialize, Deserialize)]
struct PostResult {
    jsonrpc: String,
    result: Option<String>,
    id: i32,
}

pub struct Rpc {
    // "http://192.168.2.158:9933";
    uri: String,
}

impl Rpc {
    pub fn new(uri_ref: &str) -> Self {
        Rpc {
            uri: uri_ref.to_owned(),
        }
    }

    pub fn get_balance(&self, address: &str) -> Option<i128> {
        let public_key = get_public_from_address(address);
        let method = "Balances FreeBalance".as_bytes();
        let mut input = [0u8; 52];
        let mut index = 0;
        for letter in method {
            input[index] = *letter;
            index = index + 1;
        };
        if public_key.is_err() {
            return None;
        }

        for letter in &public_key.unwrap().0 {
            input[index] = *letter;
            index = index + 1;
        };

        let result = blake2_256(&input);
        let mut hex_str = String::from("0x");
        hex_str.push_str(&hex::encode(&result));
        let resp = ureq::post(&self.uri)
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
            if result.result.is_some() {
                Some(decode_balance(&result.result.unwrap()))
            } else {
                Some(0)
            }
        } else {
            None
        }
    }

    pub fn get_identity_count(&self) -> Option<u64> {
        let method = "LitentryStorage IdentitiesCount".as_bytes();
        let result = twox_128(&method);
        let mut hex_str = String::from("0x");
        hex_str.push_str(&hex::encode(&result));
        let resp = ureq::post(&self.uri)
            .set("Content-Type", "application/json")
            .send_json(json!({
            "jsonrpc": "2.0".to_owned(),
            "method": "state_getStorage".to_owned(),
            "params": (hex_str, ),
            "id": 1,
    }));

        if resp.ok() {
            let result_string = &resp.into_string().unwrap();
            println!("get_identity_count result is {}", result_string.to_owned());
            let result: PostResult = serde_json::from_str(&result_string).unwrap();
            if result.result.is_some() {
                Some(decode_hex_to_u64(&result.result.unwrap()))
            } else {
                Some(0)
            }
        } else {
            None
        }
    }

    pub fn get_token_count(&self) -> Option<u64> {
        let method = "LitentryStorage AuthorizedTokensCount".as_bytes();
        let result = twox_128(&method);
        let mut hex_str = String::from("0x");
        hex_str.push_str(&hex::encode(&result));
        let resp = ureq::post(&self.uri)
            .set("Content-Type", "application/json")
            .send_json(json!({
            "jsonrpc": "2.0".to_owned(),
            "method": "state_getStorage".to_owned(),
            "params": (hex_str, ),
            "id": 1,
    }));

        if resp.ok() {
            let result_string = &resp.into_string().unwrap();
            println!("get_token_count result is {}", result_string.to_owned());
            let result: PostResult = serde_json::from_str(&result_string).unwrap();
            if result.result.is_some() {
                Some(decode_hex_to_u64(&result.result.unwrap()))
            } else {
                Some(0)
            }
        } else {
            None
        }
    }

    // get identity hash with hex format.
    pub fn get_identity_via_index(&self, identity_index: i32) -> Option<String> {
        let mut input_vec: Vec<u8> = Vec::new();
        let mut method = "LitentryStorage IdentitiesArray".as_bytes().to_vec();
        input_vec.append(&mut method);
        input_vec.append(&mut u64_to_little_vec(identity_index as u64));

        let result = blake2_256(&input_vec[..]);
        let mut hex_str = String::from("0x");
        hex_str.push_str(&hex::encode(&result));
        println!("LitentryStorage IdentitiesArray storge key is {}", &hex_str);
        let resp = ureq::post(&self.uri)
            .set("Content-Type", "application/json")
            .send_json(json!({
            "jsonrpc": "2.0".to_owned(),
            "method": "state_getStorage".to_owned(),
            "params": (hex_str, ),
            "id": 1,
    }));

        if resp.ok() {
            let result_string = &resp.into_string().unwrap();
            println!("get_identity_via_index result is {}", result_string.to_owned());
            let result: PostResult = serde_json::from_str(&result_string).unwrap();
            result.result
        } else {
            println!("get_identity_via_index failed.");
            None
        }
    }

    pub fn get_identity_owner_via_hash(&self, identity_hash: &str) -> Option<String> {
        let mut input_vec: Vec<u8> = Vec::new();
        let mut method = "LitentryStorage IdentityOwner".as_bytes().to_vec();
        input_vec.append(&mut method);
        input_vec.append(&mut decode_hex_hash(&identity_hash));

        let result = blake2_256(&input_vec[..]);
        let mut hex_str = String::from("0x");
        hex_str.push_str(&hex::encode(&result));
        println!("LitentryStorage IdentitiesOwner storge key is {}", &hex_str);
        let resp = ureq::post(&self.uri)
            .set("Content-Type", "application/json")
            .send_json(json!({
            "jsonrpc": "2.0".to_owned(),
            "method": "state_getStorage".to_owned(),
            "params": (hex_str, ),
            "id": 1,
    }));

        if resp.ok() {
            let result_string = &resp.into_string().unwrap();
            println!("get_identity_owner_via_hash result is {}", result_string.to_owned());
            let result: PostResult = serde_json::from_str(&result_string).unwrap();
            result.result
        } else {
            println!("get_identity_owner_via_hash failed.");
            None
        }
    }

    // get token include token hash hex, balance as string, data, data type and expired.
    pub fn get_token_hash_via_index(&self, token_index: i32) -> Option<String> {
        let mut input_vec: Vec<u8> = Vec::new();
        let mut method = "LitentryStorage AuthorizedTokensArray".as_bytes().to_vec();
        input_vec.append(&mut method);
        input_vec.append(&mut u64_to_little_vec(token_index as u64));

        let result = blake2_256(&input_vec[..]);
        let mut hex_str = String::from("0x");
        hex_str.push_str(&hex::encode(&result));
        let resp = ureq::post(&self.uri)
            .set("Content-Type", "application/json")
            .send_json(json!({
            "jsonrpc": "2.0".to_owned(),
            "method": "state_getStorage".to_owned(),
            "params": (hex_str, ),
            "id": 1,
    }));

        if resp.ok() {
            let result_string = &resp.into_string().unwrap();
            println!("get_token_hash_via_index result is {}", result_string.to_owned());
            let result: PostResult = serde_json::from_str(&result_string).unwrap();
            result.result
        } else {
            println!("get_token_hash_via_index failed.");
            None
        }
    }


    // get token include token hash hex, balance as string, data, data type and expired.
    pub fn get_token_via_hash(&self, token_hash: &str) -> Option<(String, String, String, String, String)> {
        let mut input_vec: Vec<u8> = Vec::new();
        let mut method = "LitentryStorage AuthorizedTokens".as_bytes().to_vec();
        input_vec.append(&mut method);
        input_vec.append(&mut decode_hex_hash(token_hash));

        let result = blake2_256(&input_vec[..]);
        let mut hex_str = String::from("0x");
        hex_str.push_str(&hex::encode(&result));
        let resp = ureq::post(&self.uri)
            .set("Content-Type", "application/json")
            .send_json(json!({
            "jsonrpc": "2.0".to_owned(),
            "method": "state_getStorage".to_owned(),
            "params": (hex_str, ),
            "id": 1,
    }));

        if resp.ok() {
            let result_string = &resp.into_string().unwrap();
            println!("get_token_via_hash result is {}", result_string.to_owned());
            let result: PostResult = serde_json::from_str(&result_string).unwrap();
            decode_token(&result.result.unwrap())
        } else {
            println!("get_token_via_hash failed.");
            None
        }
    }

    pub fn get_token_owner_via_hash(&self, token_hash: &str) -> Option<String> {
        let mut input_vec: Vec<u8> = Vec::new();
        let mut method = "LitentryStorage AuthorizedTokenOwner".as_bytes().to_vec();
        input_vec.append(&mut method);
        input_vec.append(&mut decode_hex_hash(&token_hash));

        let result = blake2_256(&input_vec[..]);
        let mut hex_str = String::from("0x");
        hex_str.push_str(&hex::encode(&result));
        let resp = ureq::post(&self.uri)
            .set("Content-Type", "application/json")
            .send_json(json!({
            "jsonrpc": "2.0".to_owned(),
            "method": "state_getStorage".to_owned(),
            "params": (hex_str, ),
            "id": 1,
    }));

        if resp.ok() {
            let result_string = &resp.into_string().unwrap();
            println!("get_token_owner_via_hash result is {}", result_string.to_owned());
            let result: PostResult = serde_json::from_str(&result_string).unwrap();
            result.result
        } else {
            None
        }
    }

    pub fn get_token_identity_via_hash(&self, token_hash: &str) -> Option<String> {
        let mut input_vec: Vec<u8> = Vec::new();
        let mut method = "LitentryStorage AuthorizedTokenIdentity".as_bytes().to_vec();
        input_vec.append(&mut method);
        input_vec.append(&mut decode_hex_hash(&token_hash));

        let result = blake2_256(&input_vec[..]);
        let mut hex_str = String::from("0x");
        hex_str.push_str(&hex::encode(&result));
        let resp = ureq::post(&self.uri)
            .set("Content-Type", "application/json")
            .send_json(json!({
            "jsonrpc": "2.0".to_owned(),
            "method": "state_getStorage".to_owned(),
            "params": (hex_str, ),
            "id": 1,
    }));

        if resp.ok() {
            let result_string = &resp.into_string().unwrap();
            println!("get_token_identity_via_hash result is {}", result_string.to_owned());
            let result: PostResult = serde_json::from_str(&result_string).unwrap();
            result.result
        } else {
            None
        }
    }
}

