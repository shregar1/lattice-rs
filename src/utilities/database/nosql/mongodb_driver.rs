use super::abstraction::BaseDocumentDatabaseDriver;
use async_trait::async_trait;

pub struct MongoDbDriver;
#[async_trait]
impl BaseDocumentDatabaseDriver for MongoDbDriver {
    async fn insert_document(&self, _collection: &str, _doc: &str) -> Result<String, String> { Ok("mongo_123".to_string()) }
    async fn find_document_by_id(&self, _collection: &str, _id: &str) -> Result<Option<String>, String> { Ok(None) }
    fn get_driver_name(&self) -> String { "mongodb".to_string() }
}
