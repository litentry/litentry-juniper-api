use diesel::prelude::*;
use models;
use schema::identities::dsl::*;
use Database;

impl Database {
    pub fn get_identities(&self, query_id: i32) -> std::vec::Vec<models::Identities> {
        identities.filter(id.eq(query_id)).limit(1).load::<models::Identities>(&self.establish_connection()).expect("Error load Users.")
    }

    pub fn get_identities_via_hash(&self, query_hash: &str) -> std::vec::Vec<models::Identities> {
        identities.filter(identity_hash.like(query_hash)).limit(1).load::<models::Identities>(&self.establish_connection()).expect("Error load Users.")
    }

    pub fn insert_identities(&self, new_id: i32, new_owner_id: i32, new_hash: &str) -> QueryResult<usize> {
        diesel::insert_into(identities).values(
            (id.eq(new_id), owner_id.eq(new_owner_id), identity_hash.eq(new_hash)))
            .execute(&self.establish_connection())
    }
}


