use super::abstraction::BaseQueueClient;
use async_trait::async_trait;

pub struct InMemoryQueueDriver;
#[async_trait]
impl BaseQueueClient for InMemoryQueueDriver {
    async fn publish(&self, _topic: &str, _payload: &str) -> Result<String, String> { Ok("msg_123".to_string()) }
    fn get_driver_name(&self) -> String { "in_memory".to_string() }
}
