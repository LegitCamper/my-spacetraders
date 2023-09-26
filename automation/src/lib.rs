use spacetraders::{
    enums::{ShipRole::*, ShipType::*},
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

use chrono::{Duration, Local};
use log::{info, trace};
use std::{collections::HashMap, sync::Arc};
use tokio::{
    runtime::Builder,
    sync::{mpsc, RwLock},
    task::JoinHandle,
    time::{self, sleep},
};

#[derive(Debug)]
pub struct Automation {
    pub handles: HashMap<String, JoinHandle<()>>,
    pub ships: HashMap<String, Ship>,
    pub contracts: HashMap<String, Contract>,
    pub surveys: HashMap<WaypointString, Vec<schemas::Survey>>,
    pub waypoints: HashMap<WaypointString, schemas::Waypoint>,
    // TODO: definatly cache market stuff to optimize refuel func
    pub credits: f64,
    pub euclidean_distances: Vec<AllEuclideanDistances>,
}

pub async fn ship_handler(st_interface: SpaceTraders, mut automation_data: Automation) {
    trace!("Start Ship Handler");
    // this channel if for ships to send back
    // newly purchased ships to be spawned
    let (tx, mut rx) = mpsc::channel(100);
    let runtime = Builder::new_multi_thread()
        .thread_name("SpaceTraders Ship Spawner")
        .enable_all()
        .build()
        .unwrap();

    let ships = st_interface.list_ships().await.unwrap().data;
    for ship in ships.into_iter() {
        tx.send(ship).await.unwrap();
    }

    // inits credits in automation_data
    automation_data.credits = st_interface.agent().await.unwrap().data.credits;

    let shared_data = Arc::new(RwLock::new(SharedAutomationData::new(
        st_interface,
        automation_data,
    )));

    // listens for new ship purchases and spawns new task to deal with them
    let duration = Duration::minutes(1);
    let mut last_print = Local::now();
    loop {
        if let Ok(msg) = rx.try_recv() {
            let ship_automation = ShipAutomation::new(shared_data.clone(), msg.symbol.as_str());

            ship_automation
                .shared_data
                .write()
                .await
                .automation_data
                .ships
                .insert(msg.symbol.clone(), msg.clone());

            info!("Starting new task for ship: {}", &msg.symbol);
            let join_handle: JoinHandle<()> = runtime.spawn({
                let tx = tx.clone();
                async move { ship_duty(ship_automation, tx).await }
            });
            shared_data
                .write()
                .await
                .automation_data
                .handles
                .insert(msg.symbol, join_handle);
        }

        // TODO: Decide when its time to start abandoing ships under producing
        // This also might kill a task about to sell and make money
        // Consider bariers/Task parking
        // if ship is == trash {
        //     task_to_kill = shared_data
        //         .write()
        //         .await
        //         .automation_data
        //         .handles
        //         .get(ship_id)
        //         .unwrap();
        //     task_to_kill.abort()
        // }

        let now = Local::now();
        if now >= last_print + duration {
            last_print = now;
            println!("Current Ships:");
            for (ship_id, ship) in shared_data.read().await.automation_data.ships.iter() {
                println!("ID: {ship_id}, Role: {:?}", ship.registration.role);
            }
            println!(
                "Current Credits: {}",
                shared_data.read().await.automation_data.credits
            );
        }
    }
}

pub async fn ship_duty(ship_automation: ShipAutomation, channel: mpsc::Sender<Ship>) {
    trace!("Ship Handler");

    let role = &ship_automation
        .clone_ship()
        .await
        .unwrap()
        .registration
        .role;

    // TODO: checks if ship is under producing and parks it

    match role {
        Fabricator => todo!(),
        Harvester => todo!(),
        Hauler => todo!(),
        Interceptor => todo!(),
        Excavator => miner_loop(ship_automation, channel).await,
        Transport => todo!(),
        Repair => todo!(),
        Surveyor => todo!(),
        Command => explorer_loop(ship_automation, channel).await,
        Carrier => todo!(),
        Patrol => todo!(),
        Satellite => explorer_loop(ship_automation, channel).await,
        Explorer => explorer_loop(ship_automation, channel).await,
        Refinery => todo!(),
    };
}

async fn contractor_loop(ship_automation: ShipAutomation, channel: mpsc::Sender<Ship>) {
    loop {
        admin::admin_stuff(&ship_automation, &[ShipMiningDrone], channel.clone()).await;
    }
}

async fn miner_loop(mut ship_automation: ShipAutomation, _channel: mpsc::Sender<Ship>) {
    loop {
        miner::mine(&mut ship_automation, false).await;
    }
}

async fn explorer_loop(ship_automation: ShipAutomation, channel: mpsc::Sender<Ship>) {
    loop {
        admin::buy_ship(&ship_automation, &[ShipMiningDrone], &channel).await;
        sleep(time::Duration::from_secs(120)).await;
        // explorer::
    }
}
