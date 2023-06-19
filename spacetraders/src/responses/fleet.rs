use self::schemas::{
    Agent, Contract, Cooldown, Extraction, MarketTransaction, ScannedShip, ScannedSystem,
    ScannedWaypoint, ShipFuel, ShipMount, ShipNav, ShipyardTransaction, Survey,
};

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
    pub cargo: schemas::ShipCargo,
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

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct GetShipCooldown {
    pub data: Cooldown,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct DockShip {
    pub data: ShipNav,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct CreateSurvey {
    pub data: CreateSurveyData,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct CreateSurveyData {
    pub cooldown: Cooldown,
    pub surveys: Vec<Survey>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ExtractResources {
    pub data: ExtractResourcesData,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ExtractResourcesData {
    pub cooldown: Cooldown,
    pub extraction: Extraction,
    pub cargo: ShipCargo,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct JettisonCargo {
    pub data: JettisonCargoData,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct JettisonCargoData {
    pub cargo: ShipCargo,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct JumpShip {
    pub data: JumpShipData,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct JumpShipData {
    pub cooldown: Cooldown,
    pub nav: ShipNav,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct NavigateShip {
    pub data: NavigateShipData,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct NavigateShipData {
    pub fuel: ShipFuel,
    pub nav: ShipNav,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct PatchShipNav {
    pub data: ShipNav,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct GetShipNav {
    pub data: ShipNav,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct WarpShip {
    pub data: WarpShipData,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct WarpShipData {
    pub fuel: ShipFuel,
    pub nav: ShipNav,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct SellCargo {
    pub data: SellCargoData,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct SellCargoData {
    pub agent: Agent,
    pub cargo: ShipCargo,
    pub transaction: MarketTransaction,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ScanSystems {
    pub data: ScanSystemsData,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ScanSystemsData {
    pub cooldown: Cooldown,
    pub systems: Vec<ScannedSystem>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ScanWaypoints {
    pub data: ScanWaypointsData,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ScanWaypointsData {
    pub cooldown: Cooldown,
    pub systems: Vec<ScannedWaypoint>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ScanShips {
    pub data: ScanShipsData,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ScanShipsData {
    pub cooldown: Cooldown,
    pub systems: Vec<ScannedShip>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct RefuelShip {
    pub data: RefuelShipData,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct RefuelShipData {
    pub agent: Agent,
    pub fuel: ShipFuel,
    pub transaction: MarketTransaction,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct PurchaseCargo {
    pub data: PurchaseCargoData,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct PurchaseCargoData {
    pub agent: Agent,
    pub cargo: ShipCargo,
    pub transaction: MarketTransaction,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct TransferCargo {
    pub data: TransferCargoData,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct TransferCargoData {
    pub cargo: ShipCargo,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct NegotiateContract {
    pub data: NegotiateContractData,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct NegotiateContractData {
    pub contract: Contract,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct GetMounts {
    pub data: Vec<ShipMount>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct InstallMounts {
    pub data: InstallMountsData,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct InstallMountsData {
    pub agent: Agent,
    pub mounts: Vec<ShipMount>,
    pub cargo: ShipCargo,
    pub transaction: ShipyardTransaction,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct RemoveMounts {
    pub data: RemoveMountsData,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct RemoveMountsData {
    pub agent: Agent,
    pub mounts: Vec<ShipMount>,
    pub cargo: ShipCargo,
    pub transaction: ShipyardTransaction,
}
