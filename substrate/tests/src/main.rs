#[macro_use]
extern crate jsonrpc_core_client;
extern crate futures;
extern crate substrate_rpc;
extern crate node_primitives;
extern crate hyper;
extern crate sr_primitives;
extern crate node_runtime;

mod workable_tx;
use workable_tx::get_tx;

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

use substrate_transaction_pool::txpool::ExtrinsicFor;
use node_runtime::{Address, Block, Header, SignedBlock};
use substrate_test_runtime::{Transfer, AccountKeyring, Keyring, Extrinsic};


pub fn test_author() {

    rt::run(rt::lazy(|| {
        let uri = "http://127.0.0.1:9933";

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
    // vec![1, 2, 3].into()
    client.submit_extrinsic(get_tx().into())
        .map(|result| {
            println!("{:?}", result);
        })
}

