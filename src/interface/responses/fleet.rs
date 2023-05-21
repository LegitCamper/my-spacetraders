use crate::interface::enums::*;
use crate::interface::responses::Meta;
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ListShipsL0 {
    data: Vec<ListShipsL1>,
    meta: Meta,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ListShipsL1 {
    pub symbol: String,
    pub registration: ListShipsRegistration,
    pub nav: ListShipsNav,
    pub crew: ListShipsCrew,
    pub frame: ListShipsFrame,
    pub reactor: ListShipsReactor,
    pub engine: ListShipsEngine,
    pub modules: Vec<ListShipsModules>,
    pub mounts: Vec<ListShipsMounts>,
    pub cargo: ListShipsCargo,
    pub fuel: ListShipsFuel,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ListShipsRegistration {
    pub name: String,
    #[serde(alias = "factionSymbol")]
    pub faction_symbol: String,
    pub role: ShipRole,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ListShipsNav {
    #[serde(alias = "systemSymbol")]
    pub system_symbol: String,
    #[serde(alias = "waypointSymbol")]
    pub waypoint_symbol: String,
    pub route: ListShipsNavRoute,
    pub status: ShipNavStatus,
    #[serde(alias = "flightMode")]
    pub flight_mode: FlightMode,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ListShipsNavRoute {
    pub destination: ListShipsNavRouteDestination,
    pub departure: ListShipsNavRouteDestination, // same as destinaton
    #[serde(alias = "departureTime")]
    pub departure_time: String, // timestamp
    pub arrival: String,                         // timestamp
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ListShipsNavRouteDestination {
    pub symbol: String,
    pub r#type: WaypointType,
    #[serde(alias = "systemSymbol")]
    pub system_symbol: String,
    pub x: i32,
    pub y: i32,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ListShipsCrew {
    pub current: i32,
    pub required: u32,
    pub capacity: u32,
    pub rotation: ShipCrewRotation,
    pub morale: u32,
    pub wages: u32,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ListShipsFrame {
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
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ListShipsFrameRequirements {
    #[serde(default)]
    power: u32,
    #[serde(default)]
    crew: u32,
    #[serde(default)]
    slots: u32,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ListShipsReactor {
    symbol: ShipReactorType,
    name: String,
    // description: String,
    condition: u32,
    #[serde(alias = "powerOutput")]
    power_output: u32,
    requirements: ListShipsFrameRequirements,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ListShipsEngine {
    symbol: ShipEngine,
    name: String,
    // description: String,
    condition: u32,
    speed: u32,
    requirements: ListShipsFrameRequirements,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ListShipsModules {
    symbol: ShipModule,
    // description: String,
    #[serde(default)]
    capacity: u32,
    #[serde(default)]
    range: u32,
    name: String,
    requirements: ListShipsFrameRequirements,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ListShipsMounts {
    symbol: ShipMount,
    name: String,
    // description: String,
    #[serde(default)]
    strength: u32,
    #[serde(default)]
    deposits: TradeSymbol,
    requirements: ListShipsFrameRequirements,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ListShipsCargo {
    capacity: u32,
    units: u32,
    inventory: Vec<ListShipsCargoInventory>,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ListShipsCargoInventory {
    symbol: String,
    name: String,
    // description
    units: u32,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ListShipsFuel {
    current: u32,
    capacity: u32,
    // #[serde(default)]
    consumed: ListShipsFuelConsumed,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ListShipsFuelConsumed {
    amount: u32,
    timestamp: String, // timestamp
}
