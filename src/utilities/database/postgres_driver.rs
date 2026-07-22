use super::driver::DatabaseDriverTrait;
use async_trait::async_trait;

pub struct PostgresDriver { pub connection_string: String }
#[async_trait]
impl DatabaseDriverTrait for PostgresDriver {
    async fn connect(&self) -> Result<(), String> { Ok(()) }
    async fn disconnect(&self) -> Result<(), String> { Ok(()) }
    async fn health_check(&self) -> bool { true }
    fn get_driver_name(&self) -> String { "postgres".to_string() }
}
