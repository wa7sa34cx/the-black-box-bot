mod handler;
// mod db;

use anyhow::Result;
use dotenv::dotenv;
use handler::handler;
use teloxide::{prelude::*, types::ParseMode};
use std::env;

// type Bot = AutoSend<DefaultParseMode<teloxide::Bot>>;

async fn run() -> Result<()> {
    // enable logging
    teloxide::enable_logging!();

    // create bot
    dotenv().ok();
    // let token = env::var("DB_PATH")?;
    // let bot = Bot::from_env().parse_mode(ParseMode::MarkdownV2).auto_send();
    let bot = Bot::from_env().parse_mode(ParseMode::MarkdownV2).auto_send();
    // let bot = Bot::new(&token).parse_mode(ParseMode::MarkdownV2).auto_send();
    // let bot = 
    // let bot = Bot::from_env().auto_send();

    // start the bot
    log::info!("Starting bot...");
    let bot_name = "the-black-box-bot";
    teloxide::commands_repl(bot, bot_name, handler).await;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    run().await
}
