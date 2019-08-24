#[derive(Serialize, Deserialize, Debug)]
pub struct UsersData {
    pub id: i32,
    pub name: String,
    pub address: String,
    pub balance: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Identities {
    pub id: i32,
    pub owner_id: i32,
    pub identity_hash: String,
}

#[derive(Serialize, Deserialize, Debug)]
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

pub struct TokenOwnerIdentity {
    pub token_hash: String,
    pub identity_hash: String,
    pub owner_address: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SystemInfo {
    pub name: String,
    pub version: String,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct VerifyResult {
    pub verify_result: bool,
}