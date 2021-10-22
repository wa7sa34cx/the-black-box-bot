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
    pub fn new<S>(chat_id: i64, name: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            id: 0,
            chat_id,
            name: name.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::db::models::*;

    #[test]
    fn new_item() {
        let item = Item::new(42, "hello");
        let expectation = Item {
            id: 0,
            chat_id: 42,
            name: "hello".to_owned(),
        };

        assert_eq!(item, expectation);
    }
}
