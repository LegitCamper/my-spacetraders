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
    pub data: PurchaseShipData,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct PurchaseShipData {
    pub agent: schemas::Agent,
    pub ship: schemas::Ship,
    pub transaction: schemas::ShipyardTransaction,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Ship {
    pub data: schemas::Ship,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ShipCargo {
    pub data: schemas::ShipCargo,
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
    pub units: i32,
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
    pub data: schemas::Cooldown,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct DockShip {
    pub data: DockShipNav,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct DockShipNav {
    pub nav: schemas::ShipNav,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct CreateSurvey {
    pub data: CreateSurveyData,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct CreateSurveyData {
    pub cooldown: schemas::Cooldown,
    pub surveys: Vec<schemas::Survey>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ExtractResources {
    pub data: ExtractResourcesData,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ExtractResourcesData {
    pub cooldown: schemas::Cooldown,
    pub extraction: schemas::Extraction,
    pub cargo: schemas::ShipCargo,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct JettisonCargo {
    pub data: JettisonCargoData,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct JettisonCargoData {
    pub cargo: schemas::ShipCargo,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct JumpShip {
    pub data: JumpShipData,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct JumpShipData {
    pub cooldown: schemas::Cooldown,
    pub nav: schemas::ShipNav,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct NavigateShip {
    pub data: NavigateShipData,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct NavigateShipData {
    pub fuel: schemas::ShipFuel,
    pub nav: schemas::ShipNav,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct PatchShipNav {
    pub data: schemas::ShipNav,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct GetShipNav {
    pub data: schemas::ShipNav,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct WarpShip {
    pub data: WarpShipData,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct WarpShipData {
    pub fuel: schemas::ShipFuel,
    pub nav: schemas::ShipNav,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct SellCargo {
    pub data: SellCargoData,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct SellCargoData {
    pub agent: schemas::Agent,
    pub cargo: schemas::ShipCargo,
    pub transaction: schemas::MarketTransaction,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ScanSystems {
    pub data: ScanSystemsData,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ScanSystemsData {
    pub cooldown: schemas::Cooldown,
    pub systems: Vec<schemas::ScannedSystem>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ScanWaypoints {
    pub data: ScanWaypointsData,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ScanWaypointsData {
    pub cooldown: schemas::Cooldown,
    pub waypoints: Vec<schemas::ScannedWaypoint>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ScanShips {
    pub data: ScanShipsData,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ScanShipsData {
    pub cooldown: schemas::Cooldown,
    pub ships: Vec<schemas::ScannedShip>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct RefuelShip {
    pub data: RefuelShipData,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct RefuelShipData {
    pub agent: schemas::Agent,
    pub fuel: schemas::ShipFuel,
    pub transaction: schemas::MarketTransaction,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct PurchaseCargo {
    pub data: PurchaseCargoData,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct PurchaseCargoData {
    pub agent: schemas::Agent,
    pub cargo: schemas::ShipCargo,
    pub transaction: schemas::MarketTransaction,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct TransferCargo {
    pub data: TransferCargoData,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct TransferCargoData {
    pub cargo: schemas::ShipCargo,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct NegotiateContract {
    pub data: NegotiateContractData,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct NegotiateContractData {
    pub contract: schemas::Contract,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct GetMounts {
    pub data: Vec<schemas::ShipMount>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct InstallMounts {
    pub data: InstallMountsData,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct InstallMountsData {
    pub agent: schemas::Agent,
    pub mounts: Vec<schemas::ShipMount>,
    pub cargo: schemas::ShipCargo,
    pub transaction: schemas::ShipModificationTransaction,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct RemoveMounts {
    pub data: RemoveMountsData,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct RemoveMountsData {
    pub agent: schemas::Agent,
    pub mounts: Vec<schemas::ShipMount>,
    pub cargo: schemas::ShipCargo,
    pub transaction: schemas::ShipModificationTransaction,
}
