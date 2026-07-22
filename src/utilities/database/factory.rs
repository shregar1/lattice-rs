use super::driver::DatabaseDriverTrait;
use super::sqlite_driver::SqliteDriver;
use super::postgres_driver::PostgresDriver;
use std::sync::Arc;

pub struct DatabaseDriverFactory;

impl DatabaseDriverFactory {
    pub fn create_driver(db_type: &str) -> Arc<dyn DatabaseDriverTrait> {
        match db_type.to_lowercase().as_str() {
            "postgres" | "postgresql" => Arc::new(PostgresDriver { connection_string: "postgres://localhost:5432/lattice".to_string() }),
            _ => Arc::new(SqliteDriver { file_path: ":memory:".to_string() }),
        }
    }
}
