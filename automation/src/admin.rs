use spacetraders::{
    //contracts
    // SpaceTraders,
    enums,
    requests,
    responses::{self, schemas},
    System,
    Waypoint,
};

use super::func::{get_waypoint, get_waypoints, travel_system, travel_waypoint};
use super::ShipHandlerData;

use log::{info, trace, warn};
use std::sync::Arc;
use tokio::sync::{
    mpsc::{self, channel},
    Mutex,
};

pub async fn admin_stuff(
    ship_id: &str,
    ship_handler_data: Arc<Mutex<ShipHandlerData>>,
    ship_types: &[enums::ShipType],
    channel: mpsc::Sender<responses::schemas::Ship>,
) {
    trace!("Look at contracts");

    let mut contracts = ship_handler_data
        .lock()
        .await
        .spacetraders
        .list_contracts(None)
        .await;

    if contracts.meta.total > 1 {
        for num in 2..contracts.meta.total {
            let paged_contracts = ship_handler_data
                .lock()
                .await
                .spacetraders
                .list_contracts(Some(num))
                .await
                .data;
            for paged_contract in paged_contracts.iter() {
                contracts.data.push(paged_contract.clone())
            }
        }
    }

    let mut needed_ship = vec![];

    for contract in contracts.data.iter() {
        if contract.accepted == false {
            // TODO: check the expiration here
            // TODO: definatly make sure not to accept too many or clan ranking goes down
            match contract.r#type {
                enums::ListContractsType::Procurement => {
                    needed_ship.push(enums::ShipType::ShipMiningDrone)
                }
                enums::ListContractsType::Transport => todo!(),
                enums::ListContractsType::Shuttle => todo!(),
            }

            let mut contractor_ship = false;

            // TODO: this clone is note affective
            'inner: for (_, ship) in ship_handler_data.lock().await.ships.clone().into_iter() {
                if ship.registration.role == enums::ShipRole::Hauler {
                    contractor_ship = true;
                    break 'inner;
                }
            }
            if contractor_ship {
            } else {
                buy_ship(
                    ship_id,
                    ship_handler_data.clone(),
                    &needed_ship,
                    channel.clone(),
                )
                .await
            }
        }
    }
}

pub async fn buy_ship(
    ship_id: &str,
    ship_handler_data: Arc<Mutex<ShipHandlerData>>,
    ship_types: &[enums::ShipType],
    channel: mpsc::Sender<responses::schemas::Ship>,
) {
    trace!("Buy mining ship");

    let system = ship_handler_data
        .lock()
        .await
        .ships
        .get(ship_id)
        .unwrap()
        .nav
        .system_symbol
        .clone();

    let waypoints = get_waypoints(system, ship_handler_data.clone()).await;

    'outer: for waypoint in waypoints.iter() {
        for r#trait in waypoint.traits.iter() {
            if r#trait.symbol == enums::WaypointTrait::Shipyard {
                travel_waypoint(
                    ship_id,
                    ship_handler_data.clone(),
                    waypoint.symbol.waypoint.as_str(),
                )
                .await;

                let mut ship_handler_data_u = ship_handler_data.lock().await;

                let shipyard = ship_handler_data_u
                    .spacetraders
                    .get_shipyard(waypoint.system_symbol.clone(), waypoint.symbol.clone())
                    .await;

                for shipyard_ship in shipyard.data.ships.iter() {
                    for ship_type in ship_types {
                        if shipyard_ship.r#type == *ship_type {
                            if shipyard_ship.purchase_price < ship_handler_data_u.credits {
                                let new_ship = ship_handler_data_u
                                    .spacetraders
                                    .purchase_ship(requests::PurchaseShip {
                                        ship_type: shipyard_ship.r#type,
                                        waypoint_symbol: waypoint.symbol.clone().waypoint,
                                    })
                                    .await;

                                channel.send(new_ship.data.ship.clone()).await.unwrap();

                                ship_handler_data_u.credits -= new_ship.data.transaction.price;

                                info!(
                                    "buying ship, now at {} credits",
                                    ship_handler_data_u.credits
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
