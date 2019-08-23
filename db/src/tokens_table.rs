
use diesel::prelude::*;
use models;
use schema::tokens::dsl::*;
use schema;
use Database;

impl Database {
    pub fn get_tokens(&self, query_id: i32) -> std::vec::Vec<models::Tokens> {
        tokens.filter(id.eq(query_id)).limit(1).load::<models::Tokens>(&self.establish_connection()).expect("Error load Users.")
    }

    pub fn insert_tokens(&self, new_id: i32, new_owner_id: i32, new_identity_id: i32, new_hash: &str,
        new_cost: &str, new_data: &str, new_data_type: &str, new_expired: &str) -> QueryResult<usize> {
        diesel::insert_into(tokens).values(
            (id.eq(new_id), owner_id.eq(new_owner_id), identity_id.eq(new_identity_id),
             token_hash.eq(new_hash), cost.eq(new_cost), data.eq(new_data), data_type.eq(new_data_type),
            expired.eq(new_expired)))
            .execute(&self.establish_connection())
    }

    pub fn get_tokens_via_owner_id(&self, query_id: i32) -> std::vec::Vec<models::Tokens> {
        tokens.filter(owner_id.eq(query_id)).load::<models::Tokens>(&self.establish_connection()).expect("Error load Users.")
    }

    pub fn get_tokens_identity_owner_via_id(&self, query_id: i32) -> std::vec::Vec<(models::Tokens, models::Identities, models::Users)> {
        tokens.filter(id.eq(query_id))
            .inner_join(schema::identities::dsl::identities.on(identity_id.eq(schema::identities::dsl::id)))
            .inner_join(schema::users::dsl::users.on(owner_id.eq(schema::users::dsl::id)))
            .load::<(models::Tokens, models::Identities, models::Users)>(&self.establish_connection()).expect("Error load Users.")
    }

    pub fn get_tokens_identity_owner_via_hash(&self, query_hash: &str) -> std::vec::Vec<(models::Tokens, models::Identities, models::Users)> {
        tokens.filter(token_hash.like(query_hash))
            .inner_join(schema::identities::dsl::identities.on(identity_id.eq(schema::identities::dsl::id)))
            .inner_join(schema::users::dsl::users.on(owner_id.eq(schema::users::dsl::id)))
            .load::<(models::Tokens, models::Identities, models::Users)>(&self.establish_connection()).expect("Error load Users.")
    }
}