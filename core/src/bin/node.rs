use zuz::config::VERSION;
use zuz_shared::ZuzModule;
use zuz_base::BaseModule;
use std::sync::Arc;

#[tokio::main]
async fn main() {

    let registry: Vec<Arc<dyn ZuzModule>> = vec![
        Arc::new(BaseModule),
        // Arc::new(AuthModule), 
    ];

    for module in registry {
        module.init().await.unwrap();
        module.start().await;
    }

    println!("Zuz version {} is running!", VERSION);

}