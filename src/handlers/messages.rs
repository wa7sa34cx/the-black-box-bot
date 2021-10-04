//! Handler module

use anyhow::Result;
// use teloxide::{prelude::*, requests::ResponseResult, utils::command::BotCommand};
use teloxide::{prelude::*, adaptors::DefaultParseMode};
use teloxide::utils::{command::BotCommand, markdown};

#[derive(BotCommand, PartialEq, Debug)]
pub enum Command {
    Help,
    Put(String),
    Take(String),
    Look,
    Count,
}

type Bot = AutoSend<DefaultParseMode<teloxide::Bot>>;
type Cx = UpdateWithCx<Bot, Message>;

///
pub async fn handler(cx: Cx) -> Result<()> {
    let text = cx.update.text();
    if text.is_none() {
        cx.answer("I don't know what to reply 🤷‍♂️").await?;
        return Ok(());
    }

    let text = text.unwrap();
    let command = Command::parse(text, "bot_name");
    if command.is_err() {
        cx.answer("This is not a command 😕").await?;
        return Ok(());
    }

    let command = command.unwrap();
    match command {
        Command::Help => help(&cx).await?,
        Command::Put(text) => put(&cx, &text).await?,
        Command::Take(text) => take(&cx, &text).await?, 
        Command::Look => look(&cx).await?, 
        Command::Count => count(&cx).await?, 
    };


    // cx.answer(markdown::escape(cx.update.text().unwrap_or("Don't understand"))).await?
    // cx.answer("This is not a command").await?;
        
    Ok(())
}

async fn help(cx: &Cx) -> Result<()> {
    cx.answer("HELP").await?;

    Ok(())
}

async fn put(cx: &Cx, _text: &str) -> Result<()> {
    cx.answer("PUT").await?;

    Ok(())
}

async fn take(cx: &Cx, _text: &str) -> Result<()> {
    cx.answer("TAKE").await?;

    Ok(())
}

async fn look(cx: &Cx) -> Result<()> {
    cx.answer("LOOK").await?;

    Ok(())
}

async fn count(cx: &Cx) -> Result<()> {
    cx.answer("COUNT").await?;

    Ok(())
}
