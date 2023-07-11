use super::ShipHandlerData;
use spacetraders::{
    enums::{self, ShipModule},
    requests,
    responses::{self, schemas}, //contracts
    // SpaceTraders,
    System,
    Waypoint,
};

// use async_recursion::async_recursion;
use chrono::{offset, DateTime, Local};
use log::{info, trace, warn};
use std::sync::Arc;
use tokio::{
    sync::{mpsc, Mutex},
    time::{sleep, Duration},
};

// TODO: implement copy instead for all custom structs

async fn wait_duration(ship_id: String, ship_handler_data: Arc<Mutex<ShipHandlerData>>) {
    trace!("Waiting duration");

    let local_time_to_stop: DateTime<Local> = ship_handler_data
        .lock()
        .await
        .ships
        .get(&ship_id)
        .unwrap()
        .nav
        .route
        .arrival
        .into();
    let local_time_now: DateTime<Local> = offset::Utc::now().into();
    let duration: chrono::Duration = local_time_to_stop - local_time_now;

    info!(
        "{} is moving - going to sleep for {} seconds",
        ship_handler_data
            .lock()
            .await
            .ships
            .get(&ship_id)
            .unwrap()
            .symbol,
        duration.num_seconds()
    );

    sleep(Duration::from_secs(
        duration.num_seconds().try_into().unwrap(),
    ))
    .await;
}

pub async fn travel_waypoint(
    ship_id: String,
    ship_handler_data: Arc<Mutex<ShipHandlerData>>,
    waypoint: Waypoint,
) {
    trace!("Travel Waypoint");

    // TODO: create chart here if not in mutex

    // TODO: refuel sometime
    if ship_handler_data
        .lock()
        .await
        .ships
        .get(&ship_id)
        .unwrap()
        .nav
        .waypoint_symbol
        .waypoint
        != waypoint.waypoint
    {
        // there is also a case where the ship is in transit and neither docked or there
        if ship_handler_data
            .lock()
            .await
            .ships
            .get(&ship_id)
            .unwrap()
            .nav
            .status
            == enums::ShipNavStatus::Docked
        {
            ship_handler_data
                .lock()
                .await
                .ships
                .get_mut(&ship_id)
                .unwrap()
                .nav = ship_handler_data
                .lock()
                .await
                .spacetraders
                .orbit_ship(&ship_id)
                .await
                .data
                .nav;
        }
        //TODO: consider fuel types here - eg stealth, drift
        let temp_ship_data = ship_handler_data
            .lock()
            .await
            .spacetraders
            .navigate_ship(
                &ship_id,
                requests::NavigateShip {
                    waypoint_symbol: waypoint.waypoint.clone(),
                },
            )
            .await
            .data;

        (
            ship_handler_data
                .lock()
                .await
                .ships
                .get_mut(&ship_id)
                .unwrap()
                .nav,
            ship_handler_data
                .lock()
                .await
                .ships
                .get_mut(&ship_id)
                .unwrap()
                .fuel,
        ) = (temp_ship_data.nav, temp_ship_data.fuel);

        wait_duration(ship_id, ship_handler_data).await;

        // TODO: create chart here if not in mutex
    }
}

#[allow(dead_code)]
pub async fn travel_system(
    ship_id: String,
    ship_handler_data: Arc<Mutex<ShipHandlerData>>,
    waypoint: System,
) {
    trace!("travel");

    // TODO: refuel before traveling
    if ship_handler_data
        .lock()
        .await
        .ships
        .get(&ship_id)
        .unwrap()
        .nav
        .waypoint_symbol
        .system
        != waypoint.to_system().system
    {
        // there is also a case where the ship is in transit and neither docked or there
        let ship_status = ship_handler_data
            .lock()
            .await
            .ships
            .get(&ship_id)
            .unwrap()
            .nav
            .status;
        if ship_status == enums::ShipNavStatus::Docked {
            ship_handler_data
                .lock()
                .await
                .ships
                .get_mut(&ship_id)
                .unwrap()
                .nav = ship_handler_data
                .lock()
                .await
                .spacetraders
                .orbit_ship(&ship_id)
                .await
                .data
                .nav;
        }

        // depending on whether there is a warp drive or jump drive determines the endpoint to use
        // also ensure to check if there is a jump gate

        // let time_to_stop = ship_handler_data
        //     .lock()
        //     .await
        //     .spacetraders
        //     .navigate_ship(
        //         &ship_details.data.symbol,
        //         requests::NavigateShip {
        //             waypoint_symbol: waypoint.waypoint.clone(),
        //         },
        //     )
        //     .await;

        // wait_duration(time_to_stop.data.nav.route.arrival).await;
    }
}

pub async fn mine_astroid(ship_id: &String, ship_handler_data: Arc<Mutex<ShipHandlerData>>) {
    trace!("Mining Astroid");

    let waypoints = ship_handler_data
        .lock()
        .await
        .spacetraders
        .list_waypoints(
            ship_handler_data
                .lock()
                .await
                .ships
                .get_mut(ship_id)
                .unwrap()
                .nav
                .system_symbol
                .clone(),
        )
        .await;

    for waypoint in waypoints.data.iter() {
        if waypoint.r#type == enums::WaypointType::AsteroidField {
            travel_waypoint(
                ship_id.clone(),
                ship_handler_data.clone(),
                waypoint.symbol.clone(),
            )
            .await;

            if ship_handler_data
                .lock()
                .await
                .ships
                .get(ship_id)
                .unwrap()
                .nav
                .status
                == enums::ShipNavStatus::InOrbit
            {
                ship_handler_data
                    .lock()
                    .await
                    .ships
                    .get_mut(ship_id)
                    .unwrap()
                    .nav = ship_handler_data
                    .lock()
                    .await
                    .spacetraders
                    .orbit_ship(&ship_id)
                    .await
                    .data
                    .nav;
            }

            info!("Starting mining astroid");

            'inner: for mount in ship_handler_data
                .lock()
                .await
                .ships
                .get(ship_id)
                .unwrap()
                .mounts
                .iter()
            {
                if mount.symbol == enums::ShipMount::MountSurveyorI
                    || mount.symbol == enums::ShipMount::MountSurveyorIi
                    || mount.symbol == enums::ShipMount::MountSurveyorIii
                {
                    ship_handler_data.lock().await.surveys.push(
                        ship_handler_data
                            .lock()
                            .await
                            .spacetraders
                            .create_survey(&ship_id)
                            .await
                            .data,
                    );
                    break 'inner;
                }
            }
            let temp_ship_data = ship_handler_data
                .lock()
                .await
                .spacetraders
                .extract_resources(&ship_id, None)
                .await
                .data;

            ship_handler_data
                .lock()
                .await
                .ships
                .get_mut(ship_id)
                .unwrap()
                .cargo = temp_ship_data.cargo;
            let (_cooldown, _extraction) = (temp_ship_data.cooldown, temp_ship_data.extraction);

            for waypoint in waypoints.data.iter() {
                for r#trait in waypoint.traits.iter() {
                    if r#trait.symbol == enums::WaypointTrait::Marketplace {
                        travel_waypoint(
                            ship_id.clone(),
                            ship_handler_data.clone(),
                            waypoint.symbol.clone(),
                        )
                        .await;

                        if ship_handler_data
                            .lock()
                            .await
                            .ships
                            .get(ship_id)
                            .unwrap()
                            .nav
                            .status
                            == enums::ShipNavStatus::InOrbit
                        {
                            ship_handler_data
                                .lock()
                                .await
                                .ships
                                .get_mut(ship_id)
                                .unwrap()
                                .nav = ship_handler_data
                                .lock()
                                .await
                                .spacetraders
                                .dock_ship(&&ship_id)
                                .await
                                .data
                                .nav;
                        }

                        // TODO: make sure not to sell goods used for contracts
                        // TODO: also make sure I can sell that good here
                        for item in ship_handler_data
                            .lock()
                            .await
                            .ships
                            .get(ship_id)
                            .unwrap()
                            .cargo
                            .inventory
                            .clone()
                            .iter()
                        {
                            info!("{} is selling {} {:?}", ship_id, item.units, item.symbol);

                            let mut ship_handler_data_unlocked = ship_handler_data.lock().await;
                            let temp_ship_data = ship_handler_data_unlocked
                                .spacetraders
                                .sell_cargo(
                                    &ship_id,
                                    requests::SellCargo {
                                        symbol: item.symbol.clone(),
                                        units: item.units,
                                    },
                                )
                                .await
                                .data;

                            ship_handler_data
                                .lock()
                                .await
                                .ships
                                .get_mut(ship_id)
                                .unwrap()
                                .cargo = temp_ship_data.cargo;
                            let (_agent, transaction) =
                                (temp_ship_data.agent, temp_ship_data.transaction);

                            ship_handler_data_unlocked.credits =
                                ship_handler_data_unlocked.credits + transaction.units;

                            // drop(ship_handler_data_unlocked);
                        }
                        return;
                    }
                }
            }
        }
    }

    // else maybe fly to the closest system with a shipyard - TODO: Pathfinding
    warn!("Failed to find asteroid");
}

pub async fn buy_ship(
    ship_id: &String,
    ship_handler_data: Arc<Mutex<ShipHandlerData>>,
    ship_types: &[enums::ShipType],
    channel: mpsc::Sender<responses::schemas::Ship>,
) {
    trace!("Buy mining ship");

    let waypoints = ship_handler_data
        .lock()
        .await
        .spacetraders
        .list_waypoints(
            ship_handler_data
                .lock()
                .await
                .ships
                .get(ship_id)
                .unwrap()
                .nav
                .system_symbol
                .clone(),
        )
        .await;

    'outer: for waypoint in waypoints.data.iter() {
        for r#trait in waypoint.traits.iter() {
            if r#trait.symbol == enums::WaypointTrait::Shipyard {
                travel_waypoint(
                    ship_id.clone(),
                    ship_handler_data.clone(),
                    waypoint.symbol.clone(),
                )
                .await;

                let shipyard = ship_handler_data
                    .lock()
                    .await
                    .spacetraders
                    .get_shipyard(waypoint.system_symbol.clone(), waypoint.symbol.clone())
                    .await;

                for shipyard_ship in shipyard.data.ships.iter() {
                    for ship_type in ship_types {
                        if shipyard_ship.r#type == ship_type.to_owned() {
                            let mut ship_handler_data_unlocked = ship_handler_data.lock().await;
                            if shipyard_ship.purchase_price < ship_handler_data_unlocked.credits {
                                let new_ship = ship_handler_data_unlocked
                                    .spacetraders
                                    .purchase_ship(requests::PurchaseShip {
                                        ship_type: shipyard_ship.r#type,
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

pub async fn explore(_ship_handler_data: Arc<Mutex<ShipHandlerData>>) {}

// pub async fn get_contracts(
//     ship_handler_data: Arc<Mutex<ShipHandlerData>>,
// ) -> Vec<contracts::schemas::Contract> {
//     let available_contracts = ship_handler_data
//         .lock()
//         .await
//         .spacetraders
//         .list_contracts()
//         .await
//         .data;

//     let mut accepted_contracts: Vec<contracts::schemas::Contract> = vec![];
//     for contract in available_contracts.iter() {
//         if contract.accepted {
//             accepted_contracts.push(contract.clone())
//         }
//     }
//     if accepted_contracts.is_empty() {
//         // TODO // should except the contract that is expiring first!
//         // or that gives the most rep points
//         // for now just excepting the first contract for simplicity
//         accepted_contracts.push(
//             ship_handler_data
//                 .lock()
//                 .await
//                 .spacetraders
//                 .accept_contract(&available_contracts[0].id)
//                 .await
//                 .data
//                 .contract,
//         )
//     } else if accepted_contracts.is_empty() && available_contracts.is_empty() {
//         warn!("accepted contracts is zero, but there are no contracts to accept");
//     }
//     accepted_contracts
// }

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
