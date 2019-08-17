//extern crate substrate_subxt;
//extern crate url;
//extern crate futures;
//extern crate tokio;
//
//use substrate_subxt::{ClientBuilder, Client, srml::system::System};
//use url::Url;
//use futures::future::Future;
//use substrate_subxt::srml::balances::{
//    Balances,
//    BalancesCalls,
//    BalancesStore,
//};
//use futures::stream::Stream;
//use parity_scale_codec::Encode;
//use runtime_primitives::generic::Era;
//use runtime_support::StorageMap;
//use substrate_keyring::AccountKeyring;
//use substrate_primitives::{
//    blake2_256,
//    storage::StorageKey,
//    Pair,
//};
//
//struct Runtime;
//
//impl System for Runtime {
//    type Index = <node_runtime::Runtime as srml_system::Trait>::Index;
//    type BlockNumber = <node_runtime::Runtime as srml_system::Trait>::BlockNumber;
//    type Hash = <node_runtime::Runtime as srml_system::Trait>::Hash;
//    type Hashing = <node_runtime::Runtime as srml_system::Trait>::Hashing;
//    type AccountId = <node_runtime::Runtime as srml_system::Trait>::AccountId;
//    type Lookup = <node_runtime::Runtime as srml_system::Trait>::Lookup;
//    type Header = <node_runtime::Runtime as srml_system::Trait>::Header;
//    type Event = <node_runtime::Runtime as srml_system::Trait>::Event;
//
//    type SignedExtra = (
//        srml_system::CheckGenesis<node_runtime::Runtime>,
//        srml_system::CheckEra<node_runtime::Runtime>,
//        srml_system::CheckNonce<node_runtime::Runtime>,
//        srml_system::CheckWeight<node_runtime::Runtime>,
//        srml_balances::TakeFees<node_runtime::Runtime>,
//    );
//    fn extra(nonce: Self::Index) -> Self::SignedExtra {
//        (
//            srml_system::CheckGenesis::<node_runtime::Runtime>::new(),
//            srml_system::CheckEra::<node_runtime::Runtime>::from(Era::Immortal),
//            srml_system::CheckNonce::<node_runtime::Runtime>::from(nonce),
//            srml_system::CheckWeight::<node_runtime::Runtime>::new(),
//            srml_balances::TakeFees::<node_runtime::Runtime>::from(0),
//        )
//    }
//}
//
//impl Balances for Runtime {
//    type Balance = <node_runtime::Runtime as srml_balances::Trait>::Balance;
//}
//
//type Index = <Runtime as System>::Index;
//type AccountId = <Runtime as System>::AccountId;
//type Address = <<Runtime as System>::Lookup as StaticLookup>::Source;
//type Balance = <Runtime as Balances>::Balance;

pub fn main() {
    // let mut rt = tokio::runtime::Runtime::new().unwrap();
//    let client = ClientBuilder::<Runtime>::new()
//        .set_url(Url::parse("ws://192.168.2.158:9944").unwrap())
//        .build()
//        .wait()
//        .unwrap();

   // println!("metadata is {:?}", client.metadata());
}
