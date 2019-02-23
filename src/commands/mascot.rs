pub mod item_group;
pub mod item;
use serenity::framework::standard::CreateGroup;

// Owner only commands to setup the mascot's item groups
pub fn init_mascot_ig() -> CreateGroup {
        CreateGroup::default()
            .prefixes(vec!["ralfu_ig", "ralfuig"])
            .owners_only(true)
            .default_cmd(item_group::list::List)
            .cmd("list", item_group::list::List)
            .cmd("new", item_group::new::New)
            .cmd("delete", item_group::delete::Delete)
}

// Owner only commands to setup the mascot's items
pub fn init_mascot_item() -> CreateGroup {
        CreateGroup::default()
            .prefixes(vec!["ralfu_item", "ralfuitem", "ralfu_i", "ralfui"])
            .owners_only(true)
            .default_cmd(item::list::List)
            .cmd("list", item::list::List)
            .cmd("new", item::new::New)
            .cmd("delete", item::delete::Delete)
}