
use diesel::prelude::*;
use models;
use schema::users::dsl::*;
use Database;

impl Database {
    pub fn get_users(&self, _user_id: i32) -> std::vec::Vec<models::Users> {
        users.filter(id.eq(_user_id)).limit(1).load::<models::Users>(&self.establish_connection()).expect("Error load Users.")
    }
}


