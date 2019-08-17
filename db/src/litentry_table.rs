
use diesel::prelude::*;
use models;
use schema::litentry_index::dsl::*;
use Database;

impl Database {
    pub fn get_litentry_index(&self) -> std::vec::Vec<models::LitentryIndex> {
        // just one record, each column for one index in runtime.
        litentry_index.filter(id.eq(0)).limit(1).load::<models::LitentryIndex>(&self.establish_connection()).expect("Error load Users.")
    }
}


