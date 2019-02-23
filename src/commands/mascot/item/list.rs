use std::sync::Arc;

use serenity::framework::standard::{Args, Command, CommandError, CommandOptions};
use serenity::model::channel::Message;
use serenity::prelude::Context;
use serenity::utils::MessageBuilder;

use crate::core::global::DB as db;
use crate::db::models::Item;

pub struct List;

impl Command for List {
    fn execute(&self, _: &mut Context, msg: &Message, _: Args) -> Result<(), CommandError> {
        match Item::all(&db.get_conn()) {
            Ok(results) => {
                let mut message = MessageBuilder::new()
                    .push(format!("Displaying {} items\n", results.len()));
                for item in results {
                    message = message.push(format!("{:?}\n", item));
                }
                let message = message.build();
                if let Err(why) = msg.channel_id.send_message(|m| m.content(message)) {
                    println!("Error sending message: {:?}", why);
                }
            }
            Err(_) => {
                let _ = msg.reply("Failed to retrieve result from the database.");
            }
        };
        Ok(())
    }

    fn options(&self) -> Arc<CommandOptions> {
        let default = CommandOptions::default();
        let options = CommandOptions {
            owners_only: true,
            ..default
        };
        Arc::new(options)
    }
}
