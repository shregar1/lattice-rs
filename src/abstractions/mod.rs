//! Lattice-Rs Abstractions Module
//! Base traits for Models, Repositories, Services, Orchestrators, and Controllers.

use async_trait::async_trait;

pub trait BaseModelTrait {
    fn id(&self) -> &str;
    fn urn(&self) -> &str;
}

#[async_trait]
pub trait BaseRepositoryTrait<T, ID>: Send + Sync {
    async fn find_by_id(&self, id: ID) -> Result<Option<T>, String>;
}

pub trait BaseServiceTrait: Send + Sync {}
pub trait BaseOrchestratorTrait: Send + Sync {}
