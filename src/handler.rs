//! Handler module

use teloxide::{prelude::*, requests::ResponseResult, utils::command::BotCommand};

#[derive(BotCommand)]
#[command(rename = "lowercase", description = "These commands are supported:")]
pub enum Command {
    #[command(description = "Display this text")]
    Help,
    #[command(description = "Put something in the Black Box")]
    Put(String),
    #[command(description = "Take something out of the Black Box")]
    Take(i64),
    #[command(description = "Look into the Black Box")]
    Look,
    #[command(description = "How many things are in the Black Box?")]
    Count,
}

pub async fn handler(
    cx: UpdateWithCx<AutoSend<Bot>, Message>,
    command: Command,
) -> ResponseResult<()> {
    match command {
        Command::Help => cx.answer(Command::descriptions()).await?,
        Command::Put(item) => {
            cx.answer(format!("You put {} in the Black Box", item))
                .await?
        }
        Command::Take(id) => {
            cx.answer(format!("You took {} out of the Black Box", id))
                .await?
        }
        Command::Look => cx.answer("What's in the Black Box? 🤔").await?,
        Command::Count => cx.answer("I don't know yet 🤷‍♂️").await?,
    };

    Ok(())
}
