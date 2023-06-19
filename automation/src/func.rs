use crate::{contracts, schemas};
use crate::SpaceTradersHandler;

pub async fn accept_contract(interface: &SpaceTradersHandler) -> contracts::schemas::Contract {
    let contracts = interface.list_contracts().await.data;

    for contract in contracts.iter() {
        if contract.accepted {
            return contract.clone()
        }
    }
    // TODO // should except the contract that is expiring first!
    // or that gives the most rep points
    // for now just excepting the first contract for simplicity
    interface.accept_contract(&contracts[0].id).await;
        contracts[0].clone()
}


