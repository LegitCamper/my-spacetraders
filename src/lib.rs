pub mod interface;

use interface::{
    enums::{ListContractsType, ShipRole, WaypointTrait},
    parse_waypoint,
    requests::BuyShip,
    responses::{contracts::ListContractsL1, fleet::PurchaseShipAgent},
    SpaceTradersHandler,
};

use crate::interface::responses::{contracts, systems};

// use std::sync::Arc;
// use tokio::sync::broadcast;

async fn create_interface() -> SpaceTradersHandler {
    // can manually enter token with SpaceTradersHandler::new otherwise generates random token
    SpaceTradersHandler::default().await
}

pub async fn main_algo() {
    let interface = create_interface().await;

    // loop {
    // TODO // REMOVE THIS
    complete_contracts(&interface).await;
    // }
}

async fn complete_contracts(interface: &SpaceTradersHandler) {
    // 1) check for accepted contracts
    let contracts = interface.list_contracts().await.data;

    let mut accepted: Vec<&ListContractsL1> = vec![];
    for contract in contracts.iter() {
        if contract.accepted {
            accepted.push(contract);
        }
    }

    // 1.5) if 0 accept new contract
    if accepted.is_empty() && !contracts.is_empty() {
        // TODO // should except the contract that is expiring first!
        // for now just excepting the first contract for simplicity
        interface.accept_contract(&contracts[0].id).await;
        accepted.push(&contracts[0])
    } else if accepted.is_empty() && contracts.is_empty() {
        panic!("accepted contracts is zero, but there are now contracts to accept")
    }

    for current_contract in accepted.iter() {
        let ships = interface.list_ships().await;
        let mut have_correct_ship = false;
        // 2) check if ship can complete contracts
        match current_contract.r#type {
            ListContractsType::Procurement => {
                // Need mining ship // probably
                for ship in ships.data.iter() {
                    if ship.registration.role == ShipRole::Excavator
                        || ship.registration.role == ShipRole::Harvester
                    {
                        have_correct_ship = true;
                    }
                }
            }
            ListContractsType::Transport => {
                // probably need figigate or hauler ship
                for ship in ships.data.iter() {
                    if ship.registration.role == ShipRole::Hauler
                        || ship.registration.role == ShipRole::Carrier
                    {
                        have_correct_ship = true;
                    }
                }
            }
            ListContractsType::Shuttle => {
                // probaly need shuttle
                for ship in ships.data.iter() {
                    if ship.registration.role == ShipRole::Carrier
                        || ship.registration.role == ShipRole::Transport
                    {
                        have_correct_ship = true;
                    }
                }
            }
        }

        // 2.5) if not but ship
        if !have_correct_ship {
            let system = parse_waypoint(&interface.agent().await.data.headquarters).system;
            println!("\n{}\n", system);
            for waypoint in interface.list_waypoints(&system).await.data.iter() {
                let mut found_shipyard = false;

                for waypoint_trait in waypoint.traits.iter() {
                    if waypoint_trait.symbol == WaypointTrait::Shipyard {
                        found_shipyard = true;
                        let shipyard_waypoint = parse_waypoint(&waypoint.symbol).sector;
                        let ships_in_shipyard = interface
                            .get_shipyard(&waypoint.symbol, &shipyard_waypoint)
                            .await;
                        for ship in ships_in_shipyard.data.ship_types.iter() {
                            println!("{:?}", ship);
                            // match current_contract.r#type {
                            // should buy the correct ship for the contract. for now I will just buy mining drone
                            // ListContractsType::Procurement => {
                            //     if ship.
                            // }
                            // ListContractsType::Transport => {}
                            // ListContractsType::Shuttle => {}
                            // }
                            interface
                                .purchase_ship(BuyShip {
                                    // shipType: ship.symbol,
                                    shipType: interface::enums::ShipType::ShipMiningDrone
                                        .to_string(),
                                    waypointSymbol: waypoint.symbol.clone(),
                                })
                                .await
                        }
                        break;
                    }
                    if !found_shipyard {
                        println!("Failed to find Shipyard");
                        interface.diagnose();
                    }
                }
            }
        }
    }
}
