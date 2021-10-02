//! Handler module

use anyhow::Result;
use dotenv::dotenv;
use sqlx::sqlite::SqlitePool;
use std::env;
// use teloxide::{prelude::*, requests::ResponseResult, utils::command::BotCommand};
use teloxide::{prelude::*, adaptors::DefaultParseMode};
use teloxide::utils::{command::BotCommand, markdown};

#[derive(BotCommand)]
#[command(rename = "lowercase", description = "These commands are supported:")]
pub enum Command {
    #[command(description = "Display this text")]
    Help,
    #[command(description = "Put something in the Black Box")]
    Put(String),
    #[command(description = "Take something out of the Black Box")]
    Take(String),
    #[command(description = "Look into the Black Box")]
    Look,
    #[command(description = "How many things are in the Black Box?")]
    Count,
}

type Bot = AutoSend<DefaultParseMode<teloxide::Bot>>;

/// Main handler
pub async fn handler(cx: UpdateWithCx<Bot, Message>, command: Command) -> Result<()> {
    match command {
        Command::Help => cx.answer(Command::descriptions()).await?,
        Command::Put(text) => cx.answer(answer_put(text).await).await?,
        Command::Take(text) => {
            cx.answer(format!("You took {} out of the Black Box", text))
                .await?
        }
        Command::Look => cx.answer("What's in the Black Box? 🤔").await?,
        Command::Count => cx.answer("I don't know yet 🤷‍♂️").await?,
    };

    Ok(())
}

// Put answer
async fn answer_put(text: String) -> String {
    if text.is_empty() {
        // return "Please use this format:\n /put *item*".to_string();
        return format!("Please use this format:\n /put {}", markdown::bold("item")) 
        // return format!("Please use this format:\n /put **item**") 
    }

    format!("You put {} in the Black Box", text)    
}