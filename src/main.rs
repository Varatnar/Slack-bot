#[macro_use]
extern crate serenity;

mod commands;

use std::{collections::HashSet, env};

use serenity::{
    framework::StandardFramework,
    http,
    model::{channel::Message, event::ResumedEvent, gateway::Ready},
    prelude::*,
};

use serenity::framework::standard::{Args, Command};

struct Handler;

impl EventHandler for Handler {
    fn ready(&self, _: Context, ready: Ready) {
        println!("Connected as {}", ready.user.name);
    }

    fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "%10keuro" {
            match commands::secret::ralfu::tenk_euro.execute(
                &mut ctx.clone(),
                &mut msg.clone(),
                Args::new("", &["".to_string()]),
            ) {
                Ok(()) => println!("Success"),
                Err(_) => panic!("panicked at EventHandler message"),
            }
        }
        if msg.content == "%mvp" {
            match commands::secret::ralfu::mvp.execute(
                &mut ctx.clone(),
                &mut msg.clone(),
                Args::new("", &["".to_string()]),
            ) {
                Ok(()) => println!("Success"),
                Err(_) => panic!("panicked at EventHandler message"),
            }
        }
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
            .configure(|c| c.owners(owners).prefixes(vec!["sg"]).allow_whitespace(true))
            .group("Common", |g| {
                g //.prefix("sg")
                    .command("ralfu", |c| c.cmd(commands::secret::ralfu::ralfu))
                    .command("amq", |c| c.cmd(commands::general::amq))
                    .command("quit", |c| c.cmd(commands::owner::quit).owners_only(true))
            }),
    );
    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}
