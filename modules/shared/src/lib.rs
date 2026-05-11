use async_trait::async_trait;

#[async_trait]
pub trait ZuzModule: Send + Sync {

    fn name(&self) -> &str;

    async fn init(&self) -> Result<(), Box<dyn std::error::Error>>;

    async fn start(&self);
    
}