extern crate serenity;

//use std::env;

use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
    utils::MessageBuilder,
};

struct Handler;

impl EventHandler for Handler {
    fn message(&self, _: Context, msg: Message) {
        if msg.content == "!nintendo" {
            let response = MessageBuilder::new()
                .push("Let's ")
                .push_bold_safe("smash ")
                .push("together ")
                .mention(&msg.author)
                .push(".")
                .build();

            if let Err(why) = msg.channel_id.say(&response) {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = "nothing".to_owned();

    // Create a new instance of the Client, logging in as a bot. This will
    // automatically prepend your bot token with "Bot ", which is a requirement
    // by Discord for bot users.
    let mut client = Client::new(&token, Handler).expect("Err creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}
