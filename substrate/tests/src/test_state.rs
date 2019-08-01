#[macro_use]
extern crate jsonrpc_core_client;
extern crate futures;
extern crate substrate_rpc;
extern crate node_primitives;
extern crate hyper;
extern crate sr_primitives;
extern crate node_runtime;
extern crate hex;

use futures::Future;
use hyper::rt;
use jsonrpc_core_client::{
    transports::http,
    transports::ws,
    RpcError,
    TypedClient,
    RpcChannel,
};
use substrate_rpc::{system::SystemClient,
                    chain::ChainClient,
                    state::StateClient,
                    author::{
                        AuthorClient,
                        hash::ExtrinsicOrHash,
                    },
};
use node_primitives::{Hash};
use node_runtime::{Address, Block, Header, SignedBlock};
use hex;

fn test_state() {

    rt::run(rt::lazy(|| {
        let uri = "http://192.168.2.158:9933";

        http::connect(uri)
            .and_then(|client: StateClient<Hash>| {
                get_state_api(&client)
            })
            .map_err(|e| {
                println!("Error: {:?}", e);
            })
    }));
}


fn get_state_api(client: &StateClient<Hash>) -> impl Future<Item=(), Error=RpcError> {
    client.runtime_version(Some([0; 32].into()))
        .map(|runtime_version| {
            println!("{:?}", runtime_version);
        });
    // b":code" b"templateModule something"
    client.storage(hex::encode(":code"))
        .map(|data| {
            println!(":?", data)
        })
}


