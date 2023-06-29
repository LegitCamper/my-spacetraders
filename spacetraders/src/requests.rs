use serde::Serialize;

use super::enums::{DepositSize, FactionSymbols, FlightMode, ShipType, TradeSymbol};

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
    TransferCargo(TransferCargo),
    InstallMount(InstallMount),
    RemoveMount(RemoveMount),
    DeliverCargoToContract(DeliverCargoToContract),
}

#[derive(Serialize, Debug)]
pub struct RegisterNewAgent {
    pub faction: FactionSymbols,
    pub symbol: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct PurchaseShip {
    #[serde(rename = "shipType")]
    pub ship_type: ShipType,
    #[serde(rename = "waypointSymbol")]
    pub waypoint_symbol: String,
}

#[derive(Serialize, Debug)]
pub struct ShipRefine {
    pub produce: TradeSymbol,
}

#[derive(Serialize, Debug)]
pub struct ExtractResources {
    pub survey: ExtractResourcesSurvey,
}
#[derive(Serialize, Debug)]
pub struct ExtractResourcesSurvey {
    pub signature: String,
    pub symbol: String,
    pub deposits: Vec<ExtractResourcesSurveyDeposits>,
    pub expiration: String, // datetime
    pub size: DepositSize,
}
#[derive(Serialize, Debug)]
pub struct ExtractResourcesSurveyDeposits {
    pub symbol: String,
}

#[derive(Serialize, Debug)]
pub struct JettisonCargo {
    pub symbol: TradeSymbol,
    pub units: u32,
}

#[derive(Serialize, Debug)]
pub struct JumpShip {
    #[serde(rename = "systemSymbol")]
    pub system_symbol: String,
}

#[derive(Serialize, Debug)]
pub struct NavigateShip {
    #[serde(rename = "waypointSymbol")]
    pub ship_symbol: ShipType,
}

#[derive(Serialize, Debug)]
pub struct PatchShipNav {
    #[serde(rename = "flightMode")]
    pub ship_symbol: FlightMode,
}

#[derive(Serialize, Debug)]
pub struct WarpShip {
    #[serde(rename = "waypointSymbol")]
    pub ship_symbol: String,
}

#[derive(Serialize, Debug)]
pub struct SellCargo {
    pub symbol: TradeSymbol,
    pub units: u32,
}

#[derive(Serialize, Debug)]
pub struct PurchaseCargo {
    pub symbol: TradeSymbol,
    pub units: u32,
}

#[derive(Serialize, Debug)]
pub struct TransferCargo {
    #[serde(rename = "tradeSymbol")]
    pub trade_symbol: TradeSymbol,
    pub units: u32,
    #[serde(rename = "shipSymbol")]
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
    #[serde(rename = "shipSymbol")]
    pub ship_symbol: ShipType,
    #[serde(rename = "tradeSymbol")]
    pub trade_symbol: TradeSymbol,
    pub units: i64,
}
