#[macro_use]
extern crate diesel;

pub mod models;
pub mod schema;
pub mod users_table;
pub mod identities_table;
pub mod tokens_table;
pub mod litentry_table;
pub mod event_table;

use diesel::prelude::*;

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


//    pub fn insert_users(&self, new_id: i32, new_deposit: i32, new_round: i32) -> QueryResult<usize> {
//        diesel::insert_into(users).values(
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


