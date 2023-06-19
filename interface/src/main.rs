use automation;
use spacetraders;

#[tokio::main]
async fn main() {
    let interface = spacetraders::SpaceTradersHandler::default().await;  
    automation::automate(interface).await;
}
