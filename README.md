# litentry-juniper-api

Api sever is designed as bridge between substrate blockchain and app/device. 
The server framework is juniper based on hyper.

Hyper is a fast, safe HTTP implementation written in and for Rust.
Hyper implement all request/response via "async IO" (non-blocking sockets) via 
the Tokio and Futures crates. Juniper is GraphQL query implementation in Rust, 
provide most efficient and most flexible query service.

For the interface to substrate part, most of code from substrate code base. 
Functionalists include get metadata, query storage, send transaction and subscription.


