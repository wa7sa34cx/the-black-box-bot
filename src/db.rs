//! Database module

mod models;

use anyhow::Result;
use sqlx::sqlite::SqlitePool;
use sql_builder::{quote, SqlBuilder};
use models::*;

pub async fn put_item(pool: &SqlitePool, chat_id: i64, name: String) -> Result<()> {  
    let sql = SqlBuilder::insert_into("items")
        // .field("id")
        .field("chat_id")
        .field("name")
        .field("published")
        // .values(&[&quote(post.title), &quote(post.date), &0.to_string()])
        // .values(&[&quote(post.title), "datetime('now')", "0"])
        .values(&[format!(
            "{}, {}, {}",
            quote(post.title),
            "datetime('now')",
            "0"
        )])
        // .order_asc("id")
        .sql()?;

    sqlx::query(&sql).execute(pool).await?;

    Ok(())
}