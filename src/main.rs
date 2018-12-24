#[macro_use]
extern crate serenity;

mod commands;
mod core;
mod slack_client;

use slack_client::SlackbotClient;

fn main() {
    let mut client = SlackbotClient::new();
    client.start().expect("Client failed to start")
}
