use spacetraders::{
    enums,
    responses::{
        fleet::{CreateChartData, CreateSurveyData},
        schemas::{self, Contract, Ship},
        systems::Waypoints,
    },
    SpaceTraders,
};

pub mod cache;
mod func;
use cache::AllEuclideanDistances;

// use async_recursion::async_recursion;
// use chrono::{offset, serde::ts_milliseconds, DateTime, NaiveDateTime, Utc};
// use itertools::Itertools;
use log::{info, trace};
// use rayon::prelude::*;
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
    pub surveys: Vec<CreateSurveyData>,
    pub waypoints: HashMap<String, schemas::Waypoint>,
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

    let role = ship_handler_data
        .lock()
        .await
        .ships
        .get(ship_id)
        .unwrap()
        .registration
        .role;

    match role {
        enums::ShipRole::Fabricator => todo!(),
        enums::ShipRole::Harvester => todo!(),
        enums::ShipRole::Hauler => contractor_loop().await,
        enums::ShipRole::Interceptor => todo!(),
        enums::ShipRole::Excavator => miner_loop(ship_id, ship_handler_data.clone()).await,
        enums::ShipRole::Transport => todo!(),
        enums::ShipRole::Repair => todo!(),
        enums::ShipRole::Surveyor => todo!(),
        enums::ShipRole::Command => admin_loop(ship_id, ship_handler_data.clone(), channel).await,
        enums::ShipRole::Carrier => todo!(),
        enums::ShipRole::Patrol => todo!(),
        enums::ShipRole::Satellite => explorer_loop().await,
        enums::ShipRole::Explorer => explorer_loop().await,
        enums::ShipRole::Refinery => todo!(),
    };
}

// admin can also loop through contracts and survey to find expired ones and remove them from the list
async fn admin_loop(
    ship_id: &str,
    ship_handler_data: Arc<Mutex<ShipHandlerData>>,
    channel: mpsc::Sender<Ship>,
) {
    loop {
        func::buy_ship(
            ship_id,
            ship_handler_data.clone(),
            &[enums::ShipType::ShipMiningDrone],
            channel.clone(),
        )
        .await;
    }
}

async fn contractor_loop() {
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
}

async fn miner_loop(ship_id: &str, ship_handler_data: Arc<Mutex<ShipHandlerData>>) {
    loop {
        func::mine_astroid(ship_id, ship_handler_data.clone()).await;
    }
}

async fn explorer_loop() {}
