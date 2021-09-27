// See example
// https://github.com/teloxide/teloxide/tree/dev/examples/sqlite_remember_bot

use dotenv::dotenv;
use teloxide::{prelude::*, utils::command::BotCommand};

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    // enable .env
    dotenv().ok();
    // enable logging
    teloxide::enable_logging!();

    let bot = Bot::from_env().auto_send();
    let bot_name = "hold-my-beer-bot";

    // start the bot
    log::info!("Starting bot...");
    teloxide::commands_repl(bot, bot_name, handler).await;

}
