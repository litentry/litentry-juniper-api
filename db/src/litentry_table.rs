use diesel::prelude::*;
use models;
use schema::litentry_index::dsl::*;
use Database;

impl Database {
    pub fn get_litentry_index(&self) -> std::vec::Vec<models::LitentryIndex> {
        // just one record, each column for one index in runtime.
        litentry_index.filter(id.eq(0)).limit(1).load::<models::LitentryIndex>(&self.establish_connection()).expect("Error load Users.")
    }

    pub fn update_identity_index(&self, new_identity_index: i32) -> QueryResult<usize> {
        diesel::update(litentry_index.find(0))
            .set(identity_index.eq(new_identity_index))
            .execute(&self.establish_connection())
    }

    pub fn update_token_index(&self, new_token_index: i32) -> QueryResult<usize> {
        diesel::update(litentry_index.find(0))
            .set(token_index.eq(new_token_index))
            .execute(&self.establish_connection())
    }
}


