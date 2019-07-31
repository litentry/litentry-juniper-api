extern crate juniper;
use crate::model;
use model::{Database, UsersData};
use juniper::Context;

pub struct Query;

graphql_object!(Query: Context |&self| {
  field user(&executor, id: String) -> FieldResult<Option<UsersData>> {
    let context = executor.context();
    Ok(context.db.get_users(&id)?)
  }
});
