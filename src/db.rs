use std::env;

use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};

pub mod models;
pub mod schema;

// Use initialize to make a new connection pool and afterward use `get_connection()`to retrieve one connection from the pool
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

    pub fn get_connection(&self) -> PooledConnection<ConnectionManager<SqliteConnection>> {
        let conn = self.pool
                       .clone()
                       .get()
                       .expect("Attempt to get connection timed out");
        conn.execute("PRAGMA foreign_keys = ON")
            .expect("PRAGMA foreign_keys = ON failed");
        conn
    }
}
