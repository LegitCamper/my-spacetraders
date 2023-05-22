use serde::{Serialize};

use super::enums::*;

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
pub struct BuyShip {
    pub shipType: ShipType,
    pub waypointSymbol: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
pub struct ShipRefine {
    pub produce: TradeSymbol,
}

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
pub struct ChangeFlightMode {
    pub flightMode: FlightMode,
}
