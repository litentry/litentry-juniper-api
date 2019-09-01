extern crate substrate_api_client;
extern crate codec;
extern crate primitives;
extern crate log;
extern crate node_runtime;
extern crate node_primitives;
extern crate system;
extern crate srml_balances;

use std::thread;
use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;
use log::{info, debug, error};
use node_primitives::Hash;
use litentry_substrate_utils::decode_events;

// This module depends on node_runtime.
// To avoid dependency collisions, node_runtime has been removed from the substrate-api-client library.
// Replace this crate by your own if you run a custom substrate node to get your custom events.
use node_runtime::Event;
use substrate_api_client::utils::hexstr_to_vec;
use codec::{Encode, Decode};
use substrate_api_client::{
    Api,
    compose_extrinsic,
    crypto::{AccountKey, CryptoKind},
    extrinsic,
    extrinsic::{balances, xt_primitives, contract}
};

pub fn subscribe_events(url: &str, sender: Sender<String>) {
    let api = Api::new(format!("ws://{}", url));
    api.subscribe_events(sender.clone());
}