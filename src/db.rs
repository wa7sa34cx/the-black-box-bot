mod models;
mod schema;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use models::*;

fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn get_bar() -> Vec<Beer> {
    let connection = establish_connection();

    use schema::bar::dsl::*;

    bar.limit(5)
        .load::<Beer>(&connection)
        .expect("Error loading bar")
}
