#[derive(Queryable)]
pub struct Users {
    pub id: i32,
    pub name: String,
    pub address: String,
    pub balance: String,
}
