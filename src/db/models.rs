#[derive(sqlx::FromRow, Debug, PartialEq, Default)]
pub struct Item {
    pub id: i64,
    pub chat_id: i64,
    pub name: String,
}

impl Item {
    pub fn new(chat_id: i64, name: &str) -> Self {
        Self {
            id: 0,
            chat_id,
            name: name.to_string(),
        }
    }
}
