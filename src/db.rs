#![allow(missing_docs)]
extern crate juniper;

use litentry_substrate_rpc as rpc;
use litentry_juniper_database::Database as MysqlDatabase;
use crate::model::{UsersData, Identities, Tokens};
use std::collections::HashMap;
use std::cell::RefCell;
use std::sync::Arc;
use std::sync::Mutex;

pub struct Database {
    mysql: MysqlDatabase,
    //TODO now diesel not support u64, should upgrade to u64 after diesel can.
    index_map: Arc<Mutex<RefCell<HashMap<String, i32>>>>,
}

impl Database {
    pub fn new() -> Database {
        let uri = "mysql://root:12345678@192.168.2.158:3306/litentry";
        let mysql = MysqlDatabase::new(&uri);
        let index = mysql.get_litentry_index();
        let index_map = Arc::new(Mutex::new(RefCell::new(HashMap::new())));
        if index.len() > 0 {
            index_map.lock().unwrap().borrow_mut().insert("identities".to_owned(), index[0].identity_index);
            index_map.lock().unwrap().borrow_mut().insert("tokens".to_owned(), index[0].token_index);
        } else {
            index_map.lock().unwrap().borrow_mut().insert("identities".to_owned(), -1);
            index_map.lock().unwrap().borrow_mut().insert("tokens".to_owned(), -1);
        }

        // let index_map = RefCell::new(HashMap::<String, u64>::new());
        Database {
            mysql,
            index_map,
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

    pub fn identity(&self, id: i32) -> Option<Identities> {
        let count_in_chain = rpc::get_identity_count();
        if count_in_chain.is_some() {
            let count_in_db =  self.index_map.into_inner().unwrap().borrow().get("identities").unwrap();
            let count_in_db = *count_in_db as u64;
            let count_in_chain = count_in_chain.unwrap();
            if count_in_chain > count_in_db {
                for index in count_in_db..count_in_chain {
                    let new_identity = rpc::get_identity_via_index(index);

                }
            }
        }

        None
    }

    pub fn token(&self, id: i32) -> Option<Tokens> {
        None
    }
}
