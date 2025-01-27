use crate::db::Database;
use std::thread;
use std::sync::mpsc::channel;
use std::sync::Arc;
use serde_json::{json};
use std::sync::mpsc::Sender as ThreadOut;
use ws::{connect, Handler, Handshake, Message, Result, Sender};
use litentry_runtime::LitentryEvents;

pub type OnMessageFn = fn(msg: Message, out: Sender, result: ThreadOut<String>) -> Result<()>;

pub struct RpcClient {
    pub out: Sender,
    pub request: String,
    pub result: ThreadOut<String>,
    pub on_message_fn: OnMessageFn,
}

impl Handler for RpcClient {
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        self.out.send(self.request.clone()).unwrap();
        Ok(())
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        (self.on_message_fn)(msg, self.out.clone(), self.result.clone())
    }
}

pub fn on_subscription_msg(msg: Message, _out: Sender, result: ThreadOut<String>) -> Result<()> {
    println!("on_subscription_msg");
    let retstr = msg.as_text().unwrap();
    result.send(retstr.to_owned()).unwrap();
//    let value: serde_json::Value = serde_json::from_str(retstr).unwrap();
//    match value["id"].as_str() {
//        Some(_idstr) => {},
//        _ => {
//
//            match value["method"].as_str() {
//                Some("state_storage") => {
//                    let _changes = &value["params"]["result"]["changes"];
//                    let _res_str = _changes[0][1].as_str().unwrap().to_string();
//                    result.send(_res_str).unwrap();
//                }
//                _ => println!("unsupported method"),
//            }
//        },
//    };
    Ok(())
}

pub fn subscribe_sync(db: Arc<Database>, url: &str) {
    let (events_in, events_out) = channel();
    let key = litentry_substrate_utils::twox_storage_key_hash("System", "Events");
    let jsonreq = json!({
            "method": "state_subscribeStorage",
            "params": [[key]],
            "jsonrpc": "2.0",
            "id": "1",
        }).to_string();
    println!("start rpc thread.");
    start_rpc_client_thread(url.to_owned(), jsonreq, events_in, on_subscription_msg);

    let _ = thread::Builder::new().spawn(move || {
        loop {
            println!("start to try get events");
            let event_str = events_out.recv().unwrap();

            let value: serde_json::Value = serde_json::from_str(&event_str).unwrap();
            if value["id"].as_str().is_some() || value["method"].as_str() != Some("state_storage") {
                continue;
            }
            let block = &value["params"]["result"]["block"].as_str().unwrap();
            let _changes = &value["params"]["result"]["changes"];
            // format is [[key, value]]
            let _res_str = _changes[0][1].as_str().unwrap().to_string();


            let _unhex = litentry_substrate_utils::hexstr_to_vec(_res_str);
            let mut _er_enc = _unhex.as_slice();
            println!("raw message {:?}", &mut _er_enc);


            let events = litentry_substrate_utils::decode_events(&mut _er_enc);
            for event in events {
                match event {
                    LitentryEvents::NewIdentity(_, address, hash) => db.mysql.insert_new_identity_event(block, &address, &hash),
                    LitentryEvents::NewToken(_, address, identity, token) => db.mysql.insert_new_token_event(block, &address, &identity, &token),
                };
            }
        }
    });
}


fn start_rpc_client_thread(url: String,
                           jsonreq: String,
                           result_in: ThreadOut<String>,
                           on_message_fn: OnMessageFn) {
    let _ = thread::Builder::new()
        .name("client".to_owned())
        .spawn(move || {
            connect(url, |out| {
                RpcClient {
                    out,
                    request: jsonreq.clone(),
                    result: result_in.clone(),
                    on_message_fn,
                }
            }).unwrap()
        }).unwrap();
}

