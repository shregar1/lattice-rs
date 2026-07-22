pub trait ModelTrait: Send + Sync {
    fn id(&self) -> &str;
}
