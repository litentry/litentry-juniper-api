use diesel::prelude::*;
use models;
use schema;
use Database;
use schema::new_identity_event::dsl::*;
use schema::new_token_event::dsl::*;

impl Database {
    pub fn get_new_identity_events(&self) -> std::vec::Vec<models::NewIdentityEvent> {
        schema::new_identity_event::dsl::new_identity_event.load::<models::NewIdentityEvent>(&self.establish_connection()).expect("Error load Users.")
    }

    pub fn get_new_token_events(&self) -> std::vec::Vec<models::NewTokenEvent> {
        schema::new_token_event::dsl::new_token_event.load::<models::NewTokenEvent>(&self.establish_connection()).expect("Error load Users.")
    }

    pub fn insert_new_identity_event(&self, new_block_hash: &str, new_address: &str, new_identity_hash: &str) -> QueryResult<usize> {
        diesel::insert_into(schema::new_identity_event::dsl::new_identity_event).values((
            (schema::new_identity_event::dsl::block_hash.eq(new_block_hash)),
            (schema::new_identity_event::dsl::address.eq(new_address)),
            (schema::new_identity_event::dsl::identity_hash.eq(new_identity_hash))))
            .execute(&self.establish_connection())
    }

    pub fn insert_new_token_event(&self, new_block_hash: &str, new_owner_address: &str, new_identity_hash: &str, new_token_hash: &str) -> QueryResult<usize> {
        diesel::insert_into(schema::new_token_event::dsl::new_token_event).values((
            (schema::new_token_event::dsl::block_hash.eq(new_block_hash)),
            (schema::new_token_event::dsl::owner_address.eq(new_owner_address)),
            (schema::new_token_event::dsl::identity_hash.eq(new_identity_hash)),
            (schema::new_token_event::dsl::token_hash.eq(new_token_hash))))
            .execute(&self.establish_connection())
    }
}
