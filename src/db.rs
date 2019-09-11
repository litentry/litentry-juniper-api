#![allow(missing_docs)]
extern crate juniper;

use litentry_substrate_rpc as rpc;
use litentry_juniper_database::Database as MysqlDatabase;
use crate::model::{UsersData, Identities, Tokens, VerifyResult, TokenOwnerIdentity};
use std::collections::HashMap;
use std::cell::RefCell;
use std::sync::Arc;
use std::sync::Mutex;
use std::vec::Vec;

pub struct Database {
    pub mysql: MysqlDatabase,
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
        self.sync_identities();

        let data = &self.mysql.get_identities(id);
        if data.is_empty() {
            println!("identity not found.");
            None
        } else {
            Some( Identities {
                id: data[0].id,
                owner_id: data[0].owner_id,
                identity_hash: String::from(&data[0].identity_hash),
            })
        }
    }

    pub fn sync_identities(&self) {
        let count_in_chain = rpc::get_identity_count();
        if count_in_chain.is_some() {
            let count_in_db =  self.index_map.lock().unwrap().borrow().get("identities").unwrap().to_owned();
            let count_in_chain = count_in_chain.unwrap() as i32;
            println!("sync identities count_in_db is {} count_in_chain {}", count_in_db, count_in_chain);
            if count_in_chain > count_in_db {
                let begin = count_in_db + 1;
                for index in begin..count_in_chain {
                    let new_identity = rpc::get_identity_via_index(index);
                    if new_identity.is_some() {
                        let identity = new_identity.unwrap();
                        let owner_public_key = rpc::get_identity_owner_via_hash(&identity);
                        if owner_public_key.is_some() {
                            let public_key = owner_public_key.unwrap();
                            let owner_id = &self.mysql.get_users_via_public_key(&public_key);
                            println!("insert new identity. index {} owner_id {} hash {:?}", index, owner_id[0].id, &identity);
                            let insert_result = &self.mysql.insert_identities(index, owner_id[0].id, &identity);
                            match insert_result {
                                Ok(_) => {
                                    // update last index in map.
                                    println!("update index map for identity in memory.");
                                    &self.index_map.lock().unwrap().borrow_mut().insert("identities".to_owned(), index);
                                    match &self.mysql.update_identity_index(index) {
                                        Ok(_) => {println!("update identity index in mysql successfully.")},
                                        Err(info) => {
                                            println!("failed update identity index in mysql with {:?}", info);
                                            break;
                                        }
                                    }
                                },
                                Err(info) => {
                                    println!("sql error info is {:?}.", info);
                                    break;
                                }
                            }
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
            }
        }
    }

    pub fn token(&self, id: i32) -> Option<Tokens> {
        &self.sync_tokens();
        let data = &self.mysql.get_tokens(id);
        if data.is_empty() {
            println!("token not found.");
            None
        } else {
            Some(Tokens{
                id: data[0].id,
                owner_id: data[0].owner_id,
                identity_id: data[0].identity_id,
                token_hash: String::from(&data[0].token_hash),
                cost: String::from(&data[0].cost),
                data: String::from(&data[0].data),
                data_type: String::from(&data[0].data_type),
                expired: String::from(&data[0].expired),
            })
        }
    }

    pub fn owned_tokens(&self, address: &str) -> Vec<Tokens> {
        &self.sync_tokens();
        let mut result: Vec<Tokens> = vec![];
        let owners = &self.mysql.get_users_via_address(address);
        if owners.len() > 0 {
            let tokens = &self.mysql.get_tokens_via_owner_id(owners[0].id);
            for token in tokens {
                result.push(Tokens{
                    id: token.id,
                    owner_id: token.owner_id,
                    identity_id: token.identity_id,
                    token_hash: String::from(&token.token_hash),
                    cost: String::from(&token.cost),
                    data: String::from(&token.data),
                    data_type: String::from(&token.data_type),
                    expired: String::from(&token.expired),
                })
            }
        }
        result
    }

    pub fn owned_identities(&self, address: &str) -> Vec<Identities> {
        &self.sync_identities();
        let mut result: Vec<Identities> = vec![];
        let owners = &self.mysql.get_users_via_address(address);
        if owners.len() > 0 {
            let identities = &self.mysql.get_identities_via_owner_id(owners[0].id);
            for identity in identities {
                result.push(Identities{
                    id: identity.id,
                    owner_id: identity.owner_id,
                    identity_hash: String::from(&identity.identity_hash),
                })
            }
        }
        result
    }

    pub fn verify_token(&self, token_hash: &str, signature: &str, raw_data: &str) -> VerifyResult {
        let identity_owner = &self.mysql.get_tokens_identity_owner_via_hash(token_hash);

        if identity_owner.len() > 0 {
            VerifyResult {
                verify_result: litentry_substrate_utils::verify_signature(&identity_owner[0].2.public_key, signature, raw_data)
            }
        } else {
            println!("token owner not found in db.");
            VerifyResult {
                verify_result: false
            }
        }
    }

    pub fn get_token_info(&self, token_hash: &str) -> Option<TokenOwnerIdentity> {
        let identity_owner = &self.mysql.get_tokens_identity_owner_via_hash(token_hash);
        if identity_owner.len() > 0 {
            Some(TokenOwnerIdentity {
                token_hash: token_hash.to_owned(),
                identity_hash: identity_owner[0].1.identity_hash.to_owned(),
                owner_address: identity_owner[0].2.address.to_owned(),
            })
        } else {
            None
        }
    }

    pub fn sync_tokens(&self) {
        let count_in_chain = rpc::get_token_count();
        if count_in_chain.is_some() {
            let count_in_db =  self.index_map.lock().unwrap().borrow().get("tokens").unwrap().to_owned();
            let count_in_chain = count_in_chain.unwrap() as i32;
            println!("sync tokens count_in_db is {} count_in_chain {}", count_in_db, count_in_chain);
            if count_in_chain > count_in_db {
                let begin = count_in_db + 1;
                for index in begin..count_in_chain {
                    let new_token_hash = rpc::get_token_hash_via_index(index);
                    if new_token_hash.is_some() {
                        let new_token = rpc::get_token_via_hash(&new_token_hash.unwrap());
                        if new_token.is_some() {
                            let token = new_token.unwrap();
                            let owner_public_key = rpc::get_token_owner_via_hash(&token.0);
                            if owner_public_key.is_some() {
                                let public_key = owner_public_key.unwrap();
                                let owner_id = &self.mysql.get_users_via_public_key(&public_key);
                                let token_identity = rpc::get_token_identity_via_hash(&token.0);
                                if token_identity.is_some() {
                                    let identity = token_identity.unwrap();
                                    let identity_id = &self.mysql.get_identities_via_hash(&identity);
                                    println!("insert new token. index {} owner_id {} identity_id {} token hash {} balance {}",
                                             index, owner_id[0].id, identity_id[0].id, &token.0, &token.1);

                                    let insert_result = &self.mysql.insert_tokens(index, owner_id[0].id, identity_id[0].id,
                                                                                  &token.0, &token.1, &token.2, &token.3, &token.4);
                                    match insert_result {
                                        Ok(_) => {
                                            // update last index in map.
                                            println!("update index map for tokens in memory.");
                                            &self.index_map.lock().unwrap().borrow_mut().insert("tokens".to_owned(), index);
                                            match &self.mysql.update_token_index(index) {
                                                Ok(_) => {println!("update token index in mysql successfully")},
                                                Err(info) => {
                                                    println!("failed update token index in mysql with {:?}", info);
                                                    break;
                                                }
                                            }
                                        },
                                        Err(info) => {
                                            println!("sql error info is {:?}.", info);
                                            break;
                                        }
                                    }
                                }
                            } else {
                                break;
                            }
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
            }
        }
    }
}
