#[derive(Queryable)]
pub struct Users {
    pub id: i32,
    pub deposit: i32,
    pub round: i32,
}
