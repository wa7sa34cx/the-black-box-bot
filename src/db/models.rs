#[derive(sqlx::FromRow, Debug, PartialEq)]
pub struct Item {
    pub id: i64,
    pub chat_id: i64,
    pub name: String,
}
