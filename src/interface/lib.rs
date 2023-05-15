use std::string;

use serde::{Deserialize, Serialize};

// Enums

#[derive(Deserialize, PartialEq, Eq, Debug)]
pub enum WaypointTrait {
    UNCHARTED,
    MARKETPLACE,
    SHIPYARD,
    OUTPOST,
    SCATTERED_SETTLEMENTS,
    SPRAWLING_CITIES,
    MEGA_STRUCTURES,
    OVERCROWDED,
    HIGH_TECH,
    CORRUPT,
    BUREAUCRATIC,
    TRADING_HUB,
    INDUSTRIAL,
    BLACK_MARKET,
    RESEARCH_FACILITY,
    MILITARY_BASE,
    SURVEILLANCE_OUTPOST,
    EXPLORATION_OUTPOST,
    MINERAL_DEPOSITS,
    COMMON_METAL_DEPOSITS,
    PRECIOUS_METAL_DEPOSITS,
    RARE_METAL_DEPOSITS,
    METHANE_POOLS,
    ICE_CRYSTALS,
    EXPLOSIVE_GASES,
    STRONG_MAGNETOSPHERE,
    VIBRANT_AURORAS,
    SALT_FLATS,
    CANYONS,
    PERPETUAL_DAYLIGHT,
    PERPETUAL_OVERCAST,
    DRY_SEABEDS,
    MAGMA_SEAS,
    SUPERVOLCANOES,
    ASH_CLOUDS,
    VAST_RUINS,
    MUTATED_FLORA,
    TERRAFORMED,
    EXTREME_TEMPERATURES,
    EXTREME_PRESSURE,
    DIVERSE_LIFE,
    SCARCE_LIFE,
    FOSSILS,
    WEAK_GRAVITY,
    STRONG_GRAVITY,
    CRUSHING_GRAVITY,
    TOXIC_ATMOSPHERE,
    CORROSIVE_ATMOSPHERE,
    BREATHABLE_ATMOSPHERE,
    JOVIAN,
    ROCKY,
    VOLCANIC,
    FROZEN,
    SWAMP,
    BARREN,
    TEMPERATE,
    JUNGLE,
    OCEAN,
    STRIPPED,
}

// impl ToString for WaypointTrait {
//     fn to_string(&self) -> String {
//         match self {
//             Self::Shipyard => String::from("SHIPYARD"),
//         }
//     }
// }

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

// Structs for reponses from spacetrades

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

// Structs for requests from spacetrades

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
