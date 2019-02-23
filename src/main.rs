#[macro_use]
extern crate diesel;

use slack_client::SlackbotClient;

use crate::db::models::ItemGroup;

mod commands;
mod core;
mod db;
mod slack_client;

fn main() {
    kankyo::load().expect("Failed to load .env file");

    display_ig();

    let mut client = SlackbotClient::new();
    client.start().expect("Client failed to start")
}

pub fn display_ig() {
    use crate::core::global::DB;
    match ItemGroup::all(&DB.get_connection()) {
        Ok(results) => {
            println!("Displaying {} item groups", results.len());
            for item_group in results {
                println!("{:?}", item_group);
            }
        }

        Err(_) => {
            println!("Failed to retrieve result from the database.");
        }
    };
}
