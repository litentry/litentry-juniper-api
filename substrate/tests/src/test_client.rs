#[macro_use]
extern crate jsonrpc_core_client;
extern crate futures;


use jsonrpc_core_client::{
    transports::http,
    transports::ws,
    RpcError,
    TypedClient,
    RpcChannel,
};
use futures::Future;

#[derive(Clone)]
struct TestClient(TypedClient);

impl From<RpcChannel> for TestClient {
    fn from(channel: RpcChannel) -> Self {
        TestClient(channel.into())
    }
}

impl TestClient {
    fn hello(&self, msg: &'static str) -> impl Future<Item = String, Error = RpcError> {
        self.0.call_method("system_name", "String", ())
    }
    fn fail(&self) -> impl Future<Item = (), Error = RpcError> {
        self.0.call_method("fail", "()", ())
    }
}

fn run() {
    println!("start all tests");
    let uri = "ws://192.168.2.158:9944";
    http::connect(uri)
        .and_then(|client: TestClient| {
            client.hello("http").and_then(move |result| {
                println!("result is {}", result);
                Ok(())
            })
        })
        .map_err(|e| {
            println!("Error: {:?}", e);
        });

    std::thread::sleep_ms(2000);
}

