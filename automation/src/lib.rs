use serde::Serialize;
use spacetraders::SpaceTraders;
mod func;

pub async fn automate(space_traders: SpaceTraders) {
    // loop {

    let accepted_contracts = func::get_contract(&space_traders).await;

    for current_contract in accepted_contracts {
        func::get_contract_ship(&space_traders, current_contract).await;

        // for current_contract in accepted.iter() {
        // if !have_correct_ship {
        // func::buy_ship(&space_traders);
        // }
        // }
    }
}
