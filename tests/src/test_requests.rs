extern crate serde;
extern crate serde_json;
extern crate hyper;

#[macro_use]
extern crate serde_derive;
use serde::{Serialize, Deserialize};
use std::str;
use litentry_hyper::Request;
use hyper::header::Headers;

#[derive(Serialize, Deserialize)]
struct Photo {
    jsonrpc: String,
    method: String,
    params: Vec<String>,
    id: i32,
}

fn main() {
    let p = Photo {
        jsonrpc: "jsonrpc".to_owned(),
        method: "system_name".to_owned(),
        params: vec![],
        id: 1,
    };

    let request = Request::builder()
        .method("POST")
        .header("Content-Type", "application/json")
        .uri("http://httpin.org/post")
        .body(Body::from(serde_json::to_string(&p).unwrap()))
        .unwrap();


    let mut url = "http://192.168.1.224:9933/".to_owned();
    let json = serde_json::to_string(&p).expect("Couldn't serialize config");
    println!("{}", json);

    let response = Request::json().post(&url, &json).unwrap();
    println!("response is {:?}", str::from_utf8(response.content()).unwrap());
}