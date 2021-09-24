#[macro_use]
extern crate diesel;

mod db;

pub async fn run() {
    let bar = db::get_bar();
    dbg!(bar);
}
