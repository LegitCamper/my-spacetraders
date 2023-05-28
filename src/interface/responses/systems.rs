use crate::interface::enums::*;
use crate::interface::responses::Meta;

use serde::Deserialize;
use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ListSystemsL0 {
    // data: Vec<ListSystemsL1>,
    meta: Meta,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ListSystemsL1 {
    symbol: String,
    #[serde(alias = "sectorSymbol")]
    sector_symbol: String,
    r#type: SystemType,
    x: i32,
    y: i32,
    waypoints: Vec<ListSystemsWaypoints>,
    factions: HashMap<String, String>,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ListSystemsWaypoints {
    symbol: String,
    r#type: WaypointType,
    x: i32,
    y: i32,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct GetSystemL0 {
    pub data: GetSystemsL1,
    // meta: Meta,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct GetSystemsL1 {
    pub symbol: String,
    #[serde(alias = "sectorSymbol")]
    pub sector_symbol: String,
    pub r#type: SystemType,
    pub x: i32,
    pub y: i32,
    pub waypoints: Vec<GetSystemsWaypoints>,
    pub factions: Vec<HashMap<String, String>>,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct GetSystemsWaypoints {
    pub symbol: String,
    pub r#type: WaypointType,
    pub x: i32,
    pub y: i32,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ListWaypointsL0 {
    pub data: Vec<ListWaypointsL1>,
    pub meta: Meta,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ListWaypointsL1 {
    #[serde(alias = "systemSymbol")]
    pub system_symbol: String,
    pub symbol: String,
    pub r#type: WaypointType,
    pub x: i32,
    pub y: i32,
    pub orbitals: Vec<HashMap<String, String>>,
    pub traits: Vec<ListWaypointsTraits>,
    pub chart: HashMap<String, String>,
    pub faction: HashMap<String, String>,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ListWaypointsTraits {
    pub symbol: WaypointTrait,
    pub name: String,
    // #[serde(default)] // removing description
    // pub desciption: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct GetWaypointL0 {
    pub data: GetWaypointL1,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct GetWaypointL1 {
    pub symbol: String,
    #[serde(alias = "systemSymbol")]
    pub system_symbol: String,
    pub r#type: WaypointType,
    pub x: i32,
    pub y: i32,
    pub orbitals: Vec<HashMap<String, String>>,
    #[serde(default)]
    pub faction: HashMap<String, String>,
    pub traits: Vec<ListWaypointsTraits>,
    #[serde(default)]
    pub chart: HashMap<String, String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GetMarketType {
    Purchase,
    Sell,
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GetMarketSupplyType {
    Scarce,
    Limited,
    Moderate,
    Abundant,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct GetMarketL0 {
    pub data: GetMarketL1,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct GetMarketL1 {
    pub symbol: String,
    pub exports: Vec<GetMarketDetails>,
    pub imports: Vec<GetMarketDetails>,
    pub exchange: Vec<GetMarketDetails>,
    pub transactions: Vec<GetMarketTransactions>,
    #[serde(alias = "tradeGoods")]
    pub trade_goods: Vec<GetMarketTradeGoods>,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct GetMarketDetails {
    pub symbol: TradeSymbol,
    pub name: String,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct GetMarketTransactions {
    #[serde(alias = "waypointSymbol")]
    pub waypoint_symbol: String,
    #[serde(alias = "shipSymbol")]
    pub ship_symbol: String,
    #[serde(alias = "tradeSymbol")]
    pub trade_symbol: String, //TradeSymbol,
    pub r#type: GetMarketType,
    pub units: u32,
    #[serde(alias = "pricePerUnit")]
    pub price_per_unit: u32,
    #[serde(alias = "totalPrice")]
    pub total_price: u32,
    pub timestamp: String,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct GetMarketTradeGoods {
    pub symbol: String,
    #[serde(alias = "tradeVolume")]
    pub trade_volume: u32,
    pub supply: GetMarketSupplyType,
    #[serde(alias = "purchasePrice")]
    pub purchase_price: u32,
    #[serde(alias = "sellPrice")]
    pub sell_price: u32,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct GetShipyardL0 {
    pub data: GetShipyardL1,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct GetShipyardL1 {
    pub symbol: String,
    #[serde(alias = "shipTypes")]
    pub ship_types: Vec<GetShipyardTypes>,
    #[serde(default)]
    pub transactions: Vec<GetShipyardTransactions>,
    #[serde(default)]
    pub ships: Vec<GetShipyardShips>,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct GetShipyardTypes {
    pub r#type: ShipType,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct GetShipyardTransactions {
    #[serde(alias = "waypointSymbol")]
    pub waypoint_symbol: String,
    #[serde(alias = "shipSymbol")]
    pub ship_symbol: String,
    #[serde(alias = "agentSymbol")]
    pub agent_symbol: String,
    pub timestamp: String,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct GetShipyardShips {
    pub r#type: ShipType,
    pub name: String,
    // description: String,
    #[serde(alias = "purchasePrice")]
    pub purchase_price: u32,
    pub frame: GetShipyardFrame,
    pub reactor: GetShipyardReactor,
    pub engine: GetShipyardEngine,
    pub modules: Vec<GetShipyardModules>,
    pub mounts: Vec<GetShipyardMounts>,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct GetShipyardFrame {
    pub symbol: ShipFrame,
    pub name: String,
    // description: String,
    pub condition: u32,
    #[serde(alias = "moduleSlots")]
    pub module_slots: u32,
    #[serde(alias = "mountingPoints")]
    pub mounting_points: u32,
    #[serde(alias = "fuelCapacity")]
    pub fuel_capacity: u32,
    pub requirements: HashMap<String, u32>,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct GetShipyardReactor {
    pub symbol: ShipReactor,
    pub name: String,
    // description: String,
    pub condition: u32,
    #[serde(alias = "powerOutput")]
    pub power_output: u32,
    pub requirements: HashMap<String, u32>,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct GetShipyardEngine {
    pub symbol: ShipEngine,
    pub name: String,
    // description: String,
    pub condition: u32,
    pub speed: u32,
    pub requirements: HashMap<String, u32>,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct GetShipyardModules {
    pub symbol: ShipModule,
    pub name: String,
    // description: String,
    pub capacity: u32,
    pub range: u32,
    pub requirements: HashMap<String, u32>,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct GetShipyardMounts {
    pub symbol: ShipMount,
    pub name: String,
    // description: String,
    pub strength: u32,
    pub deposits: Vec<TradeSymbol>,
    pub requirements: HashMap<String, u32>,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct GetJumpGateL0 {
    pub data: GetJumpGateL1,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct GetJumpGateL1 {
    #[serde(alias = "jumpRange")]
    pub jump_range: i64,
    #[serde(alias = "factionSymbol")]
    pub faction_symbol: String,
    #[serde(alias = "connectedSystems")]
    pub connected_systems: Vec<GetJumpGateConnectedSystems>,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct GetJumpGateConnectedSystems {
    pub symbol: String,
    #[serde(alias = "sectorSymbol")]
    pub sector_symbol: String,
    pub r#type: SystemType,
    #[serde(alias = "factionSymbol")]
    pub faction_symbol: String,
    pub x: u32,
    pub y: u32,
    pub distance: u32,
}
