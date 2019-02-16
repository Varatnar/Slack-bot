#[derive(Queryable)]
pub struct ItemGroup {
    pub layer: i32,
    pub name: String,
    pub description: Option<String>,
}
