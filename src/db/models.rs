/// This `struct` represents the model for items in the database. 
/// It can be created by the `new` method.
#[derive(sqlx::FromRow, Debug, PartialEq, Default)]
pub struct Item {
    pub id: i64,
    pub chat_id: i64,
    pub name: String,
}

/// Creates new `Item`
///
/// # Example
///
/// ```
/// let item = Item::new(42, "hello");
/// ```
impl Item {
    pub fn new(chat_id: i64, name: &str) -> Self {
        Self {
            id: 0,
            chat_id,
            name: name.to_string(),
        }
    }
}
