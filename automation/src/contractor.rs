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
use tokio::sync::Mutex;

// pub async fn get_contracts(
//     ship_handler_data: Arc<Mutex<ShipHandlerData>>,
// ) -> Vec<contracts::schemas::Contract> {
//     let available_contracts = ship_handler_data
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
