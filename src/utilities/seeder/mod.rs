use async_trait::async_trait;

#[async_trait]
pub trait BaseSeeder: Send + Sync {
    async fn seed(&self) -> Result<(), String>;
}
pub struct SeederRunner;
