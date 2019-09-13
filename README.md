# litentry-juniper-api

Api sever is designed as bridge between substrate blockchain and app/device. 
The server framework is juniper based on hyper.

Hyper is a fast, safe HTTP implementation written in and for Rust.
Hyper implement all request/response via "async IO" (non-blocking sockets) via 
the Tokio and Futures crates. Juniper is GraphQL query implementation in Rust, 
provide most efficient and most flexible query service.

For the interface to substrate part, most of code from substrate code base. 
Functionalists include get metadata, query storage, send transaction and subscription.


## how to deploy it.
1. install mysql and enable access from root.
2. create table by execute sql in github.com/litentry/litentry-juniper-api/db/src/table.sql
3. build bin by command `cargo build --release`
4. before run binary, substrate should be running and rpc port 9933 and ws port 9944 ready.
5. check the substrate via Polkadot UI, set the correct ws uri.
6. run target/release/litentry-juniper-api 
7. open url `127.0.0.1:3000/graphql`
8. all query template defined in github.com/litentry/litentry-juniper-api/db/src/graphql.txt

### Use Docker

```
docker pull juniuszhou/litentry-api:latest
docker run juniuszhou/litentry-api
```
