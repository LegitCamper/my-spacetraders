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
use func::{SharedAutomationData, ShipAutomation};

use log::{info, trace};
use std::{collections::HashMap, sync::Arc};
use tokio::{
    sync::{mpsc, RwLock},
    task,
};

#[derive(Debug)]
pub struct Automation {
    pub handles: Vec<task::JoinHandle<()>>,
    pub ships: HashMap<String, Ship>,
    pub contracts: HashMap<String, Contract>,
    pub surveys: HashMap<WaypointString, Vec<schemas::Survey>>,
    pub waypoints: HashMap<WaypointString, schemas::Waypoint>,
    // TODO: definatly cache market stuff to optimize refuel func
    pub credits: f64,
    pub euclidean_distances: Vec<AllEuclideanDistances>,
}

pub async fn start_ship_handler(st_interface: SpaceTraders, automation_data: Automation) {
    trace!("Start Ship Handler");
    let (tx, mut rx) = mpsc::channel(100);

    let ships = st_interface.list_ships().await.unwrap().data;
    for ship in ships.into_iter() {
        tx.send(ship).await.unwrap();
    }

    // mutable/sharable versions
    // let st_interface = Arc::new(RwLock::new(st_interface));
    // let automation_data = Arc::new(RwLock::new(automation_data));
    let shared_data = Arc::new(RwLock::new(SharedAutomationData {
        st_interface,
        automation_data,
    }));

    // listens for new ship purchases and spawns new task to deal with them
    while let Some(msg) = rx.recv().await {
        let ship_automation = ShipAutomation::new(shared_data.clone(), msg.symbol.as_str());

        ship_automation
            .shared_data
            .write()
            .await
            .automation_data
            .ships
            .insert(msg.symbol.clone(), msg.clone());

        info!("Starting new task for ship: {}", &msg.symbol);
        let join_handle: task::JoinHandle<()> = task::spawn({
            let tx = tx.clone();
            async move { ship_duty(ship_automation, tx).await }
        });
        shared_data
            .write()
            .await
            .automation_data
            .handles
            .push(join_handle);
    }
}

pub async fn ship_duty(ship_automation: ShipAutomation, channel: mpsc::Sender<Ship>) {
    trace!("Ship Handler");

    let role = ship_automation
        .clone_ship()
        .await
        .unwrap()
        .registration
        .role;

    match role {
        enums::ShipRole::Fabricator => todo!(),
        enums::ShipRole::Harvester => todo!(),
        enums::ShipRole::Hauler => contractor_loop(ship_automation, channel).await,
        enums::ShipRole::Interceptor => todo!(),
        enums::ShipRole::Excavator => miner_loop(ship_automation, channel).await,
        enums::ShipRole::Transport => todo!(),
        enums::ShipRole::Repair => todo!(),
        enums::ShipRole::Surveyor => todo!(),
        enums::ShipRole::Command => explorer_loop(ship_automation, channel).await,
        enums::ShipRole::Carrier => todo!(),
        enums::ShipRole::Patrol => todo!(),
        enums::ShipRole::Satellite => explorer_loop(ship_automation, channel).await,
        enums::ShipRole::Explorer => explorer_loop(ship_automation, channel).await,
        enums::ShipRole::Refinery => todo!(),
    };
}

async fn contractor_loop(ship_automation: ShipAutomation, channel: mpsc::Sender<Ship>) {
    loop {
        admin::admin_stuff(
            &ship_automation,
            &[enums::ShipType::ShipMiningDrone],
            channel.clone(),
        )
        .await;
        // contractor::
    }
}

async fn miner_loop(mut ship_automation: ShipAutomation, _channel: mpsc::Sender<Ship>) {
    loop {
        miner::mine_astroid(&ship_automation).await;
        miner::sell_mining_cargo(&mut ship_automation).await;
    }
}

async fn explorer_loop(ship_automation: ShipAutomation, channel: mpsc::Sender<Ship>) {
    loop {
        // admin::admin_stuff(
        //     ship_automation.clone(),
        //     &[enums::ShipType::ShipMiningDrone],
        //     channel.clone(),
        // )
        // .await;
        admin::buy_ship(
            &ship_automation,
            &[enums::ShipType::ShipMiningDrone],
            channel.clone(),
        )
        .await;
        tokio::time::sleep(tokio::time::Duration::from_secs(120)).await;
        // explorer::
    }
}
