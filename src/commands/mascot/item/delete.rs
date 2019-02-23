use std::sync::Arc;

use serenity::framework::standard::{Args, Command, CommandError, CommandOptions};
use serenity::model::channel::Message;
use serenity::prelude::Context;

use crate::core::global::DB as db;
use crate::db::models::Item;

// Delete command for Item
pub struct Delete;

impl Command for Delete {
    fn execute(&self, _: &mut Context, msg: &Message, mut args: Args) -> Result<(), CommandError> {
        if let Ok(name) = args.single::<String>() {
            if let Ok(deleted) = Item::delete_with_name(name, &db.get_conn()) {
                let _ = msg.reply(&format!("Deleted {} entries.", deleted));
            }
        } else {
            let _ = msg.reply("Invalid command syntax.");
        }

        Ok(())
    }

    fn options(&self) -> Arc<CommandOptions> {
        let default = CommandOptions::default();
        let options = CommandOptions {
            min_args: Some(1),
            max_args: Some(1),
            owners_only: true,
            aliases: vec!["del"].into_iter().map(|e| e.to_string()).collect(),
            ..default
        };
        Arc::new(options)
    }
}
