use super::{ShipError, ShipHandlerData};
use spacetraders::{
    enums, requests,
    responses::{self, contracts, schemas},
    SpaceTraders,
};

use async_recursion::async_recursion;
use chrono::{offset, DateTime, Local, Utc};
use log::{info, trace, warn};
use std::sync::Arc;
use std::{collections::HashMap, time};
use tokio::{
    sync::{mpsc, Mutex},
    time::sleep,
};

// this is how you serialize the waypoints
// println!("{:?}", agent.data.headquarters);
// println!("{:?}", agent.data.headquarters.waypoint);
// println!("{:?}", agent.data.headquarters.system);
// println!("{:?}", agent.data.headquarters.sector);

pub async fn wait_duration(time_to_stop: DateTime<Utc>) {
    let local_time_to_stop: DateTime<Local> = time_to_stop.into();
    let local_time_now: DateTime<Local> = offset::Utc::now().into();
    let duration: chrono::Duration = local_time_now - local_time_to_stop;

    sleep(tokio::time::Duration::from_millis(
        (duration.num_milliseconds() + 1).try_into().unwrap(),
    ))
    .await;
}

// mining astroid functions
pub async fn mine_astroid(
    _ship: schemas::Ship,
    _space_traders: Arc<Mutex<SpaceTraders>>,
    ship_handler_data: Arc<Mutex<ShipHandlerData>>,
) {
    trace!("Mining Astroid");
    // return ShipError while in transit so I can release the locks
    // then run again whenever done flying
}

#[async_recursion]
pub async fn buy_mining_ship(
    ship: schemas::Ship,
    space_traders: Arc<Mutex<SpaceTraders>>,
    ship_handler_data: Arc<Mutex<ShipHandlerData>>,
    channel: mpsc::Sender<responses::schemas::Ship>,
) {
    trace!("Buy mining ship");

    // aquire locks to data
    let space_traders_unlocked = space_traders.lock().await;
    let mut ship_handler_data_unlocked = ship_handler_data.lock().await;

    let mut need_to_wait = false;
    let mut time_to_wait: DateTime<Utc> = offset::Local::now().into();

    let ship_details = space_traders_unlocked.get_ship(&ship.symbol).await;

    let waypoints = space_traders_unlocked
        .list_waypoints(ship_details.data.nav.system_symbol)
        .await;

    for waypoint in waypoints.data.iter() {
        for r#trait in waypoint.traits.iter() {
            // info!("fs");
            if r#trait.symbol == enums::WaypointTrait::Shipyard {
                let shipyard = space_traders_unlocked
                    .get_shipyard(waypoint.system_symbol.clone(), waypoint.symbol.clone()) // TODO: implement copy instead
                    .await;

                for ship in shipyard.data.ships.iter() {
                    if ship.r#type == enums::ShipType::ShipMiningDrone {
                        if ship_details.data.nav.waypoint_symbol.waypoint
                            == waypoint.symbol.waypoint
                        {
                            if ship.purchase_price < ship_handler_data_unlocked.credits {
                                let new_ship = space_traders_unlocked
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
                        } else {
                            // there is also a case where the ship is in transit and neither docked or there
                            let ship_status = ship_details.data.nav.status;
                            if ship_status == enums::ShipNavStatus::Docked {
                                space_traders_unlocked
                                    .orbit_ship(&ship_details.data.symbol)
                                    .await;
                            }
                            let time_to_stop = space_traders_unlocked
                                .navigate_ship(
                                    &ship_details.data.symbol,
                                    requests::NavigateShip {
                                        waypoint_symbol: waypoint.symbol.waypoint.clone(),
                                    },
                                )
                                .await;

                            info!(
                                "Moving ship - going to sleep for {} milliseconds",
                                time_to_stop.data.nav.route.arrival
                            );

                            need_to_wait = true;
                            time_to_wait = time_to_stop.data.nav.route.arrival;
                            break;
                        }
                    }
                }
            }
        }
    }

    if need_to_wait {
        drop(space_traders_unlocked);
        drop(ship_handler_data_unlocked);
        wait_duration(time_to_wait).await;
        buy_mining_ship(ship, space_traders, ship_handler_data, channel).await
    } else {
        // else maybe fly to the closest system with a shipyard - TODO: Pathfinding
        warn!("Failed to find Shipyard or suitable ship")
    }
}

// complete contract functions
pub async fn get_contract(space_traders: &SpaceTraders) -> Vec<contracts::schemas::Contract> {
    let available_contracts = space_traders.list_contracts().await.data;

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
            space_traders
                .accept_contract(&available_contracts[0].id)
                .await
                .data
                .contract,
        )
    } else if accepted_contracts.is_empty() && available_contracts.is_empty() {
        panic!("accepted contracts is zero, but there are no contracts to accept")
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
