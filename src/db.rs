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
        let uri = "mysql://root:12345678@192.168.2.158:3306/litentry";
        let mysql = MysqlDatabase::new(&uri);
        Database {
            mysql,
        }
    }

    pub fn user(&self, id: i32) -> Option<UsersData> {
        println!("db users");

        let data = &self.mysql.get_users(id);
        if data.is_empty() {
            println!("not found.");
            return None;
        }

        let new_balance = rpc::get_balance(&data[0].address);
        println!("query balance in chain.");

        match new_balance {
            Some(balance) => Some(UsersData {
                id,
                name: String::from(&data[0].name),
                address: String::from(&data[0].address),
                balance: String::from(balance.to_string()),
            }),
            None => Some(UsersData {
                id,
                name: String::from(&data[0].name),
                address: String::from(&data[0].address),
                balance: String::from(&data[0].balance),
            }),
        }
    }
}
