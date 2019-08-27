use crate::db::Database;
use std::thread;
use std::sync::mpsc::channel;
use std::sync::Arc;

pub fn subscribe_sync(db: Arc<Database>, url: &str) {
    let (events_in, events_out) = channel();
    litentry_substrate_client::subscribe_events(url, events_in);
    thread::Builder::new().spawn( move || {
        loop {
            println!("start to try get events");
            let event_str = events_out.recv().unwrap();

            let _unhex = litentry_substrate_utils::hexstr_to_vec(event_str);
            let mut _er_enc = _unhex.as_slice();
            println!("raw message {:?}", &mut _er_enc);
            litentry_substrate_utils::decode_events(&mut _er_enc);
        }
    });
}
