use spacetraders::{
    responses::{
        schemas::{self, Contract, Ship},
        systems,
    },
    SpaceTraders,
};
mod func;

use log::{info, trace, warn};
use std::{collections::HashMap, sync::Arc};
use tokio::{
    sync::{mpsc, Mutex},
    task,
};

pub enum ShipError {
    ShipInTransit,
}

#[derive(Debug)]
pub struct ShipHandlerData {
    pub ships: Vec<Ship>,
    pub contracts: HashMap<String, Contract>,
    pub handles: Vec<task::JoinHandle<()>>,
    pub credits: f64,
    pub systems_db: HashMap<String, schemas::System>,
}

pub fn start_ship_handler(
    space_traders: Arc<Mutex<SpaceTraders>>,
    ship_handler_data: Arc<Mutex<ShipHandlerData>>,
) -> task::JoinHandle<()> {
    trace!("Start Ship Handler");
    task::spawn(async move {
        let (tx, mut rx) = mpsc::channel(100);

        let start_ships = space_traders.lock().await.list_ships().await.data;

        // start initial ships in their own task
        for ship in start_ships.into_iter() {
            let new_ship_handler_data = Arc::clone(&ship_handler_data);
            let new_space_traders = Arc::clone(&space_traders);
            let new_channel = tx.clone();

            info!("Starting new task for ship: {}", ship.symbol);
            let join_handle: task::JoinHandle<()> = task::spawn(async move {
                loop {
                    ship_handler(
                        ship.clone(),
                        new_ship_handler_data.clone(),
                        new_space_traders.clone(),
                        new_channel.clone(),
                    )
                    .await
                }
            });
            ship_handler_data.lock().await.handles.push(join_handle);
        }

        // listens for new ship purchases and spawns new task to deal with them
        while let Some(msg) = rx.recv().await {
            let new_ship_handler_data = Arc::clone(&ship_handler_data);
            let new_space_traders = Arc::clone(&space_traders);
            let new_channel = tx.clone();

            info!("Starting new task for ship: {}", msg.symbol);
            let join_handle: task::JoinHandle<()> = task::spawn(async move {
                loop {
                    ship_handler(
                        msg.clone(),
                        new_ship_handler_data.clone(),
                        new_space_traders.clone(),
                        new_channel.clone(),
                    )
                    .await
                }
            });
            ship_handler_data.lock().await.handles.push(join_handle);
        }
    })
}

pub async fn ship_handler(
    ship: Ship,
    ship_handler_data: Arc<Mutex<ShipHandlerData>>,
    space_traders: Arc<Mutex<SpaceTraders>>,
    channel: mpsc::Sender<Ship>,
) {
    trace!("Ship Handler");
    ship_handler_data.lock().await.ships.push(ship.clone()); // adds itself to ship_handler_data

    // mine astroids
    func::buy_mining_ship(
        ship.clone(),
        space_traders.clone(),
        ship_handler_data.clone(),
        channel,
    )
    .await;
    func::mine_astroid(
        ship.clone(),
        space_traders.clone(),
        ship_handler_data.clone(),
    )
    .await;

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

pub async fn build_system_db(
    space_traders: Arc<Mutex<SpaceTraders>>,
) -> HashMap<String, schemas::System> {
    trace!("Building system DB");

    // using the status endpoint find the last day the server was reset and
    // regen the file if been reset since then

    // aquire locks
    let space_traders_unlocked = space_traders.lock().await;

    let list_systems_meta = space_traders_unlocked.list_systems(None).await;
    info!(
        "There are {} systems - Building will take ~{} minute(s)",
        list_systems_meta.meta.total, // I use a static 20 per page,
        (list_systems_meta.meta.total / 40) / 60
    );

    let mut systems: HashMap<String, schemas::System> = HashMap::new();

    for page in 1..((list_systems_meta.meta.total / 20) + 1) {
        for system in space_traders_unlocked
            .list_systems(Some(page))
            .await
            .data
            .iter()
        {
            systems.insert(system.symbol.clone(), system.clone());
        }
    }
    info!("{} systems in db", systems.len());
    systems
}
