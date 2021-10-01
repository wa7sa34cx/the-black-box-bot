//! Database module

pub mod models;

use anyhow::Result;
use dotenv::dotenv;
use models::*;
use once_cell::sync::OnceCell;
use sql_builder::{quote, SqlBuilder};
use sqlx::sqlite::SqlitePool;
use std::env;

type Pool = SqlitePool;

static POOL: OnceCell<Pool> = OnceCell::new();

pub struct Db {
    pool: &'static Pool,
}

impl Db {
    /// Initialize database
    #[allow(unused)]
    pub async fn new(database_url: &str) -> Self {
        dotenv().ok();
        let pool = Pool::connect(database_url)
            .await
            .unwrap();
        POOL.set(pool).unwrap();

        Self {
            pool: POOL.get().unwrap(),
        }
    }

    /// Initialize database
    pub async fn from_env() -> Self {
        dotenv().ok();
        let pool = Pool::connect(&env::var("DATABASE_URL").unwrap())
            .await
            .unwrap();
        POOL.set(pool).unwrap();

        Self {
            pool: POOL.get().unwrap(),
        }
    }

    /// Get all items by chat_id
    pub async fn look(&self, item: Item) -> Result<Vec<Item>> {
        let sql = SqlBuilder::select_from("items")
            .and_where_eq("chat_id", item.chat_id)
            .order_asc("id")
            .sql()?;

        let items: Vec<Item> = sqlx::query_as(&sql).fetch_all(self.pool).await?;

        Ok(items)
    }

    /// Insert the item
    pub async fn put(&self, item: Item) -> Result<()> {
        let sql = SqlBuilder::insert_into("items")
            .field("chat_id")
            .field("name")
            .values(&[format!("{}, {}", item.chat_id, quote(item.name))])
            .sql()?;

        sqlx::query(&sql).execute(self.pool).await?;

        Ok(())
    }

    /// Delete the item
    pub async fn take(&self, item: Item) -> Result<()> {
        let sql = SqlBuilder::delete_from("items")
            .and_where_eq("id", item.id)
            .and_where_eq("chat_id", item.chat_id)
            .sql()?;

        sqlx::query(&sql).execute(self.pool).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_look() {
        let db = Db::new("sqlite://db/test/blackbox.db").await;
        let item = Item {
            id: 0,
            chat_id: 13,
            name: String::new(),
        };
        let items = db.look(item).await.unwrap();
        assert_eq!(items, vec![Item {
            id: 1,
            chat_id: 13,
            name: "hello".to_string(),
        }]);
    }
}
