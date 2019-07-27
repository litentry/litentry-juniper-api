#[macro_use]
extern crate jsonrpc_core_client;
extern crate futures;
extern crate substrate_rpc;
extern crate node_primitives;
extern crate hyper;
extern crate sr_primitives;
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
use node_primitives::{Hash};
use node_runtime::{Address, Block, Header, SignedBlock};
pub fn test_author() {

    rt::run(rt::lazy(|| {
        let uri = "http://192.168.2.158:9933";

        http::connect(uri)
            .and_then(|client: AuthorClient<Hash, Hash>| {
                get_author_api(&client)
            })
            .map_err(|e| {
                println!("Error: {:?}", e);
            })
    }));
}

fn get_author_api(client: &AuthorClient<Hash, Hash>) -> impl Future<Item=(), Error=RpcError> {
    client.submit_extrinsic(vec![1, 2, 3].into())
        .map(|result| {
            println!("{:?}", result);
        })
}


