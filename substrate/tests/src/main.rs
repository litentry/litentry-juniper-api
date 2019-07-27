#[macro_use]
extern crate jsonrpc_core_client;
extern crate futures;
extern crate substrate_rpc;
extern crate node_primitives;
extern crate hyper;
extern crate sr_primitives;
extern crate node_runtime;
extern crate substrate_transaction_pool;
extern crate substrate_test_runtime;
extern crate substrate_keyring;

use futures::Future;
use hyper::rt;
use jsonrpc_core_client::{transports::http, transports::ws, RpcError};
use substrate_rpc::{system::SystemClient,
                    chain::{ChainClient, number::NumberOrHex},
                    state::StateClient,
                    author::{AuthorClient, hash::ExtrinsicOrHash, }, };
use node_primitives::{Hash};
use substrate_transaction_pool::txpool::ExtrinsicFor;
use node_runtime::{Address, Block, Header, SignedBlock};
use substrate_test_runtime::{Transfer, Extrinsic};
use substrate_keyring::{AccountKeyring};

fn main() {
    let who = AccountKeyring::Alice;
    let nonce = 0;
    let account = AccountKeyring::from(AccountKeyring::Alice);
    let transfer = Transfer {
        from: who.into(),
        to: AccountKeyring::Bob.into(),
        nonce,
        amount: 1,
    };
//    let signature = transfer.using_encoded(|e| who.sign(e));
//    let signed_transfer = Extrinsic::Transfer(transfer, signature.into());
    let signed_transfer = transfer.into_signed_tx();
//    println!("{}", signed_transfer.());
}






