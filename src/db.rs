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
        self.pool
            .clone()
            .get()
            .expect("Attempt to get connection timed out")
    }

    pub fn new_item_group(
        &self,
        layer: i32,
        name: String,
        description: Option<String>,
    ) -> QueryResult<Option<usize>> {
        //  use crate::db::schema::item_groups::dsl::*;
        let item_group = ItemGroup {
            layer,
            name,
            description,
        };

        diesel::insert_or_ignore_into(item_groups::table)
            .values(&item_group)
            .execute(self.connect().deref())
            .optional()
    }

    pub fn del_item_group(&self, name: String) -> QueryResult<usize>
    {
        diesel::delete(item_groups::table)
            .filter(self::schema::item_groups::name.like(name))
            .execute(self.connect().deref())
    }
    
    pub fn list_item_groups(&self) -> QueryResult<Vec<ItemGroup>>
    {
        self::schema::item_groups::table.load::<ItemGroup>(self.connect().deref())
    }
}
