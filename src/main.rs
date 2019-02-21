#[macro_use]
extern crate diesel;
mod commands;
mod core;
mod db;
mod slack_client;

use slack_client::SlackbotClient;

fn main() {
    kankyo::load().expect("Failed to load .env file");

    display_ig();

    let mut client = SlackbotClient::new();
    client.start().expect("Client failed to start")
}

pub fn display_ig() {
    use crate::core::global::DB;
    match DB.list_item_groups() {
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
