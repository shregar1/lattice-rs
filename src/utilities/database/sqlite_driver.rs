use super::driver::DatabaseDriverTrait;
use async_trait::async_trait;

pub struct SqliteDriver { pub file_path: String }
#[async_trait]
impl DatabaseDriverTrait for SqliteDriver {
    async fn connect(&self) -> Result<(), String> { Ok(()) }
    async fn disconnect(&self) -> Result<(), String> { Ok(()) }
    async fn health_check(&self) -> bool { true }
    fn get_driver_name(&self) -> String { "sqlite".to_string() }
}
