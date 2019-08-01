extern crate substrate_transaction_pool;
extern crate substrate_test_runtime;

use substrate_transaction_pool::txpool::ExtrinsicFor;
use node_runtime::{Address, Block, Header, SignedBlock};
use substrate_test_runtime::{Transfer, Keyring, Extrinsic};
use substrate_keyring::{AccountKeyring};

pub fn test() {
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

    let xt = signed_transfer.encode();
    xt

//    println!("{}", signed_transfer.());
}

fn uxt(who: AccountKeyring, nonce: Index) -> Extrinsic {
    let transfer = Transfer {
        from: who.into(),
        to: AccountId::default(),
        nonce,
        amount: 1,
    };
    let signature = transfer.using_encoded(|e| who.sign(e));
    Extrinsic::Transfer(transfer, signature.into())
}


