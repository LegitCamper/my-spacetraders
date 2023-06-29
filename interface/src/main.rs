use automation::{self, start_ship_handler, ShipHandlerData};
use spacetraders;

use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let space_traders = Arc::new(Mutex::new(spacetraders::SpaceTraders::default().await));
    // let contracts_space_traders = Arc::clone(&space_traders);

    let ship_handler_data = Arc::new(Mutex::new(ShipHandlerData {
        ships: vec![],
        contracts: HashMap::new(),
        handles: vec![],
    }));

    let automation_contracts = Arc::clone(&ship_handler_data);
    let ship_handler = start_ship_handler(space_traders, automation_contracts).await;
    ship_handler.unwrap();
}
