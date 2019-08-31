//#![feature(async_await)]
////#![deny(warnings)]
//extern crate hyper;
//extern crate tokio;
//extern crate futures;
//use futures::Future;
//
//use hyper::{Body, Response, Server, Request, Method};
//use hyper::service::service_fn_ok;
//
//fn run_server() {
//    pretty_env_logger::init();
//
//    let addr = ([127, 0, 0, 1], 3000).into();
//    let new_service = || {
//        service_fn_ok(|req| {
//            let (method, uri, _, _headers, body) = req.deconstruct();
////            let headers = request.headers();
////            let (parts, body) = request.into_parts();
////            let response = if parts.method == Method::GET {
////                "Hello"
////            } else {
////                "World"
////            };
//
//            Response::new(Body::from("hello"))
//        })
//    };
//
//    let server = Server::bind(&addr)
//        .serve(new_service);
//    let (_tx, rx) = futures::sync::oneshot::channel::<()>();
//
//    let graceful = server
//        .with_graceful_shutdown(rx)
//        .map_err(|err| eprintln!("server error: {}", err));;
//    println!("Listening on http://{}", addr);
//    tokio::run(graceful);
//
//}
//
//
