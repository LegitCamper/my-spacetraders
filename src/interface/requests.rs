use serde::{Deserialize, Serialize};

use super::enums::*;

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
pub struct BuyShip {
    pub shipType: String,
    pub waypointSymbol: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
pub struct ChangeFlightMode {
    pub flightMode: FlightMode,
}
