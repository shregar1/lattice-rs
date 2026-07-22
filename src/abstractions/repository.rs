use async_trait::async_trait;

#[async_trait]
pub trait BaseRepositoryTrait<T, ID>: Send + Sync {
    async fn find_by_id(&self, id: ID) -> Result<Option<T>, String>;
}
