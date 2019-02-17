use crate::db::Database;
use lazy_static::lazy_static;

lazy_static!{
    pub static ref DB: Database = Database::initialize();
}