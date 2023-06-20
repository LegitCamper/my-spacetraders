use spacetraders::SpaceTraders;
mod func;

pub async fn automate(space_traders: SpaceTraders) {
    // loop {

    let current_contract = func::get_contract(&space_traders);

    // for current_contract in accepted.iter() {
    // if !have_correct_ship {
    // func::buy_ship(&space_traders);
    // }
    // }
}
