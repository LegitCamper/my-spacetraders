pub mod interface;

use interface::{
    enums::ContractTermType, responses::contracts::ContractTermsL1, SpaceTradersHandler,
};

// use std::sync::Arc;
// use tokio::sync::broadcast;

async fn create_interface() -> SpaceTradersHandler {
    // can manually enter token with SpaceTradersHandler::new otherwise generates random token
    SpaceTradersHandler::default().await
}

pub async fn main_algo() {
    let interface = create_interface().await;

    loop {
        // TODO // REMOVE THIS
        complete_contracts(&interface).await;
    }
}

async fn complete_contracts(interface: &SpaceTradersHandler) {
    // 1) check for accepted contracts
    let contracts = interface.list_contracts().await.data;

    let mut accepted: Vec<&ContractTermsL1> = vec![];
    for contract in contracts.iter() {
        if contract.accepted {
            accepted.push(contract);
        }
    }

    // 1.5) if 0 accept new contract
    if accepted.is_empty() && !contracts.is_empty() {
        // TODO // should except the contract that is expiring first!
        // for now just excepting the first contract for simplicity
        interface
            .accept_contract(&contracts[0].id.to_string())
            .await;
        accepted.push(&contracts[0])
    } else if accepted.is_empty() && contracts.is_empty() {
        panic!("accepted contracts is zero, but there are now contracts to accept")
    }

    for current_contract in accepted.iter() {
        let ships = interface.list_ships().await;
        println!("{:?}", ships);
        // 2 check if ship can complete contracts
        match current_contract.r#type {
            ContractTermType::Procurement => {
                // Need mining ship // probably
                // list ships and check if mining
            }
            ContractTermType::Transport => {
                // probably need figigate or hauler ship
                // list ships and check if fleet
            }
            ContractTermType::Shuttle => {
                // probaly need shuttle
                // list ships and check if shuttle
            }
        }
    }
}
