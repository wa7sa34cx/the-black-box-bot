//! Test database module

// mod macros;

use anyhow::Result;
use crate::POOL;
use crate::pool;

pub async fn db_test() -> Result<()> {  
    // let sql = "SELECT * FROM items;";

    let items = sqlx::query!(r#"SELECT * FROM items;"#).fetch_all(pool!()).await?;

    for item in items {
        println!("{}, {}, {}", item.id, item.chat_id, item.name);
    }

    Ok(())
}