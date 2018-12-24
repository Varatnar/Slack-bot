use serenity::framework::StandardFramework;
use serenity::model::id::UserId;
use std::collections::HashSet;

use commands;

pub struct SlackbotFramework;

impl SlackbotFramework {
    pub fn new(owners: HashSet<UserId>) -> StandardFramework {
        StandardFramework::new()
            .configure(|c| c.owners(owners).prefixes(vec!["sg"]).allow_whitespace(true))
            .group("Common", |g| {
                g //.prefix("sg")
                    .command("ralfu", |c| c.cmd(commands::secret::ralfu::ralfu))
                    .command("amq", |c| c.cmd(commands::general::amq))
                    .command("quit", |c| c.cmd(commands::owner::quit).owners_only(true))
            })
    }
}
