//! Database module

pub mod models;

use anyhow::Result;
use dotenv::dotenv;
use models::*;
use sql_builder::{quote, SqlBuilder};
use sqlx::sqlite::SqlitePool;
use std::env;

type Pool = SqlitePool;

/// Asyncronous database
pub struct Db {
    pool: Pool,
}

impl Db {
    /// Initialize database from str
    #[allow(unused)]
    pub async fn new(database_url: &str) -> Self {
        dotenv().ok();
        let pool = Pool::connect(database_url).await.unwrap();

        Self { pool }
    }

    /// Initialize database from env
    pub async fn from_env() -> Self {
        dotenv().ok();
        let pool = Pool::connect(&env::var("DATABASE_URL").unwrap())
            .await
            .unwrap();

        Self { pool }
    }

    /// Get all items by chat_id
    pub async fn look(&self, chat_id: i64) -> Result<Vec<Item>> {
        let sql = SqlBuilder::select_from("items")
            .and_where_eq("chat_id", chat_id)
            .order_asc("id")
            .sql()?;

        let items: Vec<Item> = sqlx::query_as(&sql).fetch_all(&self.pool).await?;

        Ok(items)
    }

    /// Insert the item
    pub async fn put(&self, item: &Item) -> Result<()> {
        let sql = SqlBuilder::insert_into("items")
            .field("chat_id")
            .field("name")
            .values(&[format!("{}, {}", item.chat_id, quote(&item.name))])
            .sql()?;

        sqlx::query(&sql).execute(&self.pool).await?;

        Ok(())
    }

    /// Delete the item
    pub async fn take(&self, item: &Item) -> Result<()> {
        let sql = SqlBuilder::select_from("items")
            .and_where_eq("name", quote(&item.name))
            .and_where_eq("chat_id", item.chat_id)
            .sql()?;

        let item: Item = sqlx::query_as(&sql).fetch_one(&self.pool).await?;

        let sql = SqlBuilder::delete_from("items")
            .and_where_eq("id", item.id)
            .sql()?;

        sqlx::query(&sql).execute(&self.pool).await?;

        Ok(())
    }

    /// Count items
    pub async fn count(&self, chat_id: i64) -> Result<i64> {
        let sql = SqlBuilder::select_from("items")
            .count("id")
            .and_where_eq("chat_id", chat_id)
            .sql()?;

        let row: (i64,) = sqlx::query_as(&sql).fetch_one(&self.pool).await?;

        Ok(row.0)
    }

    /// Delete all items
    pub async fn shake(&self, chat_id: i64) -> Result<()> {
        let sql = SqlBuilder::delete_from("items")
            .and_where_eq("chat_id", chat_id)
            .sql()?;

        sqlx::query(&sql).execute(&self.pool).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_db() {
        let db = Db::from_env().await;
        let chat_id = 42;

        test_put(&db, chat_id).await;
        test_look(&db, chat_id).await;
        test_count(&db, chat_id).await;
        test_take(&db, chat_id).await;
        test_shake(&db, chat_id).await;
    }

    async fn test_put(db: &Db, chat_id: i64) {
        let item1 = Item::new(chat_id, "apple");
        let item2 = Item::new(chat_id, "pear");

        assert_eq!(db.put(&item1).await.unwrap(), ());
        assert_eq!(db.put(&item2).await.unwrap(), ());
    }

    async fn test_look(db: &Db, chat_id: i64) {
        let items = db.look(chat_id).await.unwrap();

        assert_eq!(items[0].name, "apple".to_string());
        assert_eq!(items[1].name, "pear".to_string());
    }

    async fn test_count(db: &Db, chat_id: i64) {
        assert_eq!(db.count(chat_id).await.unwrap(), 2);
    }

    async fn test_take(db: &Db, chat_id: i64) {
        let item = Item::new(chat_id, "apple");

        assert_eq!(db.put(&item).await.unwrap(), ());
    }

    async fn test_shake(db: &Db, chat_id: i64) {
        assert_eq!(db.shake(chat_id).await.unwrap(), ());
    }
}
