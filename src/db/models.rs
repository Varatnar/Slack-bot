use diesel::{self, prelude::*};
use diesel::sqlite::SqliteConnection;

use super::schema::{item_groups, items};
use super::schema::item_groups::dsl::item_groups as all_item_groups;
use super::schema::items::dsl::items as all_items;

#[derive(Queryable, Insertable, Debug)]
pub struct ItemGroup {
    pub layer: i32,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Queryable, Debug, Clone)]
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

impl NewItem {
    pub fn with(name: String,
                description: Option<String>,
                layer: i32) -> Self {
        NewItem {
            name,
            description,
            layer,
        }
    }
}

impl Item {
    pub fn all(conn: &SqliteConnection) -> QueryResult<Vec<Item>> {
        all_items.order(items::id.asc()).load::<Item>(conn)
    }

    pub fn delete_with_name(name: String, conn: &SqliteConnection) -> QueryResult<usize> {
        diesel::delete(items::table).filter(items::name.eq(name)).execute(conn)
    }

    pub fn insert(item: NewItem, conn: &SqliteConnection) -> QueryResult<usize> {
        diesel::insert_into(items::table).values(&item).execute(conn)
    }
}

impl ItemGroup {
    pub fn with(layer: i32, name: String, description: Option<String>) -> Self {
        ItemGroup {
            layer,
            name,
            description,
        }
    }

    pub fn all(conn: &SqliteConnection) -> QueryResult<Vec<ItemGroup>> {
        all_item_groups.order(item_groups::layer.asc()).load::<ItemGroup>(conn)
    }

    pub fn delete_with_name(name: String, conn: &SqliteConnection) -> QueryResult<usize> {
        diesel::delete(item_groups::table).filter(item_groups::name.eq(name)).execute(conn)
    }

    pub fn insert(item_group: ItemGroup, conn: &SqliteConnection) -> QueryResult<usize> {
        diesel::insert_into(item_groups::table).values(&item_group).execute(conn)
    }
}
