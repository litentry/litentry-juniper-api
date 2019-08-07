#![allow(missing_docs)]
extern crate juniper;

use litentry_substrate_rpc as rpc;
use litentry_juniper_database::Database as MysqlDatabase;
use crate::model::UsersData;

pub struct Database {
    mysql: MysqlDatabase,
}

impl Database {
    pub fn new() -> Database {
        let uri = "mysql://root:12345678@192.168.1.224:3306/mysql";
        let mysql = MysqlDatabase::new(&uri);
        Database {
            mysql,
        }
    }

    pub fn user(&self, id: i32) -> Option<UsersData> {
        println!("db users");

        let data = &self.mysql.get_users(id);

        let new_balance = rpc::get()

        if data.is_empty() {
            None
        } else {
            Some(UsersData{id, name: String::from(&data[0].name)})
        }
    }
}
