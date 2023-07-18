use spacetraders::{
    enums,
    responses::{
        fleet::CreateSurveyData,
        schemas::{self, Contract, Ship},
    },
    SpaceTraders, WaypointString,
};

pub mod admin;
pub mod cache;
pub mod contractor;
pub mod explorer;
mod func;
mod miner;
use cache::AllEuclideanDistances;
use func::ShipDataAbstractor;

use log::{info, trace};
use std::{collections::HashMap, sync::Arc};
use tokio::{
    sync::{mpsc, Mutex},
    task,
};

#[derive(Debug)]
pub struct ShipHandlerData {
    pub handles: Vec<task::JoinHandle<()>>,
    pub spacetraders: SpaceTraders,
    pub ships: HashMap<String, Ship>,
    pub contracts: HashMap<String, Contract>,
    pub surveys: HashMap<WaypointString, CreateSurveyData>,
    pub waypoints: HashMap<WaypointString, schemas::Waypoint>,
    // TODO: definatly cache market stuff to optimize refuel func
    pub credits: f64,
    pub euclidean_distances: Vec<AllEuclideanDistances>,
}

pub async fn start_ship_handler(ship_handler_data: Arc<Mutex<ShipHandlerData>>) {
    trace!("Start Ship Handler");
    let (tx, mut rx) = mpsc::channel(100);

    let ships = ship_handler_data
        .lock()
        .await
        .spacetraders
        .list_ships()
        .await
        .unwrap()
        .data;

    for ship in ships.into_iter() {
        tx.send(ship).await.unwrap();
    }

    // listens for new ship purchases and spawns new task to deal with them
    while let Some(msg) = rx.recv().await {
        let new_ship_handler_data = Arc::clone(&ship_handler_data);
        let channel = tx.clone();

        ship_handler_data
            .lock()
            .await
            .ships
            .insert(msg.symbol.clone(), msg.clone());

        info!("Starting new task for ship: {}", &msg.symbol);
        let join_handle: task::JoinHandle<()> = task::spawn(async move {
            ship_handler(msg.symbol.as_str(), new_ship_handler_data.clone(), channel).await
        });
        ship_handler_data.lock().await.handles.push(join_handle);
    }
}

pub async fn ship_handler(
    ship_id: &str,
    ship_handler_data: Arc<Mutex<ShipHandlerData>>,
    channel: mpsc::Sender<Ship>,
) {
    trace!("Ship Handler");
    let ship_data = ShipDataAbstractor::new(ship_handler_data);

    let role = ship_data
        .clone_ship(ship_id)
        .await
        .unwrap()
        .registration
        .role;

    match role {
        enums::ShipRole::Fabricator => todo!(),
        enums::ShipRole::Harvester => todo!(),
        enums::ShipRole::Hauler => contractor_loop(ship_id, ship_data.clone(), channel).await,
        enums::ShipRole::Interceptor => todo!(),
        enums::ShipRole::Excavator => miner_loop(ship_id, ship_data.clone(), channel).await,
        enums::ShipRole::Transport => todo!(),
        enums::ShipRole::Repair => todo!(),
        enums::ShipRole::Surveyor => todo!(),
        enums::ShipRole::Command => explorer_loop(ship_id, ship_data.clone(), channel).await,
        enums::ShipRole::Carrier => todo!(),
        enums::ShipRole::Patrol => todo!(),
        enums::ShipRole::Satellite => explorer_loop(ship_id, ship_data.clone(), channel).await,
        enums::ShipRole::Explorer => explorer_loop(ship_id, ship_data.clone(), channel).await,
        enums::ShipRole::Refinery => todo!(),
    };
}

async fn contractor_loop(
    ship_id: &str,
    ship_data: ShipDataAbstractor,
    channel: mpsc::Sender<Ship>,
) {
    loop {
        admin::admin_stuff(
            ship_id,
            ship_data.clone(),
            &[enums::ShipType::ShipMiningDrone],
            channel.clone(),
        )
        .await;
        // contractor::
    }
}

async fn miner_loop(ship_id: &str, ship_data: ShipDataAbstractor, _channel: mpsc::Sender<Ship>) {
    loop {
        miner::mine_astroid(ship_id, ship_data.clone()).await;
        miner::sell_mining_cargo(ship_id, ship_data.clone()).await;
    }
}

async fn explorer_loop(ship_id: &str, ship_data: ShipDataAbstractor, channel: mpsc::Sender<Ship>) {
    loop {
        admin::admin_stuff(
            ship_id,
            ship_data.clone(),
            &[enums::ShipType::ShipMiningDrone],
            channel.clone(),
        )
        .await;
        tokio::time::sleep(tokio::time::Duration::from_secs(120)).await;
        // explorer::
    }
}
