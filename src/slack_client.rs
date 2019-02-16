use serenity::prelude::Client;
use std::{collections::HashSet, env};

use serenity::{
    http,
    model::{channel::Message, event::ResumedEvent, gateway::Ready},
    prelude::*,
};

use crate::core::framework::SlackbotFramework;

struct Handler;

impl EventHandler for Handler {
    fn ready(&self, _: Context, ready: Ready) {
        println!("Connected as {}", ready.user.name);
    }

    fn message(&self, _: Context, _: Message) {
        // nothing
    }

    fn resume(&self, _: Context, _: ResumedEvent) {
        println!("Resumed");
    }
}

pub struct SlackbotClient(Client);

impl SlackbotClient {
    pub fn new() -> Self {
        let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
        let mut client = Client::new(&token, Handler).expect("Err creating client");

        let owners = match http::get_current_application_info() {
            Ok(info) => {
                let mut set = HashSet::new();
                set.insert(info.owner.id);

                set
            }
            Err(why) => panic!("Couldn't get application info: {:?}", why),
        };

        client.with_framework(SlackbotFramework::new(owners));
        if let Err(why) = client.start() {
            println!("Client error: {:?}", why);
        }

        SlackbotClient(client)
    }

    pub fn start(&mut self) -> Result<(), SerenityError> {
        self.start_autosharded()
    }
    pub fn start_autosharded(&mut self) -> Result<(), SerenityError> {
        self.0.start_autosharded()
    }
}
