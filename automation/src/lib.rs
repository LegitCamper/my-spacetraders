use spacetraders::{
    enums,
    responses::schemas::{self, Contract, Ship},
    SpaceTraders, WaypointString,
};

pub mod admin;
pub mod cache;
pub mod contractor;
pub mod explorer;
mod func;
mod miner;
use cache::AllEuclideanDistances;
use func::ShipWrapper;

use log::{info, trace};
use std::{collections::HashMap, sync::Arc};
use tokio::{
    sync::{mpsc, RwLock},
    task,
};

#[derive(Debug)]
pub struct ShipHandler {
    pub handles: Vec<task::JoinHandle<()>>,
    pub spacetraders: SpaceTraders,
    pub ships: HashMap<String, Ship>,
    pub contracts: HashMap<String, Contract>,
    pub surveys: HashMap<WaypointString, Vec<schemas::Survey>>,
    pub waypoints: HashMap<WaypointString, schemas::Waypoint>,
    // TODO: definatly cache market stuff to optimize refuel func
    pub credits: f64,
    pub euclidean_distances: Vec<AllEuclideanDistances>,
}

pub async fn start_ship_handler(ship_handler_data: Arc<RwLock<ShipHandler>>) {
    trace!("Start Ship Handler");
    let (tx, mut rx) = mpsc::channel(100);

    let ships = ship_handler_data
        .read()
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
        let new_ship_handler = ship_handler_data.clone();
        ship_handler_data
            .write()
            .await
            .ships
            .insert(msg.symbol.clone(), msg.clone());

        info!("Starting new task for ship: {}", &msg.symbol);
        let join_handle: task::JoinHandle<()> = task::spawn({
            let tx = tx.clone();
            async move { ship_handler(msg.symbol.as_str(), new_ship_handler, tx).await }
        });
        ship_handler_data.write().await.handles.push(join_handle);
    }
}

pub async fn ship_handler(
    ship_id: &str,
    ship_handler_data: Arc<RwLock<ShipHandler>>,
    channel: mpsc::Sender<Ship>,
) {
    trace!("Ship Handler");
    let ship_data = ShipWrapper::new(ship_handler_data, ship_id);

    let role = ship_data.clone_ship().await.unwrap().registration.role;

    match role {
        enums::ShipRole::Fabricator => todo!(),
        enums::ShipRole::Harvester => todo!(),
        enums::ShipRole::Hauler => contractor_loop(ship_data, channel).await,
        enums::ShipRole::Interceptor => todo!(),
        enums::ShipRole::Excavator => miner_loop(ship_data, channel).await,
        enums::ShipRole::Transport => todo!(),
        enums::ShipRole::Repair => todo!(),
        enums::ShipRole::Surveyor => todo!(),
        enums::ShipRole::Command => explorer_loop(ship_data, channel).await,
        enums::ShipRole::Carrier => todo!(),
        enums::ShipRole::Patrol => todo!(),
        enums::ShipRole::Satellite => explorer_loop(ship_data, channel).await,
        enums::ShipRole::Explorer => explorer_loop(ship_data, channel).await,
        enums::ShipRole::Refinery => todo!(),
    };
}

async fn contractor_loop(ship_data: ShipWrapper, channel: mpsc::Sender<Ship>) {
    loop {
        admin::admin_stuff(
            &ship_data,
            &[enums::ShipType::ShipMiningDrone],
            channel.clone(),
        )
        .await;
        // contractor::
    }
}

async fn miner_loop(ship_data: ShipWrapper, _channel: mpsc::Sender<Ship>) {
    loop {
        miner::mine_astroid(&ship_data).await;
        miner::sell_mining_cargo(&ship_data).await;
    }
}

async fn explorer_loop(ship_data: ShipWrapper, channel: mpsc::Sender<Ship>) {
    loop {
        // admin::admin_stuff(
        //     ship_data.clone(),
        //     &[enums::ShipType::ShipMiningDrone],
        //     channel.clone(),
        // )
        // .await;
        admin::buy_ship(
            &ship_data,
            &[enums::ShipType::ShipMiningDrone],
            channel.clone(),
        )
        .await;
        tokio::time::sleep(tokio::time::Duration::from_secs(120)).await;
        // explorer::
    }
}
