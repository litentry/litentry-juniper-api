//#substrate-subxt = {git = "https://github.com/paritytech/substrate-subxt"}
//substrate-subxt = {git = "https://github.com/litentry/substrate-subxt"}
//log = "0.4"
//url = "1.7.2"
//futures = "0.1.28"
//derive_more = "0.14.0"
//jsonrpc-core-client = { version = "12.1.0", features = ["ws"] }
//num-traits = { version = "0.2", default-features = false }
//env_logger = "0.6"
//tokio = "0.1"
//serde = { version = "1.0", features = ["derive"] }
//parity-scale-codec = { version = "1.0", default-features = false, features = ["derive", "full"] }
//runtime_support = { git = "https://github.com/paritytech/substrate/", package = "srml-support" }
//runtime_primitives = { git = "https://github.com/paritytech/substrate/", package = "sr-primitives" }
//srml-system = { git = "https://github.com/paritytech/substrate/", package = "srml-system" }
//node-runtime = { git = "https://github.com/paritytech/substrate/", package = "node-runtime" }
//substrate-keyring = { git = "https://github.com/paritytech/substrate/", package = "substrate-keyring" }

extern crate substrate_subxt;
extern crate url;
extern crate futures;
extern crate tokio;
extern crate runtime_primitives;
extern crate runtime_support;
extern crate node_runtime;
extern crate srml_system;
use runtime_primitives::traits::StaticLookup;
use substrate_subxt::{ClientBuilder, Client, srml::system::System};
//                      srml::litentry::{Litentry, LitentryCalls}, };
use url::Url;
use futures::future::Future;
use substrate_subxt::srml::balances::{
    Balances,
    BalancesCalls,
    BalancesStore,
};
use futures::stream::Stream;
use parity_scale_codec::Encode;
use runtime_primitives::generic::Era;
use runtime_support::StorageMap;
use substrate_keyring::AccountKeyring;
use substrate_primitives::{
    blake2_256,
    storage::StorageKey,
    Pair,
};

struct Runtime;

impl System for Runtime {
    type Index = <node_runtime::Runtime as srml_system::Trait>::Index;
    type BlockNumber = <node_runtime::Runtime as srml_system::Trait>::BlockNumber;
    type Hash = <node_runtime::Runtime as srml_system::Trait>::Hash;
    type Hashing = <node_runtime::Runtime as srml_system::Trait>::Hashing;
    type AccountId = <node_runtime::Runtime as srml_system::Trait>::AccountId;
    type Lookup = <node_runtime::Runtime as srml_system::Trait>::Lookup;
    type Header = <node_runtime::Runtime as srml_system::Trait>::Header;
    type Event = <node_runtime::Runtime as srml_system::Trait>::Event;

    type SignedExtra = (
        srml_system::CheckGenesis<node_runtime::Runtime>,
        srml_system::CheckEra<node_runtime::Runtime>,
        srml_system::CheckNonce<node_runtime::Runtime>,
        srml_system::CheckWeight<node_runtime::Runtime>,
        srml_balances::TakeFees<node_runtime::Runtime>,
    );
    fn extra(nonce: Self::Index) -> Self::SignedExtra {
        (
            srml_system::CheckGenesis::<node_runtime::Runtime>::new(),
            srml_system::CheckEra::<node_runtime::Runtime>::from(Era::Immortal),
            srml_system::CheckNonce::<node_runtime::Runtime>::from(nonce),
            srml_system::CheckWeight::<node_runtime::Runtime>::new(),
            srml_balances::TakeFees::<node_runtime::Runtime>::from(0),
        )
    }
}

impl Balances for Runtime {
    type Balance = <node_runtime::Runtime as srml_balances::Trait>::Balance;
}

impl Litentry for Runtime {

}

type Index = <Runtime as System>::Index;
type AccountId = <Runtime as System>::AccountId;
type Address = <<Runtime as System>::Lookup as StaticLookup>::Source;
type Balance = <Runtime as Balances>::Balance;

pub fn call() {

    let mut rt = tokio::runtime::Runtime::new().unwrap();
    let client_future = ClientBuilder::<Runtime>::new()
        .set_url(Url::parse("ws:192.168.2.158:9944").unwrap())
        .build();
    let client = rt.block_on(client_future).unwrap();
    client.subscribe_events();

//    let signer = AccountKeyring::Alice.pair();
//    let mut xt = rt.block_on(client.xt(signer, None)).unwrap();
//
//    let dest = AccountKeyring::Bob.pair().public();
//    rt.block_on(xt.transfer(dest.clone().into(), 10_000))
//        .unwrap();
    rt.block_on(xt.register_identity())
        .unwrap();

    println!("metadata is {:?}", client.metadata());
}
