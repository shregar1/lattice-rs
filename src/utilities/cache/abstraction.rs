use async_trait::async_trait;

#[async_trait]
pub trait BaseCacheClient: Send + Sync {
    async fn get(&self, key: &str) -> Result<Option<String>, String>;
    async fn set(&self, key: &str, value: &str, ttl_seconds: u64) -> Result<(), String>;
    async fn delete(&self, key: &str) -> Result<bool, String>;
    fn get_driver_name(&self) -> String;
}
