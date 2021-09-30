// mod handler;
// mod db_test;
// mod macros;
mod handlers;

use anyhow::Result;
// use async_once::AsyncOnce;
use dotenv::dotenv;
// use handler::handler;
// use lazy_static::lazy_static;
// use teloxide::{Bot, adaptors::AutoSend, types::ParseMode};
use once_cell::sync::OnceCell;
use teloxide::{prelude::*, types::ParseMode};
use sqlx::{sqlite::SqlitePool, Pool, Sqlite};
use std::env;
// use db_test::db_test;
use handlers::messages;

static POOL: OnceCell<Pool<Sqlite>> = OnceCell::new();

async fn run() -> Result<()> {
    // enable global logging 
    teloxide::enable_logging!();
    
    // add .env environment
    dotenv().ok();

    // create database pool connections
    log::info!("Initializing database...");
    let pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;
    POOL.set(pool).unwrap();

    // create bot
    let bot = Bot::from_env().parse_mode(ParseMode::MarkdownV2).auto_send();
    // let bot_name = "the-black-box-bot";

    log::info!("Starting bot...");
    // teloxide::commands_repl(bot, bot_name, handler).await;
    // Dispatcher::new(bot)
    //     .messages_handler(|rx: DispatcherHandlerRx<AutoSend<Bot>, Message>| {
    //         UnboundedReceiverStream::new(rx).for_each_concurrent(None, |cx| async move {
    //             answer(cx).await.log_on_error().await
    //         })
    //     })
    //     .setup_ctrlc_handler()
    //     .dispatch()
    //     .await;

    Dispatcher::new(bot)
        .messages_handler(messages::handler)
        .setup_ctrlc_handler()
        .dispatch()
        .await;

    // db_test().await?;
    // messages::handler().await;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    run().await
}
