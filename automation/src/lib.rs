use spacetraders::{
    enums::ShipEngine,
    responses::{
        contracts,
        schemas::{Contract, Ship},
    },
    SpaceTraders,
};
mod func;

use std::{collections::HashMap, sync::Arc};
use tokio::{
    sync::{mpsc, Mutex},
    task,
};

#[derive(Debug)]
pub struct ShipHandlerData {
    pub ships: Vec<Ship>,
    pub contracts: HashMap<String, Contract>,
    pub interface: Arc<Mutex<SpaceTraders>>,
    // pub handles: Vec<task::JoinHandle<()>>,
    pub handles: Vec<()>,
}

pub async fn start_ship_handler(contracts: Arc<Mutex<ShipHandlerData>>) -> task::JoinHandle<()> {
    task::spawn(async move {
        let (tx, mut rx) = mpsc::channel(100);

        for ship in contracts.lock().await.ships.clone().into_iter() {
            let new_contracts = Arc::clone(&contracts);
            let new_channel = tx.clone();
            let handle =
                task::spawn(async move { ship_handler(ship, new_contracts, new_channel) }).await;
            // .await
            // .unwrap()
            // .await;
            contracts.lock().await.handles.push(handle.unwrap().await);
        }

        while let Some(msg) = rx.recv().await {
            let new_channel = tx.clone();
            let new_contracts = Arc::clone(&contracts);
            task::spawn(async move { ship_handler(msg, new_contracts, new_channel) })
                .await
                .unwrap()
                .await;
        }
    })
}

pub async fn ship_handler(
    ship: Ship,
    ship_handler_data: Arc<Mutex<ShipHandlerData>>,
    channel: mpsc::Sender<Ship>,
) {
    let space_traders = ship_handler_data.lock().await.interface.clone();
    automate(space_traders).await;
}

pub async fn automate(space_traders: Arc<Mutex<SpaceTraders>>) {
    println!("{:?}", space_traders.lock().await.agent().await);
    // loop {

    // let accepted_contracts = func::get_contract(&space_traders.lock().await).await;

    // println!("{:?}", accepted_contracts);
    // for current_contract in accepted_contracts {
    // println!(
    //     "{:?}",
    //     func::get_contract_ship(&spacetraders, current_contract).await
    // );

    // for current_contract in accepted.iter() {
    // if !have_correct_ship {
    // func::buy_ship(&spacetraders);
    // }
    // }
    // }
}
