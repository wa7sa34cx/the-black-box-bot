#[derive(sqlx::FromRow, Debug)]
pub struct Item {
    pub id: i64,
    pub chat_id: i64,
    pub name: String,
}

#[derive(Debug)]
pub struct NewItem {
    pub chat_id: i64,
    pub name: &'static str,
}