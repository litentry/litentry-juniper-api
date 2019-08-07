extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate hyper;
#[macro_use]
extern crate ureq;
extern crate sr_io;
extern crate hex;
//use ureq::body::Payload;

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
            "params": (hex_str.to_owned(),),
            "id": 1,
    }));

    if resp.ok() {
        println!("{}", &resp.into_string().unwrap());
//        let result: PostResult = serde_json::from_str(&resp.into_string().unwrap()).unwrap();
//        println!("{}", result.result.to_owned());
    } else {
        println!("");
    }
}



