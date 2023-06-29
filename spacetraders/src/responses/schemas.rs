use crate::{
    enums::{self, TradeSymbol},
    Sector as SectorType, System as SystemType, Waypoint as WaypointType,
};

// use bson::serde_helpers::chrono_datetime_as_bson_datetime;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer};

// TODO: deserialize faction symbol to FactionSymbol enum
// TODO: desetialize wapoints into custom struct

fn skip_trade_symbol<'de, D>(de: D) -> Result<Option<enums::TradeSymbol>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(TradeSymbol::deserialize(de).ok())
}

#[derive(Deserialize, Debug)]
pub struct Agent {
    #[serde(alias = "accountId")]
    pub account_id: String,
    pub symbol: String,
    pub headquarters: WaypointType, //String,
    pub credits: f64,
    #[serde(alias = "startingFaction")]
    pub starting_faction: String, // enums::FactionSymbols,
}

#[derive(Deserialize, Default, Debug)]
pub struct Chart {
    #[serde(alias = "waypointSymbol")]
    #[serde(default)]
    pub waypoint_symbol: String,
    #[serde(alias = "submittedBy")]
    #[serde(default)]
    pub submitted_by: String,
    #[serde(alias = "submittedOn")]
    #[serde(default)]
    // #[serde(with = "chrono_datetime_as_bson_datetime")]
    pub submitted_on: DateTime<Utc>,
}

#[derive(Deserialize, Debug)]
pub struct ConnectedSystem {
    pub symbol: String,
    #[serde(alias = "sectorSymbol")]
    pub sector_symbol: String,
    pub r#typ: enums::SystemType,
    #[serde(alias = "factionSymbol")]
    pub faction_symbol: String, // enums::FactionSymbols,
    pub x: i32,
    pub y: i32,
    pub distance: f64,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Contract {
    pub id: String,
    #[serde(alias = "factionSymbol")]
    pub faction_symbol: String, // enums::FactionSymbols,
    pub r#type: enums::ListContractsType,
    pub terms: ContractTerms,
    pub accepted: bool,
    pub fulfilled: bool,
    #[serde(default)]
    pub expiration: String,
    #[serde(default)]
    #[serde(alias = "deadlineToAccept")]
    // #[serde(with = "chrono_datetime_as_bson_datetime")]
    pub deadline_to_accept: DateTime<Utc>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ContractDeliverGood {
    #[serde(alias = "tradeSymbol")]
    #[serde(deserialize_with = "skip_trade_symbol")]
    pub trade_symbol: Option<enums::TradeSymbol>,
    #[serde(alias = "destinationSymbol")]
    pub destination_symbol: String,
    #[serde(alias = "unitsRequired")]
    pub units_required: i64,
    #[serde(alias = "unitsFulfilled")]
    pub units_fulfilled: i64,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ContractPayment {
    #[serde(alias = "onAccepted")]
    pub on_accepted: i64,
    #[serde(alias = "onFulfilled")]
    pub on_fulfilled: i64,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ContractTerms {
    // #[serde(with = "chrono_datetime_as_bson_datetime")]
    pub deadline: DateTime<Utc>,
    pub payment: ContractPayment,
    #[serde(default)]
    pub deliver: Vec<ContractDeliverGood>,
}

#[derive(Deserialize, Debug)]
pub struct Cooldown {
    #[serde(alias = "shipSymbol")]
    pub ship_symbol: String,
    #[serde(alias = "totalSeconds")]
    pub total_seconds: u32,
    #[serde(alias = "remainingSeconds")]
    pub remaining_seconds: u32,
    #[serde(default)]
    // #[serde(with = "chrono_datetime_as_bson_datetime")]
    pub expiration: DateTime<Utc>,
}

#[derive(Deserialize, Debug)]
pub struct Extraction {
    #[serde(alias = "shipSymbol")]
    pub ship_symbol: String,
    pub r#yield: ExtractionYield,
}

#[derive(Deserialize, Debug)]
pub struct ExtractionYield {
    pub symbol: enums::TradeSymbol,
    pub units: i32,
}

#[derive(Deserialize, Debug)]
pub struct Faction {
    pub symbol: String, // enums::FactionSymbols,
    pub name: String,
    // description: String,
    pub headquarters: String,
    pub traits: Vec<FactionTrait>,
    #[serde(alias = "isRecruiting")]
    pub is_recruiting: bool,
}

#[derive(Deserialize, Debug)]
pub struct FactionTrait {
    pub symbol: enums::FactionTrait,
    pub name: String,
    // description: String,
}

#[derive(Deserialize, Default, Debug)]
pub struct JumpGate {
    #[serde(alias = "jumpRange")]
    pub jump_range: f64,
    #[serde(alias = "factionSymbol")]
    #[serde(default)]
    pub faction_symbol: String, // enums::FactionSymbols, // this fails in tests, but is okay //TODO: see if fixed - issue on discord
    #[serde(alias = "connectedSystems")]
    pub connected_systems: Vec<JumpGateConnectedSystems>,
}
#[derive(Deserialize, Debug)]
pub struct JumpGateConnectedSystems {
    pub symbol: String,
    #[serde(alias = "sectorSymbol")]
    pub sector_symbol: String,
    #[serde(default)]
    pub r#type: enums::SystemType,
    #[serde(default)]
    #[serde(alias = "factionSymbol")]
    pub faction_symbol: String, // enums::FactionSymbols,
    pub x: i32,
    pub y: i32,
    pub distance: i32,
}

#[derive(Deserialize, Debug)]
pub struct Market {
    pub symbol: String,
    pub exports: Vec<MarketDetails>,
    pub imports: Vec<MarketDetails>,
    pub exchange: Vec<MarketDetails>,
    pub transactions: Vec<MarketTransaction>,
    #[serde(alias = "tradeGoods")]
    pub trade_goods: Vec<GetMarketTradeGood>,
}
#[derive(Deserialize, Debug)]
pub struct MarketDetails {
    pub symbol: enums::TradeSymbol,
    pub name: String,
    // pub description: String,
}

#[derive(Deserialize, Debug)]
pub struct GetMarketTradeGood {
    pub symbol: String,
    #[serde(alias = "tradeVolume")]
    pub trade_volume: u32,
    pub supply: enums::GetMarketSupplyType,
    #[serde(alias = "purchasePrice")]
    pub purchase_price: i32,
    #[serde(alias = "sellPrice")]
    pub sell_price: u32,
}

#[derive(Deserialize, Debug)]
pub struct MarketTransaction {
    #[serde(alias = "waypointSymbol")]
    pub waypoint_symbol: String,
    #[serde(alias = "shipSymbol")]
    pub ship_symbol: String,
    #[serde(alias = "tradeSymbol")]
    pub trade_symbol: String,
    pub r#type: enums::GetMarketType,
    pub units: i32,
    #[serde(alias = "pricePerUnit")]
    pub price_per_unit: i32,
    #[serde(alias = "totalPrice")]
    pub total_price: i32,
    // #[serde(with = "chrono_datetime_as_bson_datetime")]
    pub timestamp: DateTime<Utc>,
}

#[derive(Deserialize, Debug)]
pub struct Meta {
    pub total: u32,
    pub page: u32,
    pub limit: u32,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Ship {
    pub symbol: String,
    pub registration: ShipRegistration,
    pub nav: ShipNav,
    pub crew: ShipCrew,
    pub frame: ShipFrame,
    pub reactor: ShipReactor,
    pub engine: ShipEngine,
    pub modules: Vec<ShipModule>,
    pub mounts: Vec<ShipMount>,
    pub cargo: ShipCargo,
    pub fuel: ShipFuel,
}

#[derive(Deserialize, Debug)]
pub struct ScannedSystem {
    pub symbol: String,
    #[serde(alias = "sectorSymbol")]
    pub sector_symbol: String,
    pub r#type: enums::SystemType,
    pub x: i32,
    pub y: i32,
    pub distance: i32,
}

#[derive(Deserialize, Debug)]
pub struct ScannedWaypoint {
    #[serde(alias = "systemSymbol")]
    pub system_symbol: String,
    pub symbol: String,
    pub r#type: enums::WaypointType,
    pub x: i32,
    pub y: i32,
    pub orbitals: Vec<ScannedWaypointOrbitals>,
    pub traits: Vec<ScannedWaypointTrait>,
    #[serde(default)]
    pub chart: Chart,
    pub faction: SystemFaction,
}
#[derive(Deserialize, Debug)]
pub struct ScannedWaypointOrbitals {
    pub symbol: String,
}
#[derive(Deserialize, Debug)]
pub struct ScannedWaypointFaction {
    pub symbol: String, // enums::FactionSymbols,
}
#[derive(Deserialize, Debug)]
pub struct ScannedWaypointTrait {
    pub symbol: enums::WaypointTrait,
    pub name: String,
    // pub description: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ScannedShip {
    pub symbol: String,
    pub registration: ShipRegistration,
    pub nav: ShipNav,
    pub frame: ScannedShipFrame,
    pub reactor: ScannedShipReactor,
    pub engine: ScannedShipEngine,
    pub mounts: Vec<ScannedShipMounts>,
}
#[derive(Deserialize, Clone, Debug)]
pub struct ScannedShipReactor {
    pub symbol: String,
}
#[derive(Deserialize, Clone, Debug)]
pub struct ScannedShipFrame {
    pub symbol: String,
}
#[derive(Deserialize, Clone, Debug)]
pub struct ScannedShipEngine {
    pub symbol: String,
}
#[derive(Deserialize, Clone, Debug)]
pub struct ScannedShipMounts {
    pub symbol: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ShipCargo {
    pub capacity: u32,
    pub units: i32,
    pub inventory: Vec<ShipCargoItem>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ShipCargoItem {
    pub symbol: String,
    pub name: String,
    // description: String,
    pub units: i32,
}

#[derive(Deserialize, Debug)]
pub struct ShipCargoCondition {
    pub condition: u32,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ShipCrew {
    pub current: i32,
    pub required: i32,
    pub capacity: i32,
    pub rotation: enums::ShipCrewRotation,
    pub morale: u32,
    pub wages: u32,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ShipEngine {
    pub symbol: enums::ShipEngine,
    pub name: String,
    // description: String,
    #[serde(default)]
    pub condition: u32,
    pub speed: u32,
    pub requirements: ShipRequirements,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ShipFrame {
    pub symbol: enums::ShipFrame,
    pub name: String,
    // description: String,
    #[serde(default)]
    pub condition: u32,
    #[serde(alias = "moduleSlots")]
    pub module_slots: u32,
    #[serde(alias = "mountingPoints")]
    pub mounting_points: u32,
    #[serde(alias = "fuelCapacity")]
    pub fuel_capacity: u32,
    pub requirements: ShipRequirements,
}

#[derive(Deserialize, Default, Clone, Debug)]
pub struct ShipFuel {
    pub current: u32,
    pub capacity: u32,
    #[serde(default)]
    pub consumed: ShipFuelConsumed,
}
#[derive(Deserialize, Default, Clone, Debug)]
pub struct ShipFuelConsumed {
    pub amount: u32,
    // #[serde(with = "chrono_datetime_as_bson_datetime")]
    pub timestamp: DateTime<Utc>,
}

#[derive(Deserialize, Debug)]
pub struct ShipModificationTransaction {
    #[serde(alias = "waypointSymbol")]
    pub waypoint_symbol: String,
    #[serde(alias = "shipSymbol")]
    pub ship_symbol: String,
    #[serde(alias = "totalPrice")]
    pub total_price: i32,
    #[serde(alias = "tradeSymbol")]
    pub trade_symbol: String,
    pub timestamp: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ShipModule {
    pub symbol: enums::ShipModule,
    // description: String,
    #[serde(default)]
    pub capacity: u32,
    #[serde(default)]
    pub range: u32,
    pub name: String,
    pub requirements: ShipRequirements,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ShipMount {
    pub symbol: enums::ShipMount,
    pub name: String,
    // description: String,
    #[serde(default)]
    pub strength: u32,
    #[serde(default)]
    pub deposits: Vec<enums::TradeSymbol>,
    pub requirements: ShipRequirements,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ShipNav {
    #[serde(alias = "systemSymbol")]
    pub system_symbol: String,
    #[serde(alias = "waypointSymbol")]
    pub waypoint_symbol: String,
    pub route: ShipNavRoute,
    pub status: enums::ShipNavStatus,
    #[serde(alias = "flightMode")]
    pub flight_mode: enums::FlightMode,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ShipNavRoute {
    pub destination: ShipNavRouteWaypoint,
    pub departure: ShipNavRouteWaypoint,
    #[serde(alias = "departureTime")]
    // #[serde(with = "chrono_datetime_as_bson_datetime")]
    pub departure_time: DateTime<Utc>,
    // #[serde(with = "chrono_datetime_as_bson_datetime")]
    pub arrival: DateTime<Utc>,
}
#[derive(Deserialize, Clone, Debug)]
pub struct ShipNavRouteWaypoint {
    pub symbol: String,
    pub r#type: enums::WaypointType,
    #[serde(alias = "systemSymbol")]
    pub system_symbol: String,
    pub x: i32,
    pub y: i32,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ShipReactor {
    pub symbol: enums::ShipReactor,
    pub name: String,
    // description: String,
    #[serde(default)]
    pub condition: u32,
    #[serde(alias = "powerOutput")]
    pub power_output: u32,
    pub requirements: ShipRequirements,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ShipRegistration {
    pub name: String,
    #[serde(alias = "factionSymbol")]
    pub faction_symbol: String, // enums::FactionSymbols,
    pub role: enums::ShipRole,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ShipRequirements {
    #[serde(default)]
    pub power: i32,
    #[serde(default)]
    pub crew: i32,
    #[serde(default)]
    pub slots: i32,
}

#[derive(Deserialize, Debug)]
pub struct Shipyard {
    pub symbol: String,
    #[serde(alias = "shipTypes")]
    pub ship_types: Vec<ShipyardTypes>,
    #[serde(default)]
    pub transactions: Vec<ShipyardTransaction>,
    #[serde(default)]
    pub ships: Vec<ShipyardShip>,
}
#[derive(Deserialize, Debug)]
pub struct ShipyardTypes {
    pub r#type: enums::ShipType,
}

#[derive(Deserialize, Debug)]
pub struct ShipyardShip {
    pub r#type: enums::ShipType,
    pub name: String,
    // description: String,
    #[serde(alias = "purchasePrice")]
    pub purchase_price: i32,
    pub frame: ShipFrame,
    pub reactor: ShipReactor,
    pub engine: ShipEngine,
    pub modules: Vec<ShipModule>,
    pub mounts: Vec<ShipMount>,
}

#[derive(Deserialize, Debug)]
pub struct ShipyardTransaction {
    #[serde(alias = "waypointSymbol")]
    pub waypoint_symbol: String,
    #[serde(alias = "shipSymbol")]
    pub ship_symbol: String,
    pub price: i32,
    pub timestamp: String,
}

#[derive(Deserialize, Debug)]
pub struct Survey {
    pub signature: String,
    pub symbol: String,
    pub deposits: Vec<SurveyDeposit>,
    // #[serde(with = "chrono_datetime_as_bson_datetime")]
    pub expiration: DateTime<Utc>,
    pub size: enums::DepositSize,
}

#[derive(Deserialize, Debug)]
pub struct SurveyDeposit {
    pub symbol: String, // maybe change to enum TradeSymbol
}

#[derive(Deserialize, Debug)]
pub struct System {
    pub symbol: String,
    #[serde(alias = "sectorSymbol")]
    pub sector_symbol: String,
    pub r#type: enums::SystemType,
    pub x: i32,
    pub y: i32,
    pub waypoints: Vec<SystemWaypoint>,
    pub factions: Vec<SystemFaction>,
}

#[derive(Deserialize, Debug)]
pub struct SystemFaction {
    pub symbol: String, // enums::FactionSymbols,
}

#[derive(Deserialize, Debug)]
pub struct SystemWaypoint {
    pub symbol: String,
    pub r#type: enums::WaypointType,
    pub x: i32,
    pub y: i32,
}

#[derive(Deserialize, Debug)]
pub struct TradeGoods {
    pub symbol: enums::TradeSymbol,
    pub name: String,
    // descripton
}

#[derive(Deserialize, Debug)]
pub struct Waypoint {
    #[serde(alias = "systemSymbol")]
    pub system_symbol: String,
    pub symbol: String,
    pub r#type: enums::WaypointType,
    pub x: i32,
    pub y: i32,
    pub orbitals: Vec<ScannedWaypointOrbitals>,
    pub traits: Vec<ScannedWaypointTrait>,
    #[serde(default)]
    pub chart: Chart,
    pub faction: SystemFaction,
}

#[derive(Deserialize, Debug)]
pub struct WaypointOrbital {
    pub symbol: String,
}
