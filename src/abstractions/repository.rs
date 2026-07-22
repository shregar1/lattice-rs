use async_trait::async_trait;

#[async_trait]
pub trait BaseRepositoryTrait<T, ID>: Send + Sync {
    async fn find_by_id(&self, id: ID) -> Result<Option<T>, String>;
    async fn find_by_urn(&self, urn: &str) -> Result<Option<T>, String>;
    async fn create(&self, entity: T) -> Result<T, String>;
    async fn update(&self, id: ID, entity: T) -> Result<T, String>;
    async fn delete(&self, id: ID) -> Result<bool, String>;
    async fn soft_delete(&self, id: ID) -> Result<bool, String>;
    async fn restore(&self, id: ID) -> Result<bool, String>;
}
