pub mod models;
pub mod schema;
use self::models::*;
use self::schema::*;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use std::env;
use std::ops::Deref;

// Use initialize to make a new connection pool and aftewards use connect to get a pooled connection
pub struct Database {
    pub pool: Pool<ConnectionManager<SqliteConnection>>,
}

impl Database {
    pub fn initialize() -> Self {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let conn_manager = ConnectionManager::<SqliteConnection>::new(database_url);
        let pool = Pool::builder()
            .max_size(10)
            .build(conn_manager)
            .expect("Connection pool couldn't been made");
        Database { pool }
    }

    pub fn connect(&self) -> PooledConnection<ConnectionManager<SqliteConnection>> {
        let conn = self
            .pool
            .clone()
            .get()
            .expect("Attempt to get connection timed out");
        conn.execute("PRAGMA foreign_keys = ON")
            .expect("PRAGMA foreign_keys = ON failed");
        conn
    }
    //TODO: more generic interface for inserting/deleting/updating/listing? Perhaps someday
    pub fn new_item_group(
        &self,
        layer: i32,
        name: String,
        description: Option<String>,
    ) -> QueryResult<usize> {
        let item_group = ItemGroup {
            layer,
            name,
            description,
        };

        diesel::insert_into(item_groups::table)
            .values(&item_group)
            .execute(self.connect().deref())
    }

    pub fn del_item_group(&self, name: String) -> QueryResult<usize> {
        diesel::delete(item_groups::table)
            .filter(self::schema::item_groups::name.like(name))
            .execute(self.connect().deref())
    }

    pub fn list_item_groups(&self) -> QueryResult<Vec<ItemGroup>> {
        self::schema::item_groups::table.load::<ItemGroup>(self.connect().deref())
    }

    pub fn new_item(
        &self,
        name: String,
        description: Option<String>,
        layer: i32,
    ) -> QueryResult<usize> {
        let item = NewItem {
            name,
            description,
            layer,
        };
        diesel::insert_into(items::table)
            .values(&item)
            .execute(self.connect().deref())
    }

    pub fn del_item(&self, name: String) -> QueryResult<usize> {
        diesel::delete(items::table)
            .filter(self::schema::items::name.like(name))
            .execute(self.connect().deref())
    }

    pub fn list_items(&self) -> QueryResult<Vec<Item>> {
        self::schema::items::table.load::<Item>(self.connect().deref())
    }
}
