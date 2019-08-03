#![allow(missing_docs)]
extern crate juniper;
use litentry_juniper_database::Database as MysqlDatabase;
use crate::model::UsersData;

pub struct Database {
    mysql: MysqlDatabase,
}

impl Database {
    pub fn new() -> Database {
        let uri = "mysql://root:12345678@127.0.0.1:3306/mysql";
        let mysql = MysqlDatabase::new(&uri);
        Database {
            mysql,
        }
    }

    pub fn get_users(&self, id: i32) -> Option<UsersData> {
        println!("db get_users");
        Some(UsersData{id, name: String::from("hello")})
//        self.mysql.get_users(id).first().map(|u| UsersData{
//            id: u.id, name: u.name
//        })
    }

    pub fn user(&self, id: i32) -> Option<UsersData> {
        println!("db users");
        Some(UsersData{id, name: String::from("hello")})
//        self.mysql.get_users(id).first().map(|u| UsersData{
//            id: u.id, name: u.name
//        })
    }
}