pub mod general;
pub mod mascot;
pub mod owner;

use self::general::*;
use self::owner::*;

use serenity::framework::standard::CreateGroup;

pub fn init_general() -> CreateGroup {
        CreateGroup::default()
                .cmd("amq", Amq)
                .command("quit", |c| c.cmd(quit).owners_only(true))
}

