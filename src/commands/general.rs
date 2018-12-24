use serenity::framework::standard::{Args, Command, CommandError, CommandOptions};
use serenity::model::channel::Message;
use serenity::prelude::Context;
use serenity::utils::Colour;
use std::sync::Arc;

pub struct Amq;

impl Command for Amq {
    fn options(&self) -> Arc<CommandOptions> {
        let default = CommandOptions::default();
        Arc::new(default)
    }

    fn execute(&self, _: &mut Context, msg: &Message, mut args: Args) -> Result<(), CommandError> {
        let (room, password) = match (args.single::<String>(), args.single::<String>()) {
            (Ok(room), Ok(password)) => (room, password),
            _ => ("Gelo".to_owned(), "the obvious".to_owned()),
        };

        if let Err(why) = msg.channel_id.send_message(|m| {
            m.embed(|e| {
                e.title("AMQ")
                    .url("https://animemusicquiz.com/")
                    .color(Colour::new(0xdc6676))
                    .image("https://i.imgur.com/ZYadmWW.png")
                    .field("Room", room, true)
                    .field("Password", password, true)
            })
        }) {
            println!("Error sending message: {:?}", why);
        }
        Ok(())
    }
}
