use async_trait::async_trait;

#[async_trait]
pub trait BaseMigration: Send + Sync {
    async fn up(&self) -> Result<(), String>;
    async fn down(&self) -> Result<(), String>;
}
pub struct MigrationRunner;
