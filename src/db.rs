pub mod models;
pub mod schema;
use diesel::prelude::SqliteConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use std::env;

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
}
