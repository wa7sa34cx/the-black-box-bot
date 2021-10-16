//! Handler module

use crate::db::{models::*, Db};
use anyhow::Result;
use async_once::AsyncOnce;
use lazy_static::lazy_static;
use teloxide::utils::{command::BotCommand, markdown};
use teloxide::{adaptors::DefaultParseMode, prelude::*};
use tokio::time::{sleep, Duration};

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
    Delay(u64),
}

type Bot = AutoSend<DefaultParseMode<teloxide::Bot>>;
type Cx = UpdateWithCx<Bot, Message>;

/// Main handler.
pub async fn handler(cx: Cx) -> Result<()> {
    let chat_id = cx.update.chat_id();

    let text = match cx.update.text() {
        None => {
            answer(&cx, "I don't know what to reply 🤷‍♂️\nTry /help command").await?;
            return Ok(());
        }
        Some(text) => text,
    };

    let command = match Command::parse(text, "bot_name") {
        Err(_) => {
            answer(&cx, "I don't know this command 😕").await?;
            return Ok(());
        }
        Ok(command) => command,
    };

    let ans = match command {
        Command::Start => start().await?,
        Command::Help => help().await?,
        Command::Put(text) => put(chat_id, &text).await?,
        Command::Take(text) => take(chat_id, &text).await?,
        Command::Look => look(chat_id).await?,
        Command::Count => count(chat_id).await?,
        Command::Shake => shake(chat_id).await?,
        Command::Delay(secs) => delay(secs).await?,
    };

    answer(&cx, &ans).await?;

    Ok(())
}

// Displays the greeting.
async fn start() -> Result<String> {
    Ok(format!(
        "This is the *Black Box*\\. You can hold any items in it\\. \
        Type /help to view supported commands\\.",
    ))
}

// Displays help info.
async fn help() -> Result<String> {
    Ok(markdown::escape(
        "These commands are supported:\n\n\
        /put <some item> - Put item\n\n\
        /take <some item> - Take item\n\n\
        /look - Look into\n\n\
        /shake - Shake all items out\n\n\
        /count - Count items\n\n\
        /help - Display help info
    ",
    ))
}

// Puts item.
async fn put(chat_id: i64, text: &str) -> Result<String> {
    if text.is_empty() {
        return Ok(format!(
            "Please use this format:\n\n /put *{}*",
            markdown::escape("<some item>")
        ));
    }

    let item = Item::new(chat_id, text.trim());
    DB.get().await.put(&item).await?;

    Ok(format!(
        "You put *{}* in the Black Box",
        markdown::escape(text)
    ))
}

// Takes item.
async fn take(chat_id: i64, text: &str) -> Result<String> {
    if text.is_empty() {
        return Ok(format!(
            "Please use this format:\n\n /take *{}*",
            markdown::escape("<some item>")
        ));
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

// Looks into.
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

// Counts items.
async fn count(chat_id: i64) -> Result<String> {
    let count = DB.get().await.count(chat_id).await?;

    match count {
        0 => return Ok(format!("There aren't any items in the Black Box 🤷‍♂️")),
        1 => return Ok(format!("There is *one* item in the Black Box")),
        n => return Ok(format!("There are *{}* items in the Black Box", n)),
    }
}

// Shakes all items out.
async fn shake(chat_id: i64) -> Result<String> {
    DB.get().await.shake(chat_id).await?;

    Ok(format!("The Black Box is now empty"))
}

// Answers with delay (for testing concurrency).
async fn delay(secs: u64) -> Result<String> {
    if secs > 60 {
        return Ok(format!("Maximum value is 60 secs"));
    }

    // thread::sleep(Duration::from_secs(secs)); // Bad practice!
    sleep(Duration::from_secs(secs)).await;

    Ok(format!("I waited *{}* seconds before answering you", secs))
}

// Sends answer.
async fn answer(cx: &Cx, answer: &str) -> Result<()> {
    cx.answer(answer).await?;

    Ok(())
}
