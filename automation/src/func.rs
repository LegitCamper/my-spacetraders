use super::{ShipError, ShipHandlerData};
use spacetraders::{
    enums, requests,
    responses::{self, contracts, schemas},
    SpaceTraders, System, Waypoint,
};

use async_recursion::async_recursion;
use chrono::{offset, DateTime, Local, Utc};
use itertools::Itertools;
use log::{info, trace, warn};
use std::{collections::HashMap, time};
use std::{str::ParseBoolError, sync::Arc};
use tokio::{
    sync::{mpsc, Mutex},
    time::{sleep, Duration},
};

pub async fn wait_duration(time_to_stop: DateTime<Utc>) {
    trace!("Waiting duration");

    let local_time_to_stop: DateTime<Local> = time_to_stop.into();
    let local_time_now: DateTime<Local> = offset::Utc::now().into();
    let duration: chrono::Duration = local_time_to_stop - local_time_now;

    info!(
        "Moving ship - going to sleep for {} seconds", // TODO: maybe log what ship is sleeping
        duration.num_seconds()
    );

    sleep(Duration::from_secs(
        duration.num_seconds().try_into().unwrap(),
    ))
    .await;
}

pub async fn travel_waypoint(
    ship: schemas::Ship,
    ship_handler_data: Arc<Mutex<ShipHandlerData>>,
    waypoint: Waypoint,
) {
    trace!("travel");

    let ship_details = ship_handler_data
        .lock()
        .await
        .spacetraders
        .get_ship(&ship.symbol)
        .await;

    // TODO: refuel before traveling
    if ship_details.data.nav.waypoint_symbol.waypoint != waypoint.waypoint {
        // there is also a case where the ship is in transit and neither docked or there
        let ship_status = ship_details.data.nav.status;
        if ship_status == enums::ShipNavStatus::Docked {
            ship_handler_data
                .lock()
                .await
                .spacetraders
                .orbit_ship(&ship_details.data.symbol)
                .await;
        }
        //TODO: consider fuel types here
        let time_to_stop = ship_handler_data
            .lock()
            .await
            .spacetraders
            .navigate_ship(
                &ship_details.data.symbol,
                requests::NavigateShip {
                    waypoint_symbol: waypoint.waypoint.clone(),
                },
            )
            .await;

        wait_duration(time_to_stop.data.nav.route.arrival).await;
    }
}

pub async fn mine_astroid(ship: schemas::Ship, ship_handler_data: Arc<Mutex<ShipHandlerData>>) {
    trace!("Mining Astroid");

    let ship_details = ship_handler_data
        .lock()
        .await
        .spacetraders
        .get_ship(&ship.symbol)
        .await;

    let waypoints = ship_handler_data
        .lock()
        .await
        .spacetraders
        .list_waypoints(ship_details.data.nav.system_symbol)
        .await;

    for waypoint in waypoints.data.iter() {
        if waypoint.r#type == enums::WaypointType::AsteroidField {
            travel_waypoint(
                ship.clone(),
                ship_handler_data.clone(),
                waypoint.symbol.clone(),
            )
            .await;

            ship_handler_data
                .lock()
                .await
                .spacetraders
                .orbit_ship(&ship.symbol)
                .await;

            info!("Starting mining astroid");

            // check if ship can survey and do survey before mining
            ship_handler_data
                .lock()
                .await
                .spacetraders
                .extract_resources(&ship.symbol, None)
                .await;

            for waypoint in waypoints.data.iter() {
                for r#trait in waypoint.traits.iter() {
                    if r#trait.symbol == enums::WaypointTrait::Marketplace {
                        travel_waypoint(
                            ship.clone(),
                            ship_handler_data.clone(),
                            waypoint.symbol.clone(),
                        )
                        .await;

                        ship_handler_data
                            .lock()
                            .await
                            .spacetraders
                            .dock_ship(&ship.symbol)
                            .await;

                        // aquire locks
                        let mut ship_handler_data_unlocked = ship_handler_data.lock().await;

                        // TODO: make sure not to sell goods used for contracts
                        // TODO: also make sure I can sell that good here
                        let cargo = ship_handler_data_unlocked
                            .spacetraders
                            .get_ship_cargo(&ship.symbol)
                            .await;

                        for item in cargo.data.inventory.iter() {
                            info!(
                                "{} is selling {} {:?}",
                                ship.symbol, item.units, item.symbol
                            );

                            let transaction = ship_handler_data_unlocked
                                .spacetraders
                                .sell_cargo(
                                    &ship.symbol,
                                    requests::SellCargo {
                                        symbol: item.symbol.clone(),
                                        units: item.units,
                                    },
                                )
                                .await;

                            ship_handler_data_unlocked.credits = ship_handler_data_unlocked.credits
                                + transaction.data.transaction.units
                        }
                        return; // lock should be released here
                    }
                }
            }
        }
    }

    // else maybe fly to the closest system with a shipyard - TODO: Pathfinding
    warn!("Failed to find asteroid");
}

pub async fn buy_ship(
    ship: schemas::Ship,
    ship_handler_data: Arc<Mutex<ShipHandlerData>>,
    ship_types: &[enums::ShipType],
    channel: mpsc::Sender<responses::schemas::Ship>,
) {
    trace!("Buy mining ship");

    let ship_details = ship_handler_data
        .lock()
        .await
        .spacetraders
        .get_ship(&ship.symbol)
        .await;

    let waypoints = ship_handler_data
        .lock()
        .await
        .spacetraders
        .list_waypoints(ship_details.data.nav.system_symbol)
        .await;

    'outer: for waypoint in waypoints.data.iter() {
        for r#trait in waypoint.traits.iter() {
            if r#trait.symbol == enums::WaypointTrait::Shipyard {
                travel_waypoint(ship, ship_handler_data.clone(), waypoint.symbol.clone()).await;

                let mut ship_handler_data_unlocked = ship_handler_data.lock().await;

                let shipyard = ship_handler_data_unlocked
                    .spacetraders
                    .get_shipyard(waypoint.system_symbol.clone(), waypoint.symbol.clone()) // TODO: implement copy instead
                    .await;

                for ship in shipyard.data.ships.iter() {
                    for ship_type in ship_types {
                        if ship.r#type == ship_type.to_owned() {
                            if ship.purchase_price < ship_handler_data_unlocked.credits {
                                let new_ship = ship_handler_data_unlocked
                                    .spacetraders
                                    .purchase_ship(requests::PurchaseShip {
                                        ship_type: ship.r#type,
                                        waypoint_symbol: waypoint.symbol.clone().waypoint,
                                    })
                                    .await;

                                channel.send(new_ship.data.ship.clone()).await.unwrap();

                                ship_handler_data_unlocked.credits = ship_handler_data_unlocked
                                    .credits
                                    - new_ship.data.transaction.price;

                                info!(
                                    "buying ship, now at {} credits",
                                    ship_handler_data_unlocked.credits
                                );
                                return;
                            } else {
                                warn!("Not enough money to buy ship");
                                return;
                            }
                        }
                    }
                }
                break 'outer;
            }
        }
    }
    // else maybe fly to the closest system with a shipyard - TODO: Pathfinding
    warn!("Failed to find Shipyard or suitable ship");
}

// TODO: this has to be super unoptimized. takes way to long
fn euclidean_distance(
    current_system: schemas::System,
    systems: HashMap<String, schemas::System>,
    num_returns: Option<u32>,
) -> HashMap<u64, (i32, i32)> {
    trace!("Euclidean Distance Caluclations");
    let num_systems_to_return = match num_returns {
        Some(num) => num,
        None => 5,
    };

    let mut closest_systems: HashMap<u64, (i32, i32)> =
        HashMap::with_capacity(num_systems_to_return.try_into().unwrap());
    let (my_x, my_y) = (current_system.x, current_system.y);

    for (_, system) in systems.iter() {
        let (system_x, system_y) = (system.x, system.y);

        let distance: f64 = ((my_x as f64 - my_y as f64).powi(2)
            + (system_x as f64 - system_y as f64).powi(2))
        .sqrt();
        // giving up trying to do this with floats
        // I am going to round and hope it works out
        let distance: u64 = distance.round() as u64;

        if closest_systems.len() < num_systems_to_return.try_into().unwrap() {
            closest_systems.insert(distance, (system_x, system_y));
        } else {
            'inner: for system_distance in closest_systems.clone().keys().sorted() {
                if distance < *system_distance || distance == *system_distance {
                    closest_systems.remove(&distance);
                    closest_systems.insert(distance, (system_x, system_y));
                    break 'inner;
                }
            }
        }
    }
    closest_systems
}

pub async fn explore(ship_handler_data: Arc<Mutex<ShipHandlerData>>) {}

pub async fn get_contracts(
    ship_handler_data: Arc<Mutex<ShipHandlerData>>,
) -> Vec<contracts::schemas::Contract> {
    let available_contracts = ship_handler_data
        .lock()
        .await
        .spacetraders
        .list_contracts()
        .await
        .data;

    let mut accepted_contracts: Vec<contracts::schemas::Contract> = vec![];
    for contract in available_contracts.iter() {
        if contract.accepted {
            accepted_contracts.push(contract.clone())
        }
    }
    if accepted_contracts.is_empty() {
        // TODO // should except the contract that is expiring first!
        // or that gives the most rep points
        // for now just excepting the first contract for simplicity
        accepted_contracts.push(
            ship_handler_data
                .lock()
                .await
                .spacetraders
                .accept_contract(&available_contracts[0].id)
                .await
                .data
                .contract,
        )
    } else if accepted_contracts.is_empty() && available_contracts.is_empty() {
        warn!("accepted contracts is zero, but there are no contracts to accept");
    }
    accepted_contracts
}

// pub async fn get_contract_ship(
//     space_traders: &SpaceTraders,
//     contract: schemas::Contract,
// ) -> schemas::Ship {
//     let my_ships = space_traders.list_ships().await;

//     for ship in my_ships.data.iter() {
//         match contract.r#type {
//             enums::ListContractsType::Procurement => {
//                 // Need mining ship // probably
//                 if ship.registration.role == enums::ShipRole::Excavator
//                     || ship.registration.role == enums::ShipRole::Harvester
//                 {
//                     return ship.clone();
//                 } else {
//                     // TODO: find ship that matches role and buy that one
//                     return buy_ship(&space_traders, enums::ShipType::ShipMiningDrone).await;
//                 }
//             }
//             enums::ListContractsType::Transport => {
//                 // probably need figigate or hauler ship
//                 if ship.registration.role == enums::ShipRole::Hauler
//                     || ship.registration.role == enums::ShipRole::Carrier
//                 {
//                     return ship.clone();
//                 } else {
//                     // TODO: find ship that matches role and buy that one
//                     return buy_ship(&space_traders, enums::ShipType::ShipLightHauler).await;
//                 }
//             }
//             enums::ListContractsType::Shuttle => {
//                 // probaly need shuttle
//                 if ship.registration.role == enums::ShipRole::Carrier
//                     || ship.registration.role == enums::ShipRole::Transport
//                 {
//                     return ship.clone();
//                 } else {
//                     // TODO: find ship that matches role and buy that one
//                     return buy_ship(&space_traders, enums::ShipType::ShipHeavyFreighter).await;
//                 }
//             }
//         }
//     }
//     my_ships.data[0].clone()
// }

// pub async fn purchase_ship(space_traders: &SpaceTraders, _ship: enums::ShipType) -> schemas::Ship {
//     let mut found_shipyard = false;
//     let system = parse_waypoint(&space_traders.agent().await.data.headquarters).system;

//     for waypoint in space_traders.list_waypoints(&system).await.data.iter() {
//         for waypoint_trait in waypoint.traits.iter() {
//             if waypoint_trait.symbol == enums::WaypointTrait::Shipyard {
//                 found_shipyard = true;
//                 let parsed_waypoint = parse_waypoint(&waypoint.symbol);
//                 let ships_in_shipyard = space_traders
//                     .get_shipyard(&parsed_waypoint.system, &parsed_waypoint.waypoint)
//                     .await;

//                 for ship in ships_in_shipyard.data.ship_types.iter() {
//                     // match current_contract.r#type {
//                     // should buy the correct ship for the contract. for now I will just buy mining drone
//                     if ship.r#type == enums::ShipType::ShipMiningDrone {
//                         // ListContractsType::Procurement => {
//                         //     if ship.
//                         // }
//                         // ListContractsType::Transport => {}
//                         // ListContractsType::Shuttle => {}
//                         let u = space_traders
//                             .purchase_ship(requests::PurchaseShip {
//                                 ship_type: ship.r#type,
//                                 waypoint_symbol: waypoint.symbol.clone(),
//                             })
//                             .await;
//                         break;
//                     }
//                 }
//                 break;
//             }
//         }
//     }
//     if !found_shipyard {
//         space_traders.diagnose();
//     }
//     panic!("WORKED")
// }
