#[derive(Queryable)]
pub struct Users {
    pub id: i32,
    pub name: String,
    pub address: String,
    pub public_key: String,
    pub balance: String,
}

#[derive(Queryable)]
pub struct Identities {
    pub id: i32,
    pub owner_id: i32,
    pub identity_hash: String,
}

#[derive(Queryable)]
pub struct Tokens {
    pub id: i32,
    pub owner_id: i32,
    pub identity_id: i32,
    pub token_hash: String,
    pub cost: String,
    pub data: String,
    pub data_type: String,
    pub expired: String,
}

#[derive(Queryable)]
pub struct TokensFull {
    pub id: i32,
    pub owner_id: i32,
    pub identity_id: i32,
    pub token_hash: String,
    pub cost: String,
    pub data: String,
    pub data_type: String,
    pub expired: String,
    pub identity_hash: String,
}

#[derive(Queryable)]
pub struct LitentryIndex {
    pub id: i32,
    pub identity_index: i32,
    pub token_index: i32,
}


#[derive(Queryable)]
pub struct NewIdentityEvent {
    pub id: i32,
    pub block_hash: String,
    pub address: String,
    pub identity_hash: String,
}

#[derive(Queryable)]
pub struct NewTokenEvent {
    pub id: i32,
    pub block_hash: String,
    pub owner_address: String,
    pub identity_hash: String,
    pub token_hash: String,
}



