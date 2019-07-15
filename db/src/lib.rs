#[macro_use]
extern crate diesel;

pub mod models;
pub mod schema;

use diesel::prelude::*;
use std::env;
use schema::users;
use models::*;
use schema::users::dsl::*;
use diesel::insert_into;

struct Database {
    conn: MysqlConnection,
}

impl Database {

    // "mysql://root:123456@127.0.0.1:3306/mysql"
    pub fn establish_connection(url: &str) -> MysqlConnection {
        MysqlConnection::establish(url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", url))
    }

    pub fn new(url: &str) -> Self {
        Database {
            conn: Database::establish_connection(url)
        }
    }

    pub fn insert_users(&self, new_id: i32, new_deposit: i32, new_round: i32) -> QueryResult<usize> {
        insert_into(users).values(
            (id.eq(new_id), deposit.eq(new_deposit), round.eq(new_round)))
            .execute(&self.conn)
    }

    pub fn delete_users(&self, delete_id: i32) -> QueryResult<usize> {
        diesel::delete(users.filter(id.eq(delete_id)))
        .execute(&self.conn)
        // .expect("Error deleting posts")
    }

    pub fn update_users(&self, update_id: i32, update_deposit: i32, update_round: i32) -> QueryResult<usize> {
        diesel::update(users.find(update_id))
        .set(deposit.eq(update_deposit))
        .execute(&self.conn)
    }

    pub fn get_users(&self) -> std::vec::Vec<models::Users> {
        users.filter(id.eq(1)).limit(3).load::<Users>(&self.conn).expect("Error load Users.")
    }
}



