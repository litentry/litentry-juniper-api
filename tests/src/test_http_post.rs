//[dependencies]
//futures-util = "0.2.1"
//serde = "1.0"
//serde_json = "1.0"
//serde_derive = "1.0.98"
//hyper = "0.12.33"
//ureq = { version = "0.11.0", features = ["json"]}

extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate hyper;
#[macro_use]
extern crate ureq;

use serde::{Serialize, Deserialize};
use std::str;
use std::os;
use hyper::{Client, Request};
use hyper::Body;

#[derive(Serialize, Deserialize)]
struct PostResult {
    jsonrpc: String,
    result: String,
    id: i32,
}

fn main() {
    let resp = ureq::post("http://192.168.1.224:9933/")
        .set("Content-Type", "application/json")
        .send_json(json!({
            "jsonrpc": "2.0".to_owned(),
            "method": "system_name".to_owned(),
            "params": [],
            "id": 1,
    }));

    if resp.ok() {
        let result: PostResult = serde_json::from_str(&resp.into_string().unwrap()).unwrap();
        println!("{}", &result.result);
    };
}
