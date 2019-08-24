extern crate failure;
#[macro_use]
extern crate failure_derive;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate juniper;

#[macro_use(bson, doc)]
extern crate bson;
extern crate mongodb;

extern crate warp;

use std::sync::Arc;
use warp::{filters::BoxedFilter, Filter};

mod db;
mod error;
mod gql;
mod model;

use db::Db;
use gql::{Mutations, Query};

#[derive(Clone)]
pub struct Context {
    pub db: Arc<Db>,
}

impl juniper::Context for Context {}
pub type Schema = juniper::RootNode<'static, Query, Mutations>;

fn main() {
    let ctx = Context {
        db: Arc::new(Db::new("mydb")),
    };
    let schema = Schema::new(Query, Mutations);

    let gql_index = warp::get2().and(warp::index()).and_then(web_index);
    let gql_query = make_graphql_filter("query", schema, ctx);

    let routes = gql_index.or(gql_query);
    warp::serve(routes).unstable_pipeline().run(([127, 0, 0, 1], 3030))
}

pub fn web_index() -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::http::Response::builder()
        .header("content-type", "application/json; charset=utf-8")
        .body(juniper::graphiql::graphiql_source("/query"))
        .expect("response is valid"))
}

pub fn make_graphql_filter<Query, Mutation, Context>(
    path: &'static str,
    schema: juniper::RootNode<'static, Query, Mutation>,
    ctx: Context,
) -> BoxedFilter<(impl warp::Reply,)>
    where
        Context: juniper::Context + Send + Sync + Clone + 'static,
        Query: juniper::GraphQLType<Context = Context, TypeInfo = ()> + Send + Sync + 'static,
        Mutation: juniper::GraphQLType<Context = Context, TypeInfo = ()> + Send + Sync + 'static,
{
    let schema = Arc::new(schema);

    let context_extractor = warp::any().map(move || -> Context { ctx.clone() });

    let handle_request =
        move |context: Context, request: juniper::http::GraphQLRequest| -> Result<Vec<u8>, serde_json::Error> {
            serde_json::to_vec(&request.execute(&schema, &context))
        };

    warp::post2()
        .and(warp::path(path.into()))
        .and(context_extractor)
        .and(warp::body::json())
        .map(handle_request)
        .map(build_response)
        .boxed()
}

fn build_response(response: Result<Vec<u8>, serde_json::Error>) -> warp::http::Response<Vec<u8>> {
    match response {
        Ok(body) => warp::http::Response::builder()
            .header("content-type", "application/json; charset=utf-8")
            .body(body)
            .expect("response is valid"),
        Err(_) => warp::http::Response::builder()
            .status(warp::http::StatusCode::INTERNAL_SERVER_ERROR)
            .body(Vec::new())
            .expect("status code is valid"),
    }
}


