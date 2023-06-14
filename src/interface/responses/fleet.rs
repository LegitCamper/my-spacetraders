pub use super::schemas;

use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Ships {
    pub data: Vec<schemas::Ship>,
    pub meta: schemas::Meta,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct PurchaseShip {
    pub data: PurchaseShipL1,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct PurchaseShipL1 {
    pub agent: PurchaseShipAgent,
    pub ship: schemas::Ship,
    pub transaction: PurchaseShipTransaction,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct PurchaseShipAgent {
    #[serde(alias = "accountId")]
    pub account_id: String,
    pub symbol: String,
    pub headquarters: String,
    pub credits: u32,
    #[serde(alias = "startingFaction")]
    pub starting_faction: String,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct PurchaseShipTransaction {
    #[serde(alias = "waypointSymbol")]
    pub waypoint_symbol: String,
    #[serde(alias = "shipSymbol")]
    pub ship_symbol: String,
    pub price: u32,
    #[serde(alias = "agentSymbol")]
    pub agent_symbol: String,
    pub timestamp: String, // timestamp
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Ship {
    pub data: schemas::Ship,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ShipCargo {
    pub data: ShipCargoData,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ShipCargoData {
    pub capacity: u32,
    pub units: u32,
    pub inventory: Vec<schemas::ShipCargoItem>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct OrbitShip {
    pub data: OrbitShipData,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct OrbitShipData {
    pub nav: schemas::ShipNav,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ShipRefine {
    pub data: ShipRefineData,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ShipRefineData {
    pub cargo: ShipCargo,
    pub cooldown: schemas::Cooldown,
    pub produced: Vec<ShipRefineIO>,
    pub consumed: Vec<ShipRefineIO>,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ShipRefineIO {
    #[serde(alias = "tradeSymbol")]
    pub trade_symbol: String,
    pub units: u32,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct CreateChart {
    pub data: CreateChartData,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct CreateChartData {
    pub chart: schemas::Chart,
    pub waypoint: schemas::Waypoint,
}
