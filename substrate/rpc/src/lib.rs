extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate hyper;
#[macro_use]
extern crate ureq;

//use ureq::body::Payload;

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

// const URI: &str = "http://192.168.1.224:9933";
const URI: &str = "http://127.0.0.1:9933";

pub fn get<A>(method_name: &str, params_tuple: A) -> Result<String, String> {
    let resp = ureq::post(URI)
        .set("Content-Type", "application/json")
        .send_json(json!({
            "jsonrpc": "2.0".to_owned(),
            "method": method_name.to_owned(),
            "params": params_tuple,
            "id": 1,
    }));

    if resp.ok() {
        let result: PostResult = serde_json::from_str(&resp.into_string().unwrap()).unwrap();
        Ok(result.result.to_owned())
    } else {
        Err("post result is nok.".to_owned())
    }
}


pub fn get_one<A>(method_name: &str, params: (A)) -> Result<String, String> {
    let resp = ureq::post(URI)
        .set("Content-Type", "application/json")
        .send_json(json!({
            "jsonrpc": "2.0".to_owned(),
            "method": method_name.to_owned(),
            "params": params,
            "id": 1,
    }));

    if resp.ok() {
        let result: PostResult = serde_json::from_str(&resp.into_string().unwrap()).unwrap();
        Ok(result.result.to_owned())
    } else {
        Err("post result is nok.".to_owned())
    }
}

pub fn get_two<A, B>(method_name: &str, params: (A, B)) -> Result<String, String> {
    let resp = ureq::post(URI)
        .set("Content-Type", "application/json")
        .send_json(json!({
            "jsonrpc": "2.0".to_owned(),
            "method": method_name.to_owned(),
            "params": params,
            "id": 1,
    }));

    if resp.ok() {
        let result: PostResult = serde_json::from_str(&resp.into_string().unwrap()).unwrap();
        Ok(result.result.to_owned())
    } else {
        Err("post result is nok.".to_owned())
    }
}
