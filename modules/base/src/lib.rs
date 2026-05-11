use zuz_shared::ZuzModule;
use async_trait::async_trait;

pub struct BaseModule;

#[async_trait]
impl ZuzModule for BaseModule {
    
    fn name(&self) -> &str {
        "BaseModule"
    }

    async fn init(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("BaseModule initialized!");
        Ok(())
    }

    async fn start(&self) {
        println!("BaseModule started!");
    }

}