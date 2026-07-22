pub trait FactoryTrait<T>: Send + Sync {
    fn create(&self) -> T;
}
