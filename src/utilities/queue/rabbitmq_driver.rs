use super::abstraction::BaseQueueClient;
use async_trait::async_trait;

pub struct RabbitMQQueueDriver;
#[async_trait]
impl BaseQueueClient for RabbitMQQueueDriver {
    async fn publish(&self, _topic: &str, _payload: &str) -> Result<String, String> { Ok("amqp_123".to_string()) }
    fn get_driver_name(&self) -> String { "rabbitmq".to_string() }
}
