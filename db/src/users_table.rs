
use diesel::prelude::*;
use models;
use schema::users::dsl::*;
use Database;

impl Database {
    pub fn get_users(&self, query_id: i32) -> std::vec::Vec<models::Users> {
        users.filter(id.eq(query_id)).limit(1).load::<models::Users>(&self.establish_connection()).expect("Error load Users.")
    }

    pub fn get_users_via_public_key(&self, public_key: String) -> std::vec::Vec<models::Users> {
        users.filter(id.eq(query_id)).limit(1).load::<models::Users>(&self.establish_connection()).expect("Error load Users.")
    }
}


