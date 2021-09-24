#[derive(Queryable, Debug)]
pub struct Beer {
    pub id: i64,
    pub chat_id: i64,
    pub text: String,
}
