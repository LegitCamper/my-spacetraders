use spacetraders::{
    responses::{
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
    pub handles: Vec<task::JoinHandle<()>>,
}

pub fn start_ship_handler(
    space_traders: Arc<Mutex<SpaceTraders>>,
    contracts: Arc<Mutex<ShipHandlerData>>,
) -> task::JoinHandle<()> {
    task::spawn(async move {
        let (tx, mut rx) = mpsc::channel(100);

        let start_ships = space_traders.lock().await.list_ships().await.data;

        // start initial ships in their own task
        for ship in start_ships.into_iter() {
            let new_contracts = Arc::clone(&contracts);
            let new_space_traders = Arc::clone(&space_traders);
            let new_channel = tx.clone();

            let join_handle: task::JoinHandle<()> = task::spawn(async move {
                // loop {
                ship_handler(ship, new_contracts, new_space_traders, new_channel).await
                // }
            });
            contracts.lock().await.handles.push(join_handle);
        }

        // listens for new ship purchases and spawns new task to deal with them
        while let Some(msg) = rx.recv().await {
            let new_contracts = Arc::clone(&contracts);
            let new_space_traders = Arc::clone(&space_traders);
            let new_channel = tx.clone();

            let join_handle: task::JoinHandle<()> = task::spawn(async move {
                // loop {
                ship_handler(msg, new_contracts, new_space_traders, new_channel).await
                // }
            });
            contracts.lock().await.handles.push(join_handle);
        }
    })
}

pub async fn ship_handler(
    ship: Ship,
    ship_handler_data: Arc<Mutex<ShipHandlerData>>,
    space_traders: Arc<Mutex<SpaceTraders>>,
    _channel: mpsc::Sender<Ship>,
) {
    ship_handler_data.lock().await.ships.push(ship.clone());

    // println!("{:?}", space_traders.lock().await.agent().await);
    // println!("{:?}", ship_handler_data.lock().await.ships);

    // mine astroids
    // func::mine_astroid(ship, space_traders).await;
    func::buy_mining_ship(ship, space_traders).await;

    // do contracts

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
}
