//! Main module

mod db;
mod handler;

use dotenv::dotenv;
use handler::handler;
use std::env;
use teloxide::{prelude::*, types::ParseMode};

async fn run() {
    teloxide::enable_logging!();

    dotenv().ok();

    let bot = Bot::from_env()
        .parse_mode(ParseMode::MarkdownV2)
        .auto_send();

    log::info!("Starting bot...");

    teloxide::repl(bot, handler).await;
}

#[tokio::main]
async fn main() {
    run().await;
}
