extern crate juniper;
use juniper::FieldResult;
use crate::model;
use model::{UsersData};
use crate::db::Database as Context;
//use super::Context;

pub struct Query;

graphql_object!(UsersData: Context |&self| {
    field id() -> i32 { self.id }
    field name() -> String { self.name.clone() }
});

graphql_object!(Query: Context |&self| {
  field user(&executor, id: i32) -> Option<UsersData> {
    let context = executor.context();
    println!("id is {}", id);
    context.user(id)
  }
});
