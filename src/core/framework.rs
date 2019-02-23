use serenity::framework::StandardFramework;
use serenity::model::id::UserId;
use std::collections::HashSet;

use crate::commands::{self, mascot};

pub struct SlackbotFramework;

impl SlackbotFramework {
    pub fn new(owners: HashSet<UserId>) -> StandardFramework {
        StandardFramework::new()
            .configure(|c| c.owners(owners).prefixes(vec!["sg"]).allow_whitespace(true))
            .group("Common", |_| commands::init_general())
            .group("Mascot's ItemGroup", |_| mascot::init_mascot_ig())
            .group("Mascot's Item", |_| mascot::init_mascot_item())
        //TODO User-Defined group with embeds + image templates from database
    }
}
