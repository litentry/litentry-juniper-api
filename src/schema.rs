extern crate juniper;

use crate::model;
use model::{UsersData, Identities, Tokens, SystemInfo, VerifyResult, TokenOwnerIdentity};
use crate::db::Database as Context;
use std::vec::Vec;

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

graphql_object!(SystemInfo: Context |&self| {
    field name() -> String { self.name.clone() }
    field version() -> String { self.version.clone() }
});

graphql_object!(VerifyResult: Context |&self| {
    field verify_result() -> bool { self.verify_result }
});

graphql_object!(TokenOwnerIdentity: Context |&self| {
    field token_hash() -> String { self.token_hash.clone() }
    field identity_hash() -> String { self.identity_hash.clone() }
    field owner_address() -> String { self.owner_address.clone() }
});

graphql_object!(Query: Context |&self| {
  field create_identity(&executor) -> Option<String> {
    let context = executor.context();
    context.create_identity()
  }

  field create_token(&executor, to: String, identity_hash: String, cost: String,
    data: String, data_type: String, expired: String) -> Option<String> {
    let context = executor.context();
    context.create_token(to, identity_hash, cost, data, data_type, expired)
  }

  field user(&executor, id: i32) -> Option<UsersData> {
    let context = executor.context();
    context.user(id)
  }
  field identity(&executor, id: i32) -> Option<Identities> {
    let context = executor.context();
    context.identity(id)
  }
  field token(&executor, id: i32) -> Option<Tokens> {
    let context = executor.context();
    context.token(id)
  }
  field owned_tokens(&executor, address: String) -> Vec<Tokens> {
    let context = executor.context();
    context.owned_tokens(&address)
  }
  field owned_identities(&executor, address: String) -> Vec<Identities> {
    let context = executor.context();
    context.owned_identities(&address)
  }
  field get_token_info(&executor, token_hash: String) -> Option<TokenOwnerIdentity> {
    let context = executor.context();
    context.get_token_info(&token_hash)
  }

  field verify_token(&executor, token_hash: String, signature: String, raw_data: String) -> VerifyResult {
    let context = executor.context();
    context.verify_token(&token_hash, &signature, &raw_data)
  }

  field system_info(&executor) -> Option<SystemInfo> {
    let context = executor.context();
    Some(SystemInfo {
        name: "hello".to_owned(),
        version: "world".to_owned(),
    })
  }
});
