use super::schema::{item_groups, items};

#[derive(Queryable, Insertable, Debug)]
pub struct ItemGroup {
    pub layer: i32,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Queryable, Debug)]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub layer: i32,
}

#[derive(Insertable, Debug)]
#[table_name = "items"]
pub struct NewItem {
    pub name: String,
    pub description: Option<String>,
    pub layer: i32,
}
