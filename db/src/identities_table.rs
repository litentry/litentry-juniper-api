
use diesel::prelude::*;
use models;
use schema::identities::dsl::*;
use Database;

impl Database {
    pub fn get_identities(&self, query_id: i32) -> std::vec::Vec<models::Identities> {
        identities.filter(id.eq(query_id)).limit(1).load::<models::Identities>(&self.establish_connection()).expect("Error load Users.")
    }
}


