use hex_literal::hex;
use substrate_primitives::{
    ed25519, sr25519, hexdisplay::HexDisplay, Pair, Public, blake2_256, sr25519::Public,
    crypto::{Ss58Codec, set_default_ss58_version, Ss58AddressFormat},
};
use parity_codec::{Encode, Decode};
use sr_primitives::generic::Era;
use node_primitives::{Balance, Index, Hash};
use node_runtime::{Call, UncheckedExtrinsic, BalancesCall, Runtime};
use substrate_keyring::{AccountKeyring, Ed25519Keyring, Sr25519Keyring};
use srml_system::{CheckEra, CheckNonce, CheckWeight};
use srml_balances::TakeFees;

pub fn get_tx() -> std::vec::Vec<u8> {
//    let extra = |i: Index, f: Balance| {
//        (
//            CheckEra::<Runtime>::from(Era::Immortal),
//            CheckNonce::<Runtime>::from(i),
//            CheckWeight::<Runtime>::from(),
//            TakeFees::<Runtime>::from(f),
//        )
//    };

    let alice: sr25519::Pair = Ed25519Keyring::Alice.into();
    let bob: sr25519::Pair = Ed25519Keyring::Bob.into();
    let to = sr25519::Public::from_raw(AccountKeyring::Alice.to_raw_public());

    let singer_secret = hex!["dcd1346701ca8396496e52aa2785b1748deb6db09551b72159dcb3e08991025b"];
    let signer = sr25519::Pair::from_seed(&singer_secret.into());
    let to_public = "5DAA5yYQ6qsjhrDGZxPTyxieoYfwNJCM9ATrpKEw3YiEbj8B";
    let to = sr25519::Public::from_string(&to_public).ok().unwrap();

    let amount = str::parse::<Balance>("1")
        .expect("Invalid 'amount' parameter; expecting an integer.");

    let index = str::parse::<Index>("0")
        .expect("Invalid 'amount' parameter; expecting an integer.");

    let function = Call::Balances(BalancesCall::transfer(to.into(), amount));

    let genesis_hash: Hash =  hex!["10c08714a10c7da78f40a60f6f732cf0dba97acfb5e2035445b032386157d5c3"].into();

    println!("Using a genesis hash of {}", HexDisplay::from(&genesis_hash.as_ref()));

    let raw_payload = (function, extra(index, 0), genesis_hash);
    let signature = raw_payload.using_encoded(|payload| if payload.len() > 256 {
        signer.sign(&blake2_256(payload)[..])
    } else {
        println!("Signing {}", HexDisplay::from(&payload));
        signer.sign(payload)
    });
    let extrinsic = UncheckedExtrinsic::new_signed(
        raw_payload.0,
        signer.public().into(),
        signature.into(),
        extra(index, 0),
    );
    println!("0x{}", hex::encode(&extrinsic.encode()));
    extrinsic.encode()
}

//fn main() {
//    get_tx();
//}

