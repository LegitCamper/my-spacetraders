use spacetraders::{
    //contracts
    // SpaceTraders,
    enums,
    requests,
    responses,
};

use super::func::ShipAutomation;

use log::{error, info, trace, warn};
use tokio::sync::mpsc;

pub async fn admin_stuff(
    ship_data: &ShipAutomation,
    _ship_types: &[enums::ShipType],
    channel: mpsc::Sender<responses::schemas::Ship>,
) {
    trace!("Look at contracts");

    let contracts = match ship_data
        .st_interface
        .read()
        .await
        .list_contracts(false)
        .await
    {
        Ok(contracts) => contracts,
        Err(_) => {
            error!("{} Failed to get Contracts", ship_data.ship_id);
            return;
        }
    };

    let mut needed_ship = vec![];

    for contract in contracts.data.iter() {
        if !contract.accepted {
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

            'inner: for (_, ship) in ship_data.clone_ships().await.into_iter() {
                if ship.registration.role == enums::ShipRole::Hauler {
                    contractor_ship = true;
                    break 'inner;
                }
            }
            if contractor_ship {
            } else {
                buy_ship(ship_data, &needed_ship, channel.clone()).await
            }
        }
    }
}

pub async fn buy_ship(
    ship_data: &ShipAutomation,
    ship_types: &[enums::ShipType],
    channel: mpsc::Sender<responses::schemas::Ship>,
) {
    trace!("Buy mining ship");

    let ship = ship_data.clone_ship().await.unwrap();

    let waypoints = ship_data.get_waypoints(&ship.nav.system_symbol).await;

    'outer: for waypoint in waypoints.iter() {
        for r#trait in waypoint.traits.iter() {
            if r#trait.symbol == enums::WaypointTrait::Shipyard {
                ship_data
                    .travel_waypoint(waypoint.symbol.waypoint.as_str())
                    .await;

                let shipyard = ship_data
                    .st_interface
                    .read()
                    .await
                    .get_shipyard(&waypoint.system_symbol, &waypoint.symbol)
                    .await;

                for shipyard_ship in shipyard.unwrap().data.ships.iter() {
                    for ship_type in ship_types {
                        if shipyard_ship.r#type == *ship_type {
                            if shipyard_ship.purchase_price < ship_data.get_credits().await {
                                // locking both as write to prevent access while mutating data
                                let (unlocked_interface, mut unlocked_ship) = (
                                    ship_data.st_interface.write().await,
                                    ship_data.ship_automation.write().await,
                                );

                                let new_ship = unlocked_interface
                                    .purchase_ship(requests::PurchaseShip {
                                        ship_type: shipyard_ship.r#type.clone(),
                                        waypoint_symbol: waypoint.symbol.clone().waypoint,
                                    })
                                    .await
                                    .unwrap();

                                channel.send(new_ship.data.ship.clone()).await.unwrap();

                                unlocked_ship.credits -= new_ship.data.transaction.price;

                                info!("buying ship, now at {} credits", unlocked_ship.credits);
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
