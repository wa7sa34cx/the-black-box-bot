//! Handler module

use crate::db::models::*;
use crate::db::Db;
use anyhow::Result;
use async_once::AsyncOnce;
use lazy_static::lazy_static;
use teloxide::utils::{command::BotCommand, markdown};
use teloxide::{adaptors::DefaultParseMode, prelude::*};

lazy_static! {
    static ref DB: AsyncOnce<Db> = AsyncOnce::new(async { Db::from_env().await });
}

#[derive(BotCommand, PartialEq, Debug)]
#[command(rename = "lowercase")]
pub enum Command {
    Start,
    Help,
    Put(String),
    Take(String),
    Look,
    Count,
    Shake,
}

type Bot = AutoSend<DefaultParseMode<teloxide::Bot>>;
type Cx = UpdateWithCx<Bot, Message>;

/// Main handler
pub async fn handler(cx: Cx) -> Result<()> {
    let chat_id = cx.update.chat_id();

    let text = cx.update.text();
    if text.is_none() {
        answer(&cx, "I don't know what to reply 🤷‍♂️\nTry /help command").await?;
        return Ok(());
    }

    let text = text.unwrap();
    let command = Command::parse(text, "bot_name");
    if command.is_err() {
        answer(&cx, "I don't know this command 😕").await?;
        return Ok(());
    }

    let command = command.unwrap();
    let ans = match command {
        Command::Start => start().await?,
        Command::Help => help().await?,
        Command::Put(text) => put(chat_id, &text).await?,
        Command::Take(text) => take(chat_id, &text).await?,
        Command::Look => look(chat_id).await?,
        Command::Count => count(chat_id).await?,
        Command::Shake => shake(chat_id).await?,
    };

    answer(&cx, &ans).await?;

    Ok(())
}

// Display greeting
async fn start() -> Result<String> {
    Ok(format!(
        "This is the *Black Box*\\. You can hold any items in it\\.\nType /help for help\\.",
    ))
}

// Display help info
async fn help() -> Result<String> {
    Ok(format!(
        "These commands are supported:\n\n\
        /put *some item* \\- Put item\n\
        /take *some item* \\- Take item\n\
        /look \\- Look into\n\
        /shake \\- Shake items out\n\
        /count \\- Count items\n\
        /help \\- Display help info
    "
    ))
}

// Put item
async fn put(chat_id: i64, text: &str) -> Result<String> {
    if text.is_empty() {
        return Ok(format!("Please use this format:\n\n /put *some item*"));
    }

    let item = Item::new(chat_id, text.trim());
    DB.get().await.put(&item).await?;

    Ok(format!(
        "You put *{}* in the Black Box",
        markdown::escape(text)
    ))
}

// Take item
async fn take(chat_id: i64, text: &str) -> Result<String> {
    if text.is_empty() {
        return Ok(format!("Please use this format:\n\n /take *some item*"));
    }

    let item = Item::new(chat_id, text);
    match DB.get().await.take(&item).await {
        Err(e) => {
            log::warn!("{}", e);
            Ok(format!("There is no such item in the Black Box 🤷‍♂️"))
        }
        Ok(_) => Ok(format!(
            "You took *{}* out of the Black Box",
            markdown::escape(text)
        )),
    }
}

// Look into
async fn look(chat_id: i64) -> Result<String> {
    let items = DB.get().await.look(chat_id).await?;

    if items.is_empty() {
        return Ok(format!("There aren't any items in the Black Box 🤷‍♂️"));
    }

    if items.len() == 1 {
        return Ok(format!(
            "There is one item in the Black Box:\n\n{}",
            markdown::escape(&items[0].name)
        ));
    }

    let answer: String = items.iter().enumerate().fold(String::new(), |acc, en| {
        let (i, item) = en;
        format!("{}{}. {}\n", acc, i + 1, item.name)
    });

    Ok(format!(
        "There are several items in the Black Box:\n\n{}",
        markdown::escape(&answer)
    ))
}

// Count items
async fn count(chat_id: i64) -> Result<String> {
    let count = DB.get().await.count(chat_id).await?;

    match count {
        0 => return Ok(format!("There aren't any items in the Black Box 🤷‍♂️")),
        1 => return Ok(format!("There is *one* item in the Black Box")),
        n => return Ok(format!("There are *{}* items in the Black Box", n)),
    }
}

// Shake out
async fn shake(chat_id: i64) -> Result<String> {
    DB.get().await.shake(chat_id).await?;

    Ok(format!("The Black Box is now empty"))
}

// Send answer
async fn answer(cx: &Cx, answer: &str) -> Result<()> {
    cx.answer(answer).await?;

    Ok(())
}
