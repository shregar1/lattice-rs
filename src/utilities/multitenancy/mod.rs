use async_trait::async_trait;

#[async_trait]
pub trait BaseTenantIsolationStrategy: Send + Sync {
    fn resolve_tenant_id(&self, context: &str) -> String;
}
pub struct HeaderBasedTenantIsolationStrategy;
