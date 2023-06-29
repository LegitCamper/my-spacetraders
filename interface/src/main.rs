use automation;
use spacetraders;

use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let space_traders = Arc::new(Mutex::new(spacetraders::SpaceTraders::default().await));
    let contracts_space_traders = Arc::clone(&space_traders);

    let contracts = Arc::new(Mutex::new(automation::ShipHandlerData {
        ships: vec![],
        contracts: HashMap::new(),
        interface: contracts_space_traders,
        handles: vec![],
    }));

    let automation_contracts = Arc::clone(&contracts);
    automation::start_ship_handler(automation_contracts)
        .await
        .await
        .unwrap();
}
