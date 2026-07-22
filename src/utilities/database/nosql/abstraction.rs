use async_trait::async_trait;

#[async_trait]
pub trait BaseDocumentDatabaseDriver: Send + Sync {
    async fn insert_document(&self, collection: &str, doc_json: &str) -> Result<String, String>;
    async fn find_document_by_id(&self, collection: &str, id: &str) -> Result<Option<String>, String>;
    fn get_driver_name(&self) -> String;
}
