use super::schema::item_groups;

#[derive(Queryable, Insertable, Debug)]
pub struct ItemGroup {
    pub layer: i32,
    pub name: String,
    pub description: Option<String>,
}
