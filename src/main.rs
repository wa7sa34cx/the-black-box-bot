// mod db;
mod handlers;

use anyhow::Result;
use dotenv::dotenv;
use handlers::messages;
use std::env;
use teloxide::{prelude::*, types::ParseMode};
// use teloxide::utils::markdown;

async fn run() -> Result<()> {
    teloxide::enable_logging!();

    dotenv().ok();

    let bot = Bot::from_env()
        .parse_mode(ParseMode::MarkdownV2)
        .auto_send();

    log::info!("Starting bot...");

    teloxide::repl(bot, messages::handler).await;

    // teloxide::repl(bot, |cx| async move {
    //     cx.answer(markdown::escape(cx.update.text().unwrap_or("Don't understand"))).await?;
    //     respond(())
    // })
    // .await;

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
