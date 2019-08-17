extern crate futures;
extern crate hyper;
#[macro_use]
extern crate juniper;
extern crate juniper_hyper;
extern crate pretty_env_logger;
extern crate juniper_codegen;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

mod model;
mod schema;
mod db;
use db::Database;

use futures::future;
use hyper::rt::{self, Future};
use hyper::service::service_fn;
use hyper::Method;
use hyper::{Body, Response, Server, StatusCode};
use juniper::*;
use std::sync::Arc;

impl juniper::Context for Database {}

fn main() {
    pretty_env_logger::init();

    let addr = ([0, 0, 0, 0], 3000).into();

    let db = Arc::new(Database::new());
    let root_node = Arc::new(RootNode::new(schema::Query, EmptyMutation::<Database>::new()));

    let new_service = move || {
        let root_node = root_node.clone();
        let ctx = db.clone();
        service_fn(move |req| -> Box<dyn Future<Item = _, Error = _> + Send> {
            let root_node = root_node.clone();
            let ctx = ctx.clone();
            match (req.method(), req.uri().path()) {
                (&Method::GET, "/") => Box::new(juniper_hyper::graphiql("/graphql")),
                (&Method::GET, "/graphql") => {
                    println!("Method::GET");
                    Box::new(juniper_hyper::graphql(root_node, ctx, req))
                },
                (&Method::POST, "/graphql") => {
                    println!("Method::POST");
                    println!("{:?}", &req);
                    Box::new(juniper_hyper::graphql(root_node, ctx, req))
                }
                _ => {
                    println!("Method No match.");
                    let mut response = Response::new(Body::empty());
                    *response.status_mut() = StatusCode::NOT_FOUND;
                    Box::new(future::ok(response))
                }
            }
        })
    };
    let server = Server::bind(&addr)
        .serve(new_service)
        .map_err(|e| eprintln!("server error: {}", e));
    println!("Listening on http://{}", addr);

    rt::run(server);
}

