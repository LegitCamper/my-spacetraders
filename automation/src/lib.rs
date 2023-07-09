use spacetraders::{
    enums,
    responses::schemas::{self, Contract, Ship},
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
    pub ships: Vec<Ship>,
    pub contracts: HashMap<String, Contract>,
    pub credits: f64,
    pub euclidean_distances: Vec<AllEuclideanDistances>,
}

pub async fn start_ship_handler(ship_handler_data: Arc<Mutex<ShipHandlerData>>) {
    trace!("Start Ship Handler");
    let (tx, mut rx) = mpsc::channel(100);

    let start_ships = ship_handler_data
        .lock()
        .await
        .spacetraders
        .list_ships()
        .await
        .data;

    // start initial ships in their own task
    for ship in start_ships.into_iter() {
        let new_ship_handler_data = Arc::clone(&ship_handler_data);
        let new_channel = tx.clone();

        info!("Starting new task for ship: {}", ship.symbol);
        let join_handle: task::JoinHandle<()> = task::spawn(async move {
            ship_handler(
                ship.clone(),
                new_ship_handler_data.clone(),
                new_channel.clone(),
            )
            .await
        });
        ship_handler_data.lock().await.handles.push(join_handle);
    }

    // listens for new ship purchases and spawns new task to deal with them
    while let Some(msg) = rx.recv().await {
        let new_ship_handler_data = Arc::clone(&ship_handler_data);
        let new_channel = tx.clone();

        info!("Starting new task for ship: {}", msg.symbol);
        let join_handle: task::JoinHandle<()> = task::spawn(async move {
            ship_handler(
                msg.clone(),
                new_ship_handler_data.clone(),
                new_channel.clone(),
            )
            .await
        });
        ship_handler_data.lock().await.handles.push(join_handle);
    }
}

pub async fn ship_handler(
    ship: Ship,
    ship_handler_data: Arc<Mutex<ShipHandlerData>>,
    channel: mpsc::Sender<Ship>,
) {
    trace!("Ship Handler");

    ship_handler_data.lock().await.ships.push(ship.clone()); // adds itself to ship_handler_data

    match ship.registration.role {
        enums::ShipRole::Fabricator => todo!(),
        enums::ShipRole::Harvester => todo!(),
        enums::ShipRole::Hauler => contractor_loop().await,
        enums::ShipRole::Interceptor => todo!(),
        enums::ShipRole::Excavator => miner_loop(ship, ship_handler_data).await,
        enums::ShipRole::Transport => todo!(),
        enums::ShipRole::Repair => todo!(),
        enums::ShipRole::Surveyor => todo!(),
        enums::ShipRole::Command => admin_loop(ship, ship_handler_data, channel).await,
        enums::ShipRole::Carrier => todo!(),
        enums::ShipRole::Patrol => todo!(),
        enums::ShipRole::Satellite => explorer_loop().await,
        enums::ShipRole::Explorer => explorer_loop().await,
        enums::ShipRole::Refinery => todo!(),
    };
}

async fn admin_loop(
    mut ship: schemas::Ship,
    ship_handler_data: Arc<Mutex<ShipHandlerData>>,
    channel: mpsc::Sender<Ship>,
) {
    loop {
        ship = func::buy_ship(
            ship,
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

async fn miner_loop(mut ship: schemas::Ship, ship_handler_data: Arc<Mutex<ShipHandlerData>>) {
    loop {
        ship = func::mine_astroid(ship, ship_handler_data.clone()).await;
    }
}

async fn explorer_loop() {}
