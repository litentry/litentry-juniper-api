#[macro_use]
extern crate jsonrpc_core_client;
extern crate futures;
extern crate substrate_rpc;
extern crate node_primitives;
extern crate hyper;
extern crate sr_primitives;
extern crate node_runtime;
extern crate substrate_test_runtime;
extern crate substrate_keyring;
extern crate parity_codec;
extern crate srml_system;
extern crate srml_balances;
extern crate srml_indices;
extern crate sr_io;
extern crate substrate_primitives;
extern crate substrate_serializer;

use std::sync::Arc;

use parity_codec::{Encode, Decode, Compact};
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
use node_runtime::{Address, Block, Header, SignedBlock, Call, BalancesCall, Runtime, UncheckedExtrinsic,
                   SignedExtra};
use substrate_transaction_pool::txpool::ExtrinsicFor;
use substrate_test_runtime::{Transfer, Extrinsic};
use substrate_keyring::{AccountKeyring, Ed25519Keyring, Sr25519Keyring};
use sr_primitives::{generic::{Era},
                    traits::{Verify, Lazy, SignedExtension}};
use substrate_primitives::{sr25519::Public, Blake2Hasher, H256, ed25519, Pair, sr25519};
use srml_system::{CheckEra, CheckNonce, CheckWeight};
use sr_io::blake2_256;
use srml_balances::TakeFees;
use substrate_serializer as ser;
use srml_indices::address::Address as RawAddress;


fn get_tx() -> std::vec::Vec<u8> {
    let alice: Arc<ed25519::Pair> = Arc::new(Ed25519Keyring::Alice.into());
    let bob: Arc<ed25519::Pair> = Arc::new(Ed25519Keyring::Bob.into());
    let to = Public::from_raw(AccountKeyring::Alice.to_raw_public());
    let function = Call::Balances(BalancesCall::transfer(to.into(), 0));

    let third: Arc<sr25519::Pair> = Arc::new(Sr25519Keyring::Alice.into());

    let payload = (
        function,
        Era::immortal(),
        ser::from_str::<H256>("\"0x8e2cecefe5e385a5eaaef255a861294ea59c7939847786029cf3b3f12947ba99\"").unwrap()

    );

    let check_era = CheckEra::<Runtime>::from(Era::Immortal);
    let check_nonce = CheckNonce::<Runtime>::from(0);
    let check_weight = CheckWeight::<Runtime>::from();
    let take_fees = TakeFees::<Runtime>::from(0);
    let extra = (check_era, check_nonce, check_weight, take_fees);

    let signature = payload.using_encoded(|data| alice.sign(&data.encode()));
    // let signature = alice.sign(&payload.encode()).into();
    let id = alice.public();
    UncheckedExtrinsic {
        signature: Some((RawAddress::Id(third.public()), signature.into(), extra)),
        function: payload.0,
    }.encode()
}



//pub fn get_tx() -> std::vec::Vec<u8> {
//    let who = AccountKeyring::Alice;
//    let nonce = 0;
//    let account = AccountKeyring::from(AccountKeyring::Alice);
//    let transfer = Transfer {
//        from: who.into(),
//        to: AccountKeyring::Bob.into(),
//        nonce,
//        amount: 1,
//    };
//    let signed_transfer = transfer.into_signed_tx();
//
//    let xt = signed_transfer.encode();
//    xt
//}

//fn get_tx() -> std::vec::Vec<u8> {
//    let amount = 5;
//    let mut index = 0;
//    let to = Public::from_raw(AccountKeyring::Alice.to_raw_public());
//    let from = Public::from_raw(AccountKeyring::Alice.to_raw_public());
//    // hardcode here, should get via chain_getBlockHash(0) api.
//    // let genesis_hash = Hash::from(b"8e2cecefe5e385a5eaaef255a861294ea59c7939847786029cf3b3f12947ba99");
//
//    let genesis_hash = ser::from_str::<H256>("\"0x8e2cecefe5e385a5eaaef255a861294ea59c7939847786029cf3b3f12947ba99\"");
//
//    // let genesis_hash = "0x8e2cecefe5e385a5eaaef255a861294ea59c7939847786029cf3b3f12947ba99";
//    let signer = AccountKeyring::Alice.clone();
//
//    let function = Call::Balances(BalancesCall::transfer(to.into(), amount));
//
//    let check_era = CheckEra::<Runtime>::from(Era::Immortal);
//    let check_nonce = CheckNonce::<Runtime>::from(index);
//    let check_weight = CheckWeight::<Runtime>::from();
//    let take_fees = TakeFees::<Runtime>::from(0);
//    let extra = (check_era, check_nonce, check_weight, take_fees);
//
//    let raw_payload = (function, extra.clone(), genesis_hash);
//
//
//    let signature = raw_payload.using_encoded(|payload| if payload.len() > 256 {
//        signer.sign(&blake2_256(payload)[..])
//    } else {
//        signer.sign(payload.encode())
//    });
//    UncheckedExtrinsic::new_signed(
//        raw_payload.0,
//        from.into(),
//        signature.into(),
//        extra,
//    ).encode()
//}

pub fn main() {

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
    client.submit_extrinsic(get_tx().into())
        .map(|result| {
            println!("{:?}", result);
        })
}

