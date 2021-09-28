// See example
// https://github.com/teloxide/teloxide/tree/dev/examples/sqlite_remember_bot

mod handler;

use dotenv::dotenv;
use handler::handler;
use teloxide::prelude::*;

async fn run() {
    // enable .env
    dotenv().ok();
    // enable logging
    teloxide::enable_logging!();

    let bot = Bot::from_env().auto_send();
    let bot_name = "the-black-box-bot";

    // start the bot
    log::info!("Starting bot...");
    teloxide::commands_repl(bot, bot_name, handler).await;
}

#[tokio::main]
async fn main() {
    run().await;
}
