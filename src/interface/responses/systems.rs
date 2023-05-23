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
    data: GetSystemsL1,
    // meta: Meta,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct GetSystemsL1 {
    symbol: String,
    #[serde(alias = "sectorSymbol")]
    sector_symbol: String,
    r#type: SystemType,
    x: i32,
    y: i32,
    waypoints: Vec<GetSystemsWaypoints>,
    factions: Vec<HashMap<String, String>>,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct GetSystemsWaypoints {
    symbol: String,
    r#type: WaypointType,
    x: i32,
    y: i32,
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
    data: GetWaypointL1,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct GetWaypointL1 {
    symbol: String,
    #[serde(alias = "systemSymbol")]
    system_symbol: String,
    r#type: WaypointType,
    x: i32,
    y: i32,
    orbitals: Vec<HashMap<String, String>>,
    #[serde(default)]
    faction: HashMap<String, String>,
    traits: Vec<ListWaypointsTraits>,
    #[serde(default)]
    chart: HashMap<String, String>,
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
    data: GetMarketL1,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct GetMarketL1 {
    symbol: String,
    exports: Vec<GetMarketDetails>,
    imports: Vec<GetMarketDetails>,
    exchange: Vec<GetMarketDetails>,
    transactions: Vec<GetMarketTransactions>,
    #[serde(alias = "tradeGoods")]
    trade_goods: Vec<GetMarketTradeGoods>,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct GetMarketDetails {
    symbol: TradeSymbol,
    name: String,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct GetMarketTransactions {
    #[serde(alias = "waypointSymbol")]
    waypoint_symbol: String,
    #[serde(alias = "shipSymbol")]
    ship_symbol: String,
    #[serde(alias = "tradeSymbol")]
    trade_symbol: String, //TradeSymbol,
    r#type: GetMarketType,
    units: u32,
    #[serde(alias = "pricePerUnit")]
    price_per_unit: u32,
    #[serde(alias = "totalPrice")]
    total_price: u32,
    timestamp: String,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct GetMarketTradeGoods {
    symbol: String,
    #[serde(alias = "tradeVolume")]
    trade_volume: u32,
    supply: GetMarketSupplyType,
    #[serde(alias = "purchasePrice")]
    purchase_price: u32,
    #[serde(alias = "sellPrice")]
    sell_price: u32,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct GetShipyardL0 {
    data: GetShipyardL1,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct GetShipyardL1 {
    symbol: String,
    #[serde(alias = "shipTypes")]
    ship_types: Vec<GetShipyardTypes>,
    transactions: Vec<GetShipyardTransactions>,
    ships: Vec<GetShipyardShips>,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct GetShipyardTypes {
    r#type: ShipType,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct GetShipyardTransactions {
    #[serde(alias = "waypointSymbol")]
    waypoint_symbol: String,
    #[serde(alias = "shipSymbol")]
    ship_symbol: String,
    #[serde(alias = "agentSymbol")]
    agent_symbol: String,
    timestamp: String,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct GetShipyardShips {
    r#type: ShipType,
    name: String,
    // description: String,
    #[serde(alias = "purchasePrice")]
    purchase_price: u32,
    frame: GetShipyardFrame,
    reactor: GetShipyardReactor,
    engine: GetShipyardEngine,
    modules: Vec<GetShipyardModules>,
    mounts: Vec<GetShipyardMounts>,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct GetShipyardFrame {
    symbol: ShipFrame,
    name: String,
    // description: String,
    condition: u32,
    #[serde(alias = "moduleSlots")]
    module_slots: u32,
    #[serde(alias = "mountingPoints")]
    mounting_points: u32,
    #[serde(alias = "fuelCapacity")]
    fuel_capacity: u32,
    requirements: HashMap<String, u32>,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct GetShipyardReactor {
    symbol: ShipReactor,
    name: String,
    // description: String,
    condition: u32,
    #[serde(alias = "powerOutput")]
    power_output: u32,
    requirements: HashMap<String, u32>,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct GetShipyardEngine {
    symbol: ShipEngine,
    name: String,
    // description: String,
    condition: u32,
    speed: u32,
    requirements: HashMap<String, u32>,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct GetShipyardModules {
    symbol: ShipModule,
    name: String,
    // description: String,
    capacity: u32,
    range: u32,
    requirements: HashMap<String, u32>,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct GetShipyardMounts {
    symbol: ShipMount,
    name: String,
    // description: String,
    strength: u32,
    deposits: Vec<TradeSymbol>,
    requirements: HashMap<String, u32>,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct GetJumpGateL0 {
    data: GetJumpGateL1,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct GetJumpGateL1 {
    #[serde(alias = "jumpRange")]
    jump_range: i64,
    #[serde(alias = "factionSymbol")]
    faction_symbol: String,
    #[serde(alias = "connectedSystems")]
    connected_systems: Vec<GetJumpGateConnectedSystems>,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct GetJumpGateConnectedSystems {
    symbol: String,
    #[serde(alias = "sectorSymbol")]
    sector_symbol: String,
    r#type: SystemType,
    #[serde(alias = "factionSymbol")]
    faction_symbol: String,
    x: u32,
    y: u32,
    distance: u32,
}
