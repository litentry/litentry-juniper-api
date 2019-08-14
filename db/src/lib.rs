#[macro_use]
extern crate diesel;

pub mod models;
pub mod schema;
use diesel::prelude::*;
use models::*;
use schema::users::dsl::*;

pub struct Database {
     url: String,
}

impl Database {

    // "mysql://root:123456@127.0.0.1:3306/mysql"
    pub fn establish_connection(&self) -> MysqlConnection {
        MysqlConnection::establish(&(&self.url))
            .unwrap_or_else(|_| panic!("Error connecting to {}", &self.url))
    }

    pub fn new(url: &str) -> Self {
        Database {url: String::from(url)}
    }

    pub fn get_users(&self, user_id: i32) -> std::vec::Vec<models::Users> {
        users.filter(id.eq(user_id)).limit(1).load::<Users>(&self.establish_connection()).expect("Error load Users.")
    }

    pub fn get_identities(&self, identity_id: i32) -> std::vec::Vec<models::Identities> {
        users.filter(id.eq(identity_id)).limit(1).load::<Identities>(&self.establish_connection()).expect("Error load Identities.")
    }

    pub fn get_tokens(&self, token_id: i32) -> std::vec::Vec<models::Tokens> {
        users.filter(id.eq(token_id)).limit(1).load::<Tokens>(&self.establish_connection()).expect("Error load Tokens.")
    }

    pub fn get_litentry_index(&self, user_id: i32) -> std::vec::Vec<models::LitentryIndex> {
        users.filter(id.eq(user_id)).limit(1).load::<LitentryIndex>(&self.establish_connection()).expect("Error load LitentryIndex.")
    }

//    pub fn insert_users(&self, new_id: i32, new_deposit: i32, new_round: i32) -> QueryResult<usize> {
//        insert_into(users).values(
//            (id.eq(new_id), deposit.eq(new_deposit), round.eq(new_round)))
//            .execute(&self.establish_connection())
//    }
//
//    pub fn delete_users(&self, delete_id: i32) -> QueryResult<usize> {
//        diesel::delete(users.filter(id.eq(delete_id)))
//        .execute(&self.establish_connection())
//        // .expect("Error deleting posts")
//    }
//
//    pub fn update_users(&self, update_id: i32, update_deposit: i32, update_round: i32) -> QueryResult<usize> {
//        diesel::update(users.find(update_id))
//        .set(deposit.eq(update_deposit))
//        .execute(&self.establish_connection())
//    }


}


