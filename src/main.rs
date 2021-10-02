mod db;
// mod handlers;

use anyhow::Result;
use dotenv::dotenv;
// use handlers::messages;
use std::env;
use teloxide::{prelude::*, types::ParseMode};

async fn run() -> Result<()> {
    teloxide::enable_logging!();

    dotenv().ok();

    let bot = Bot::from_env()
        .parse_mode(ParseMode::MarkdownV2)
        .auto_send();

    log::info!("Starting bot...");

    // Dispatcher::new(bot)
    //     .messages_handler(messages::handler)
    //     .setup_ctrlc_handler()
    //     .dispatch()
    //     .await;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    run().await
}
