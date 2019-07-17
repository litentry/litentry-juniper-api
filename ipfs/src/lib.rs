extern crate ipfsapi;
use ipfsapi::IpfsApi;
//use failure::{Error};

fn get_ipfs_api(server: &str, port: u16) -> IpfsApi {
    // IpfsApi::new("192.168.1.245", 5001)
    IpfsApi::new(server, port)
}

fn cat_content(api: &IpfsApi, hash: &str) -> String {
    let bytes = api.cat(hash).unwrap();
    String::from_utf8(bytes.collect()).unwrap()
}

fn pin_file(api: &IpfsApi, hash: &str) -> ipfsapi::pin::PinResponse {
    api.pin_add(hash, true).unwrap()
}

fn pin_rm(api: &IpfsApi, hash: &str) -> ipfsapi::pin::PinResponse {
    api.pin_rm(hash, true).unwrap()
}

