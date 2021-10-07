# The Black Box bot

This is the **Black Box** telegram bot written in Rust. You can hold any items in it.
Try it here: [https://t.me/the_black_box_bot](https://t.me/the_black_box_bot)

## Usage

These commands are supported:

- `/put <some item>` - Put item
- `/take <some item>` - Take item
- `/look` - Look into
- `/shake` - Shake all items out
- `/count` - Count items
- `/help` - Display help info

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/)
- [SQLite](https://sqlite.org/)
- [Sqlx-cli](https://github.com/launchbadge/sqlx/tree/master/sqlx-cli)

### Deployment

1. Fork this repository to your folder
1. Talk to [@BotFather](https://t.me/botfather) and go through some dialog options until you've successfully created a bot. You should receive a token in the format of `123456789:blablabla`
1. Edit `.env.example` by putting there your `DATABASE_URL` and `TELOXIDE_TOKEN`
1. Rename `.env.example` to `.env`
1. Create database with command `sqlx database create`
1. Run this command `sqlx migrate run`. It will create tables in your database
1. Now after all these are set-up to run the bot just execute `cargo run` from your terminal.
