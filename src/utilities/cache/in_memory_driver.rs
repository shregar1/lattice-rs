use super::abstraction::BaseCacheClient;
use async_trait::async_trait;

pub struct InMemoryCacheDriver;
#[async_trait]
impl BaseCacheClient for InMemoryCacheDriver {
    async fn get(&self, _key: &str) -> Result<Option<String>, String> { Ok(None) }
    async fn set(&self, _key: &str, _value: &str, _ttl: u64) -> Result<(), String> { Ok(()) }
    async fn delete(&self, _key: &str) -> Result<bool, String> { Ok(true) }
    fn get_driver_name(&self) -> String { "in_memory".to_string() }
}
