use automation;
use spacetraders;

#[tokio::main]
async fn main() {
    let interface = spacetraders::SpaceTraders::default().await;
    automation::automate(interface).await;
}
