use crate::core::global::DB as db;
use serenity::framework::standard::{Args, Command, CommandError, CommandOptions};
use serenity::model::channel::Message;
use serenity::prelude::Context;
use std::sync::Arc;
pub struct New;

impl Command for New {
    fn execute(&self, _: &mut Context, msg: &Message, mut args: Args) -> Result<(), CommandError> {
        if let (Ok(layer), Ok(name), description) = (
            args.single::<i32>(),
            args.single::<String>(),
            args.rest(),
        ) {
            if let Ok(Some(1)) = db.new_item_group(layer, name.clone(), Some(description.to_owned()))
            {
                let _ = msg.reply(&format!("New item group {} added succesfully.", name));
            }
            else
            {
                let _ = msg.reply(&format!("Failed to add item group {} to database.", name));
            }
        }
        else
        {
            let _ = msg.reply("Invalid command syntax.");
        }
        Ok(())
    }

    fn options(&self) -> Arc<CommandOptions> {
        let default = CommandOptions::default();
        let options = CommandOptions {
            min_args: Some(2),
            owners_only: true,
            aliases: vec!["add", "create"].into_iter().map(|e| e.to_string()).collect(),
            ..default
        };
        Arc::new(options)
    }
}
