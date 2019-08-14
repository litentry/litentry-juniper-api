extern crate juniper;
use juniper::FieldResult;
use crate::model;
use model::{UsersData, Identities, Tokens};
use crate::db::Database as Context;

pub struct Query;

graphql_object!(UsersData: Context |&self| {
    field id() -> i32 { self.id }
    field name() -> String { self.name.clone() }
    field address() -> String { self.address.clone() }
    field balance() -> String { self.balance.clone() }

});

graphql_object!(Identities: Context |&self| {
    field id() -> i32 { self.id }
    field owner_id() -> i32 { self.owner_id }
    field identity_hash() -> String { self.identity_hash.clone() }
});

graphql_object!(Tokens: Context |&self| {
    field id() -> i32 { self.id }
    field owner_id() -> i32 { self.owner_id }
    field identity_id() -> i32 { self.identity_id }
    field token_hash() -> String { self.token_hash.clone() }
    field cost() -> String { self.cost.clone() }
    field data() -> String { self.data.clone() }
    field data_type() -> String { self.data_type.clone() }
    field expired() -> String { self.expired.clone() }
});

graphql_object!(Query: Context |&self| {
  field user(&executor, id: i32) -> Option<UsersData> {
    let context = executor.context();
    context.user(id)
  }
});

graphql_object!(Query: Context |&self| {
  field identity(&executor, id: i32) -> Option<Identities> {
    let context = executor.context();
    context.identity(id)
  }
});

graphql_object!(Query: Context |&self| {
  field token(&executor, id: i32) -> Option<Tokens> {
    let context = executor.context();
    context.token(id)
  }
});
