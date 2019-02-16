use serenity::framework::StandardFramework;
use serenity::model::id::UserId;
use std::collections::HashSet;

use crate::commands;

pub struct SlackbotFramework;

impl SlackbotFramework {
    pub fn new(owners: HashSet<UserId>) -> StandardFramework {
        StandardFramework::new()
            .configure(|c| c.owners(owners).prefixes(vec!["sg"]).allow_whitespace(true))
            .group("Common", |_| commands::init_general())
            // comment out the line below to make a copy of bot from repository run
            .group("Secret", |_| commands::secret::init_secret())
            //TODO User-Defined group with embeds + image templates from database
    }
}
