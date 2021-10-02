//! Handler module

use anyhow::Result;
use sqlx::sqlite::SqlitePool;
// use teloxide::{prelude::*, requests::ResponseResult, utils::command::BotCommand};
use teloxide::prelude::*;
use teloxide::utils::{command::BotCommand, markdown};

type Pool = SqlitePool;

#[derive(BotCommand)]
#[command(rename = "lowercase", description = "These commands are supported:\n")]
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

/// Main hadler
pub struct Handler<B, P> {
    bot: B,
    pool: P,
}