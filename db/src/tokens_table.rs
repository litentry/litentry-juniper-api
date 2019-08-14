
use diesel::prelude::*;
use models;
use schema::tokens::dsl::*;
use Database;

impl Database {
    pub fn get_tokens(&self, query_id: i32) -> std::vec::Vec<models::Tokens> {
        tokens.filter(id.eq(query_id)).limit(1).load::<models::Tokens>(&self.establish_connection()).expect("Error load Users.")
    }
}


