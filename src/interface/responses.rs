use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::enums::*;

#[derive(Deserialize, Debug)]
pub struct Meta {
    // metadata for responses
    total: u32,
    page: u32,
    limit: u32,
}

#[derive(Deserialize, Debug)]
pub struct AgentL0 {
    pub data: AgentL1,
}
#[derive(Deserialize, Debug)]
pub struct AgentL1 {
    #[serde(alias = "accountId")]
    pub account_id: String,
    pub symbol: String,
    pub headquarters: String,
    pub credits: u64,
}

#[derive(Deserialize, Debug)]
pub enum ContractTermType {
    #[serde(alias = "PROCUREMENT")]
    Procurement,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ContractTermsL0 {
    // pub data: ContractTermsL1,
    pub data: Vec<ContractTermsL1>,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ContractTermsL1 {
    id: String,
    #[serde(alias = "factionSymbol")]
    faction_symbol: String,
    r#type: ContractTermType,
    terms: ContractTermsL2,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ContractTermsL2 {
    deadline: String, // maybe parse this to timestamp
    payment: ContractTermsL3,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ContractTermsL3 {
    #[serde(alias = "onAccepted")]
    on_accepted: u64,
    #[serde(alias = "onFulfilled")]
    on_fulfilled: u64,
    #[serde(default)]
    deliver: Vec<ContractTermsL4>,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ContractTermsL4 {
    #[serde(alias = "tradeSymbol")]
    trade_symbol: TradeSymbol,
    #[serde(alias = "destinationSymbol")]
    destination_symbol: String,
    #[serde(alias = "unitesRequired")]
    units_required: u64,
    #[serde(alias = "unitsFulfilled")]
    units_fulfilled: u64,
    accepted: bool,
    fulfilled: bool,
    experation: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ListSystemsL0 {
    data: Vec<ListSystemsL1>,
    meta: Meta,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ListSystemsL1 {
    symbol: String,
    #[serde(alias = "sectorSymbol")]
    sector_symbol: String,
    r#type: SystemType,
    x: u32,
    y: u32,
    waypoints: Vec<ListSystemsWaypoints>,
    factions: Vec<String>,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ListSystemsWaypoints {
    symbol: String,
    r#type: WaypointType,
    x: u32,
    y: u32,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct GetSystemL0 {
    data: ListSystemsL1,
    meta: Meta,
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
    pub x: u32,
    pub y: u32,
    pub orbitals: Vec<String>,
    pub traits: Vec<ListWaypointsTraits>,
    pub chart: HashMap<String, String>,
    pub faction: HashMap<String, String>,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ListWaypointsTraits {
    pub symbol: WaypointType,
    pub name: String,
    // #[serde(default)] // removing description
    // pub desciption: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct GetWaypointL0 {
    data: ListSystemsL1,
    meta: Meta,
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
    symbol: TradeGood,
    name: String,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct GetMarketTransactions {
    #[serde(alias = "waypointSymbol")]
    waypoint_symbol: String,
    #[serde(alias = "shipSymbol")]
    ship_symbol: ShipType,
    #[serde(alias = "tradeSymbol")]
    trade_symbol: TradeGood,
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
    ship_types: GetShipyardTypes,
    transactions: GetShipyardTransactions,
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
    modules: GetShipyardModules,
    mounts: Vec<GetShipyardMounts>,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct GetShipyardFrame {
    symbol: ShipType,
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
    symbol: ShipReactorType,
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
    deposits: TradeGood, // this is not right //TODO
    requirements: HashMap<String, u32>,
}
