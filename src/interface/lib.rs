use serde::{Deserialize, Serialize};

// Other structs for reponses from spacetrades

#[derive(Deserialize, Debug)]
pub struct AgentL0 {
    pub data: AgentInfoL1,
}
#[derive(Deserialize, Debug)]
pub struct AgenL1 {
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
    pub data: ContractTermsL1,
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
    trade_symbol: Item,
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
pub struct WaypointsListedL0 {
    pub data: Vec<WaypointsListedL1>,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct WaypointsListedL1 {
    #[serde(alias = "systemSymbol")]
    pub system_symbol: String,
    pub symbol: String,
    pub r#type: WaypointTrait,
    pub x: i64,
    pub y: i64,
    pub orbitals: Vec<WaypointsListedOrbitals>,
    pub traits: Vec<WaypointsListedTraits>,
    pub chart: WaypointsListedCharts,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct WaypointsListedOrbitals {
    symbol: String,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct WaypointsListedTraits {
    pub symbol: WaypointTrait,
    pub name: String,
    #[serde(default)]
    pub desciption: String,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct WaypointsListedCharts {
    #[serde(alias = "submittedBy")]
    submitted_by: Faction,
    #[serde(alias = "submittedOn")]
    submitted_on: String,
    // desciption: String,
    // faction: Vec<faction>,
}

// Other structs for requests from spacetrades

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

#[derive(Deserialize, Debug)]
pub enum Item {
    // not sure this is a good idea
    #[serde(alias = "ALUMINUM_ORE")]
    AluminumOre,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum FlightMode {
    #[serde(alias = "CRUISE")]
    Cruise,
    #[serde(alias = "BURN")]
    Burn,
    #[serde(alias = "DRIFT")]
    Drift,
    #[serde(alias = "STEALTH")]
    Stealth,
}

#[derive(Deserialize, PartialEq, Eq, Debug)]
pub enum WaypointTrait {
    #[serde(alias = "SHIPYARD")]
    Shipyard,
    #[serde(alias = "PLANET")]
    Planet,
    #[serde(alias = "MOON")]
    Moon,
    #[serde(alias = "ASTEROID_FIELD")]
    AsteroidField,
    #[serde(alias = "GAS_GIANT")]
    GasGiant,
    #[serde(alias = "ORBITAL_STATION")]
    OrbitalStation,
    #[serde(alias = "OVERCROWDED")]
    Overcrowded,
    #[serde(alias = "BUREAUCRATIC")]
    Buereaucratic,
    #[serde(alias = "MARKETPLACE")]
    Marketplace,
    #[serde(alias = "HIGH_TECH")]
    HighTech,
    #[serde(alias = "TEMPERATE")]
    Termerate,
    #[serde(alias = "BARREN")]
    Barren,
    #[serde(alias = "TRADING_HUB")]
    TradingHub,
    #[serde(alias = "VOLCANIC")]
    Volcanic,
    #[serde(alias = "FROZEN")]
    Frozen,
    #[serde(alias = "TOXIC_ATMOSPHERE")]
    ToxicAtmoshere,
    #[serde(alias = "WEAK_GRAVITY")]
    WeakGravity,
    #[serde(alias = "MINERAL_DEPOSITS")]
    MineralDeposits,
    #[serde(alias = "COMMON_METAL_DEPOSITS")]
    CommonMetalDeposits,
    #[serde(alias = "PRECIOUS_METAL_DEPOSITS")]
    PrecuousMetalDeposits,
    #[serde(alias = "STRIPPED")]
    Striped,
    #[serde(alias = "VIBRANT_AURORAS")]
    VibrantAuroras,
    #[serde(alias = "STRONG_MAGNETOSPHERE")]
    StrongMagnetosphere,
    #[serde(alias = "MILITARY_BASE")]
    MilitaryBase,
    #[serde(alias = "DRY_SEABEDS")]
    DrySeabeds,
    #[serde(alias = "JUMP_GATE")]
    JumpGate,
}

#[derive(Deserialize, Debug)]
pub enum Faction {
    #[serde(alias = "COSMIC")]
    Cosmic,
}

#[derive(Deserialize, Debug)]
pub enum Ships {
    #[serde(alias = "SHIP_PROBE")]
    ShipProbe,
    #[serde(alias = "SHIP_MINING_DRONE")]
    ShipMiningDrone,
    #[serde(alias = "SHIP_ORE_HOUND")]
    ShipOreHound,
    #[serde(alias = "SHIP_REFINING_FREIGHTER")]
    ShipRefiningFreighter,
}
