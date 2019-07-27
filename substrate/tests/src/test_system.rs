#[macro_use]
extern crate jsonrpc_core_client;
extern crate futures;
extern crate substrate_rpc;
extern crate node_primitives;
extern crate hyper;
extern crate sr_primitives;
extern crate srml_indices;
extern crate node_runtime;

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

//use srml_indices::{Address};
use node_primitives::{Hash, BlockNumber};
// use sr_primitives::generic::{BlockId, SignedBlock, Header, Block, UncheckedMortalCompactExtrinsic};

//use sr_primitives::traits::{BlakeTwo256};
use node_runtime::{Address, Block, Header, SignedBlock};
fn run_system() {
    rt::run(rt::lazy(|| {
        let uri = "http://192.168.2.158:9933";

        http::connect(uri)
            .and_then(|client: SystemClient<Hash, Hash>| {
                get_system_api(&client)
            })
            .map_err(|e| {
                println!("Error: {:?}", e);
            })
    }));

}

fn get_system_api(client: &SystemClient<Hash, Hash>) -> impl Future<Item=(), Error=RpcError> {
    let _ = client.system_name()
        .map(|system_name| {
            println!("{:?}", system_name);
        }).wait();
    let _ = client.system_chain()
        .map(|system_chain| {
            println!("{:?}", system_chain);
        }).wait();
    let _ = client.system_version()
        .map(|system_version| {
            println!("{:?}", system_version);
        }).wait();
    client.system_properties()
        .map(|system_properties| {
            println!("{:?}", system_properties);
        })
}



