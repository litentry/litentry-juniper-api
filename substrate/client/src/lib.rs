extern crate substrate_api_client;
extern crate codec;
extern crate primitives;
extern crate log;
extern crate node_runtime;
extern crate node_primitives;
extern crate system;
extern crate srml_balances;

use substrate_api_client::{
    Api,
    crypto::{AccountKey, CryptoKind},
    extrinsic,
};

pub struct LitentryClient {
    pub url: String,
}

impl LitentryClient {
    pub fn new(sub_url: &str) -> Self {
        LitentryClient {
            url: sub_url.to_owned(),
        }
    }

    pub fn create_identity(&self) {
        let from = AccountKey::new("//Alice", Some(""), CryptoKind::Sr25519);
        let api = Api::new(self.url.to_owned()).set_signer(from.clone());

        let xt = extrinsic::litentry::register_identity(
            api.clone(),
        );

        println!("Sending an extrinsic from Alice (Key = {:?}) to create new identity\n", from.public());
        println!("[+] Composed extrinsic: {:?}\n", xt);

        //send and watch extrinsic until finalized
        let tx_hash = api.send_extrinsic(xt.hex_encode()).unwrap();
        println!("[+] Transaction got finalized. Hash: {:?}\n", tx_hash);
    }
}

//pub fn subscribe_events(url: &str, sender: Sender<String>) {
//    let api = Api::new(format!("ws://{}", url));
//    api.subscribe_events(sender.clone());
//}