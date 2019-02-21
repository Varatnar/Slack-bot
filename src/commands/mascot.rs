pub mod item_group;
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