pub use crate::responses::schemas::Survey as ExtractResources;

use serde::Serialize;

use super::enums::{FactionSymbols, FlightMode, ShipType, TradeSymbol};

#[derive(Debug, Serialize)]
pub enum Requests {
    RegisterNewAgent(RegisterNewAgent),
    PurchaseShip(PurchaseShip),
    ShipRefine(ShipRefine),
    ExtractResources(ExtractResources),
    JettisonCargo(JettisonCargo),
    JumpShip(JumpShip),
    NavigateShip(NavigateShip),
    PatchShipNav(PatchShipNav),
    WarpShip(WarpShip),
    SellCargo(SellCargo),
    PurchaseCargo(PurchaseCargo),
    RefuelShip(RefuelShip),
    TransferCargo(TransferCargo),
    InstallMount(InstallMount),
    RemoveMount(RemoveMount),
    DeliverCargoToContract(DeliverCargoToContract),
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct RegisterNewAgent {
    pub faction: FactionSymbols,
    pub symbol: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct PurchaseShip {
    pub ship_type: ShipType,
    pub waypoint_symbol: String,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ShipRefine {
    pub produce: TradeSymbol,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct JettisonCargo {
    pub symbol: TradeSymbol,
    pub units: i32,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct JumpShip {
    pub system_symbol: String,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct NavigateShip {
    pub waypoint_symbol: String,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct PatchShipNav {
    pub ship_symbol: FlightMode,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct WarpShip {
    pub ship_symbol: String,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct SellCargo {
    pub symbol: TradeSymbol,
    pub units: i32,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct PurchaseCargo {
    pub symbol: TradeSymbol,
    pub units: i32,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct RefuelShip {
    pub units: i32,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct TransferCargo {
    pub trade_symbol: TradeSymbol,
    pub units: i32,
    pub ship_symbol: String,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct InstallMount {
    pub symbol: String,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct RemoveMount {
    pub symbol: String,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct DeliverCargoToContract {
    pub ship_symbol: ShipType,
    pub trade_symbol: TradeSymbol,
    pub units: i64,
}
