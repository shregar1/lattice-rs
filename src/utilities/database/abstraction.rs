use async_trait::async_trait;

#[async_trait]
pub trait BaseDatabaseDriver: Send + Sync {
    async fn connect(&self) -> Result<(), String>;
    async fn disconnect(&self) -> Result<(), String>;
    async fn health_check(&self) -> bool;
    fn get_driver_name(&self) -> String;
}
