
mod commands;
mod core;
mod slack_client;

use slack_client::SlackbotClient;

fn main() {
    kankyo::load().expect("Failed to load .env file");
    let mut client = SlackbotClient::new();
    client.start().expect("Client failed to start")
}
