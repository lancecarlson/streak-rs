extern crate dotenv;

use streak::Client;
use self::dotenv::dotenv;

use std::env;

fn setup_client() -> Client {
    dotenv().ok();

    let api_key = env::var("STREAK_API_KEY").expect("STREAK_API_KEY to be set");

    Client::new(&api_key)
}
