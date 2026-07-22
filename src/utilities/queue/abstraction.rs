use async_trait::async_trait;

#[async_trait]
pub trait BaseQueueClient: Send + Sync {
    async fn publish(&self, topic: &str, payload: &str) -> Result<String, String>;
    fn get_driver_name(&self) -> String;
}
