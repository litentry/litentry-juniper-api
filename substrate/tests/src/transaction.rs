extern crate substrate_transaction_pool;
extern crate substrate_test_runtime;

use substrate_transaction_pool::txpool::ExtrinsicFor;
use node_runtime::{Address, Block, Header, SignedBlock};
use substrate_test_runtime::{Transfer, AccountKeyring, Keyring};

fn test() {
    let account = AccountKeyring::from(Keyring::Alice);

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


