extern crate futures;
extern crate tokio;
extern crate websocket;

use futures::future::Future;
use futures::sink::Sink;
use futures::stream::Stream;
use futures::sync::mpsc;
use std::io::stdin;
use std::thread;
use websocket::result::WebSocketError;
use websocket::{ClientBuilder, OwnedMessage};

const CONNECTION: &'static str = "ws://127.0.0.1:9944";

fn build_ws_client(receiver: mpsc::Receiver<OwnedMessage>) {
    let mut runtime = tokio::runtime::current_thread::Builder::new()
        .build()
        .unwrap();

    let runner = ClientBuilder::new(CONNECTION)
        .unwrap()
        .add_protocol("rust-websocket")
        .async_connect_insecure()
        .and_then(|(duplex, _)| {
            // split duplex to sender and receiver.
            let (sink, stream) = duplex.split();
            stream
                .filter_map(|message| {
                    println!("Received Message: {:?}", message);
                    // map message to option.
                    match message {
                        OwnedMessage::Close(e) => Some(OwnedMessage::Close(e)),
                        OwnedMessage::Ping(d) => Some(OwnedMessage::Pong(d)),
                        _ => None,
                    }
                })
                // receive data from stdin channel.
                .select(receiver.map_err(|_| WebSocketError::NoDataAvailable))
                // send message to ws server.
                .forward(sink)
        });
    runtime.block_on(runner).unwrap();
}

fn main() {
    let (usr_msg, stdin_ch) = mpsc::channel(0);
    thread::spawn(|| {
        let mut input = String::new();
        let mut stdin_sink = usr_msg.wait();
        loop {
            input.clear();
            stdin().read_line(&mut input).unwrap();
            let trimmed = input.trim();

            // send message via stdin.
            let (close, msg) = match trimmed {
                "/close" => (true, OwnedMessage::Close(None)),
                "/ping" => (false, OwnedMessage::Ping(b"PING".to_vec())),
                _ => (false, OwnedMessage::Text(trimmed.to_string())),
            };

            stdin_sink
                // send message to ws client
                .send(msg)
                .expect("Sending message across stdin channel.");

            if close {
                break;
            }
        }
    });

    build_ws_client(stdin_ch);
}


