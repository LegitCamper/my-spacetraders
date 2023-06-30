use super::ShipHandlerData;
use spacetraders::{
    enums, requests,
    responses::{self, contracts, schemas},
    SpaceTraders,
};

use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};

// this is how you serialize the waypoints
// println!("{:?}", agent.data.headquarters);
// println!("{:?}", agent.data.headquarters.waypoint);
// println!("{:?}", agent.data.headquarters.system);
// println!("{:?}", agent.data.headquarters.sector);

// mining astroid functions
pub async fn mine_astroid(_ship: schemas::Ship, _space_traders: Arc<Mutex<SpaceTraders>>) {
    println!("Mining");
}

pub async fn buy_mining_ship(
    _ship: schemas::Ship,
    space_traders: Arc<Mutex<SpaceTraders>>,
    ship_handler_data: Arc<Mutex<ShipHandlerData>>,
    channel: mpsc::Sender<responses::schemas::Ship>,
) {
    let agent = space_traders.lock().await.agent().await;

    let waypoints = space_traders
        .lock()
        .await
        .list_waypoints(agent.data.headquarters.to_system())
        .await;

    for waypoint in waypoints.data.iter() {
        for r#trait in waypoint.traits.iter() {
            if r#trait.symbol == enums::WaypointTrait::Shipyard {
                let shipyard = space_traders
                    .lock()
                    .await
                    .get_shipyard(waypoint.system_symbol.clone(), waypoint.symbol.clone()) // TODO: implement copy instead
                    .await;

                for ship in shipyard.data.ships.iter() {
                    if ship.r#type == enums::ShipType::ShipMiningDrone {
                        // fly ship to waypoint if not there already
                        // for now I will assume the ship is at the waypoint
                        // if space_traders.
                        if ship.purchase_price < agent.data.credits {
                            let new_ship = space_traders
                                .lock()
                                .await
                                .purchase_ship(requests::PurchaseShip {
                                    ship_type: ship.r#type,
                                    waypoint_symbol: waypoint.symbol.clone().waypoint,
                                })
                                .await;

                            channel.send(new_ship.data.ship.clone()).await.unwrap();
                        }
                    }
                }
            }
        }
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
//                         println!("{:?}", u);
//                         break;
//                     }
//                 }
//                 println!("yolo");
//                 break;
//             }
//         }
//     }
//     if !found_shipyard {
//         println!("Failed to find Shipyard");
//         space_traders.diagnose();
//     }
//     panic!("WORKED")
// }
