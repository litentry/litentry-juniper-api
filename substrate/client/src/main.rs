extern crate substrate_api_client;
extern crate codec;
extern crate primitives;
extern crate log;
extern crate node_runtime;
extern crate node_primitives;
extern crate system;
extern crate srml_balances;

use std::sync::mpsc::channel;
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

fn main() {

    let url = "192.168.2.158:9944";
    let from = AccountKey::new("//Alice", Some(""), CryptoKind::Sr25519);
    let api = Api::new(format!("ws://{}", url))
        .set_signer(from);

    let (events_in, events_out) = channel();
    api.subscribe_events(events_in.clone());

    loop {
        println!("start to try get events");
        let event_str = events_out.recv().unwrap();

        let _unhex = hexstr_to_vec(event_str);
        let mut _er_enc = _unhex.as_slice();
        println!("raw message {:?}", &mut _er_enc);
        decode_events(&mut _er_enc);
        let _events = Vec::<system::EventRecord::<Event, Hash>>::decode(&mut _er_enc);
        match _events {
            Ok(evts) => {
                for evr in &evts {
                    println!("decoded: phase {:?} event {:?}", evr.phase, evr.event);
                    match &evr.event {
                        Event::balances(be) => {
                            println!(">>>>>>>>>> balances event: {:?}", be);
                            match &be {
                                srml_balances::RawEvent::Transfer(transactor, dest, value, fee) => {
                                    println!("Transactor: {:?}", transactor);
                                    println!("Destination: {:?}", dest);
                                    println!("Value: {:?}", value);
                                    println!("Fee: {:?}", fee);
                                },
                                _ => {
                                    println!("ignoring unsupported balances event");
                                },
                            }
                        },
                        _ => {
                            println!("ignoring unsupported module event: {:?}", evr.event)
                        },
                    }
                }
            }
            Err(_) => println!("couldn't decode event record list")
        }
    }
}