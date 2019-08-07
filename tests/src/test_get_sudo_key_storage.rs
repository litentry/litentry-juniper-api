//[dependencies]
//futures-util = "0.2.1"
//serde = "1.0"
//serde_json = "1.0"
//serde_derive = "1.0.98"
//hyper = "0.12.33"
//ureq = { version = "0.11.0", features = ["json"]}
//sr-io = {git = "https://github.com/paritytech/substrate"}
//hex = "0.3.2"

extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate hyper;
#[macro_use]
extern crate ureq;
extern crate sr_io;
extern crate hex;

use serde::{Serialize, Deserialize};
use std::str;
use std::os;
use hyper::{Client, Request};
use hyper::Body;
use sr_io::{twox_128};

#[derive(Serialize, Deserialize)]
struct PostResult {
    jsonrpc: String,
    result: String,
    id: i32,
}

fn main() {
    let result = twox_128(b"Sudo Key");
    let mut hex_str = String::from("0x");
    hex_str.push_str(&hex::encode(&result));
    println!("{:?}", &hex_str);
    let resp = ureq::post("http://192.168.1.224:9933/")
        .set("Content-Type", "application/json")
        .send_json(json!({
            "jsonrpc": "2.0".to_owned(),
            "method": "state_getStorage".to_owned(),
            // "params": [hex_str.to_owned()],
            "params": (hex_str.to_owned(),),
            "id": 1,
    }));

    if resp.ok() {
        println!("{}", &resp.into_string().unwrap());
//        let result: PostResult = serde_json::from_str(&resp.into_string().unwrap()).unwrap();
//        println!("{}", result.result.to_owned());
    } else {
        println!("nok");
    }
}



