use serde::Serialize;

use super::enums::{FactionSymbols, FlightMode, ShipType, TradeSymbol};

#[derive(Serialize, Debug)]
pub struct RegisterNewAgent {
    pub faction: FactionSymbols,
    pub symbol: String,
    #[serde(default)]
    pub email: String,
}

#[derive(Serialize, Debug)]
pub struct BuyShip {
    #[serde(alias = "shipType")]
    pub ship_type: ShipType,
    #[serde(alias = "waypointSymbol")]
    pub waypoint_symbol: String,
}

#[derive(Serialize, Debug)]
pub struct ShipRefine {
    pub produce: TradeSymbol,
}

#[derive(Serialize, Debug)]
pub struct JettisonCargo {
    pub symbol: TradeSymbol,
    pub units: u32,
}

#[derive(Serialize, Debug)]
pub struct JumpShip {
    #[serde(alias = "shipSymbol")]
    pub ship_symbol: ShipType,
}

#[derive(Serialize, Debug)]
pub struct NavigateShip {
    #[serde(alias = "waypointSymbol")]
    pub ship_symbol: ShipType,
}

#[derive(Serialize, Debug)]
pub struct PatchShipNav {
    #[serde(alias = "flightMode")]
    pub ship_symbol: FlightMode,
}

#[derive(Serialize, Debug)]
pub struct WarpShip {
    #[serde(alias = "waypointSymbol")]
    pub ship_symbol: String,
}

#[derive(Serialize, Debug)]
pub struct InstallMount {
    pub symbol: String,
}

#[derive(Serialize, Debug)]
pub struct RemoveMount {
    pub symbol: String,
}

#[derive(Serialize, Debug)]
pub struct DeliverCargoToContract {
    #[serde(alias = "shipSymbol")]
    pub ship_symbol: ShipType,
    #[serde(alias = "tradeSymbol")]
    pub trade_symbol: TradeSymbol,
    pub units: String, //i64,
}
