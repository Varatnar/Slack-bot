#[macro_use]
extern crate serenity;

mod commands;

use std::{collections::HashSet, env};

use serenity::{
    framework::StandardFramework,
    http,
    model::{event::ResumedEvent, gateway::Ready},
    prelude::*,
};

struct Handler;

impl EventHandler for Handler {
    fn ready(&self, _: Context, ready: Ready) {
        println!("Connected as {}", ready.user.name);
    }

    fn resume(&self, _: Context, _: ResumedEvent) {
        println!("Resumed");
    }
}

fn main() {
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

    client.with_framework(
        StandardFramework::new()
            .configure(|c| c.owners(owners).prefix("sg "))
            .command("ralfu", |c| c.cmd(commands::secret::ralfu::ralfu))
            .command("amq", |c| c.cmd(commands::general::amq))
            .command("quit", |c| c.cmd(commands::owner::quit).owners_only(true)),
    );
    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}
