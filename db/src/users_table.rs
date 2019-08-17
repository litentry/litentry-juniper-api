
use diesel::prelude::*;
use models;
use schema::users::dsl::*;
use Database;

impl Database {
    pub fn get_users(&self, query_id: i32) -> std::vec::Vec<models::Users> {
        users.filter(id.eq(query_id)).limit(1).load::<models::Users>(&self.establish_connection()).expect("Error load Users.")
    }

    pub fn get_users_via_public_key(&self, query_key: &str) -> std::vec::Vec<models::Users> {
        users.filter(public_key.like(query_key)).limit(1).load::<models::Users>(&self.establish_connection()).expect("Error load Users.")
    }
}


