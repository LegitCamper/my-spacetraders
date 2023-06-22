use automation;
use spacetraders;

#[tokio::main]
async fn main() {
    let spacetraders = spacetraders::SpaceTraders::default();
    automation::automate(spacetraders.await).await;
}
