use std::collections::HashMap;

use serde::{Deserialize, Serialize};

// Enums

#[derive(Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WaypointTrait {
    Uncharted,
    Marketplace,
    Shipyard,
    Outpost,
    Scatteredsettlements,
    Sprawlingcities,
    Megastructures,
    Overcrowded,
    Hightech,
    Corrupt,
    Bureaucratic,
    Tradinghub,
    Industrial,
    Blackmarket,
    Researchfacility,
    Militarybase,
    Surveillanceoutpost,
    Explorationoutpost,
    Mineraldeposits,
    Commonmetaldeposits,
    Preciousmetaldeposits,
    Raremetaldeposits,
    Methanepools,
    Icecrystals,
    Explosivegases,
    Strongmagnetosphere,
    Vibrantauroras,
    Saltflats,
    Canyons,
    Perpetualdaylight,
    Perpetualovercast,
    Dryseabeds,
    Magmaseas,
    Supervolcanoes,
    Ashclouds,
    Vastruins,
    Mutatedflora,
    Terraformed,
    Extremetemperatures,
    Extremepressure,
    Diverselife,
    Scarcelife,
    Fossils,
    Weakgravity,
    Stronggravity,
    Crushinggravity,
    Toxicatmosphere,
    Corrosiveatmosphere,
    Breathableatmosphere,
    Jovian,
    Rocky,
    Volcanic,
    Frozen,
    Swamp,
    Barren,
    Temperate,
    Jungle,
    Ocean,
    Stripped,
}

#[derive(Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WaypointType {
    Planet,
    Gasgiant,
    Moon,
    Orbitalstation,
    Jumpgate,
    Asteroidfield,
    Nebula,
    Debrisfield,
    Gravitywell,
}

#[derive(Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SystemType {
    Neutronstar,
    Redstar,
    Orangestar,
    Bluestar,
    Youngstar,
    Whitedwarf,
    Blackhole,
    Hypergiant,
    Nebula,
    Unstable,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ShipType {
    Shipprobe,
    Shipminingdrone,
    Shipinterceptor,
    Shiplighthauler,
    Shipcommandfrigate,
    Shipexplorer,
    Shipheavyfreighter,
    Shiplightshuttle,
    Shiporehound,
    Shiprefiningfreighter,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ShipRole {
    Fabricator,
    Harvester,
    Hauler,
    Interceptor,
    Excavator,
    Transport,
    Repair,
    Surveyor,
    Command,
    Carrier,
    Patrol,
    Satellite,
    Explorer,
    Refinery,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ShipReactorType {
    Reactorsolari,
    Reactorfusioni,
    Reactorfissioni,
    Reactorchemicali,
    Reactorantimatteri,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ShipNavStatus {
    Intransit,
    Inorbit,
    Docked,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ShipMount {
    Mountgassiphoni,
    Mountgassiphonii,
    Mountgassiphoniii,
    Mountsurveyori,
    Mountsurveyorii,
    Mountsurveyoriii,
    Mountsensorarrayi,
    Mountsensorarrayii,
    Mountsensorarrayiii,
    Mountmininglaseri,
    Mountmininglaserii,
    Mountmininglaseriii,
    Mountlasercannoni,
    Mountmissilelauncheri,
    Mountturreti,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TradeSymbol {
    Preciousstones,
    Quartzsand,
    Siliconcrystals,
    Ammoniaice,
    Liquidhydrogen,
    Liquidnitrogen,
    Icewater,
    Exoticmatter,
    Advancedcircuitry,
    Gravitonemitters,
    Iron,
    Ironore,
    Copper,
    Copperore,
    Aluminum,
    Aluminumore,
    Silver,
    Silverore,
    Gold,
    Goldore,
    Platinum,
    Platinumore,
    Diamonds,
    Uranite,
    Uraniteore,
    Meritium,
    Meritiumore,
    Hydrocarbon,
    Antimatter,
    Fertilizers,
    Fabrics,
    Food,
    Jewelry,
    Machinery,
    Firearms,
    Assaultrifles,
    Militaryequipment,
    Explosives,
    Labinstruments,
    Ammunition,
    Electronics,
    Shipplating,
    Equipment,
    Fuel,
    Medicine,
    Drugs,
    Clothing,
    Microprocessors,
    Plastics,
    Polynucleotides,
    Biocomposites,
    Nanobots,
    Aimainframes,
    Quantumdrives,
    Roboticdrones,
    Cyberimplants,
    Genetherapeutics,
    Neuralchips,
    Moodregulators,
    Viralagents,
    Microfusiongenerators,
    Supergrains,
    Laserrifles,
    Holographics,
    Shipsalvage,
    Relictech,
    Novellifeforms,
    Botanicalspecimens,
    Culturalartifacts,
    Reactorsolari,
    Reactorfusioni,
    Reactorfissioni,
    Reactorchemicali,
    Reactorantimatteri,
    Engineimpulsedrivei,
    Engineiondrivei,
    Engineiondriveii,
    Enginehyperdrivei,
    Modulemineralprocessori,
    Modulecargoholdi,
    Modulecrewquartersi,
    Moduleenvoyquartersi,
    Modulepassengercabini,
    Modulemicrorefineryi,
    Moduleorerefineryi,
    Modulefuelrefineryi,
    Modulesciencelabi,
    Modulejumpdrivei,
    Modulejumpdriveii,
    Modulejumpdriveiii,
    Modulewarpdrivei,
    Modulewarpdriveii,
    Modulewarpdriveiii,
    Moduleshieldgeneratori,
    Moduleshieldgeneratorii,
    Mountgassiphoni,
    Mountgassiphonii,
    Mountgassiphoniii,
    Mountsurveyori,
    Mountsurveyorii,
    Mountsurveyoriii,
    Mountsensorarrayi,
    Mountsensorarrayii,
    Mountsensorarrayiii,
    Mountmininglaseri,
    Mountmininglaserii,
    Mountmininglaseriii,
    Mountlasercannoni,
    Mountmissilelauncheri,
    Mountturreti,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ShipModule {
    Modulemineralprocessori,
    Modulecargoholdi,
    Modulecrewquartersi,
    Moduleenvoyquartersi,
    Modulepassengercabini,
    Modulemicrorefineryi,
    Moduleorerefineryi,
    Modulefuelrefineryi,
    Modulesciencelabi,
    Modulejumpdrivei,
    Modulejumpdriveii,
    Modulejumpdriveiii,
    Modulewarpdrivei,
    Modulewarpdriveii,
    Modulewarpdriveiii,
    Moduleshieldgeneratori,
    Moduleshieldgeneratorii,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ShipFram {
    Frameprobe,
    Framedrone,
    Frameinterceptor,
    Frameracer,
    Framefighter,
    Framefrigate,
    Frameshuttle,
    Frameexplorer,
    Frameminer,
    Framelightfreighter,
    Frameheavyfreighter,
    Frametransport,
    Framedestroyer,
    Framecruiser,
    Framecarrier,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ShipEngine {
    Engineimpulsedrivei,
    Engineiondrivei,
    Engineiondriveii,
    Enginehyperdrivei,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TradeGood {
    Preciousstones,
    Quartzsand,
    Siliconcrystals,
    Ammoniaice,
    Liquidhydrogen,
    Liquidnitrogen,
    Icewater,
    Exoticmatter,
    Advancedcircuitry,
    Gravitonemitters,
    Iron,
    Ironore,
    Copper,
    Copperore,
    Aluminum,
    Aluminumore,
    Silver,
    Silverore,
    Gold,
    Goldore,
    Platinum,
    Platinumore,
    Diamonds,
    Uranite,
    Uraniteore,
    Meritium,
    Meritiumore,
    Hydrocarbon,
    Antimatter,
    Fertilizers,
    Fabrics,
    Food,
    Jewelry,
    Machinery,
    Firearms,
    Assaultrifles,
    Militaryequipment,
    Explosives,
    Labinstruments,
    Ammunition,
    Electronics,
    Shipplating,
    Equipment,
    Fuel,
    Medicine,
    Drugs,
    Clothing,
    Microprocessors,
    Plastics,
    Polynucleotides,
    Biocomposites,
    Nanobots,
    Aimainframes,
    Quantumdrives,
    Roboticdrones,
    Cyberimplants,
    Genetherapeutics,
    Neuralchips,
    Moodregulators,
    Viralagents,
    Microfusiongenerators,
    Supergrains,
    Laserrifles,
    Holographics,
    Shipsalvage,
    Relictech,
    Novellifeforms,
    Botanicalspecimens,
    Culturalartifacts,
    Reactorsolari,
    Reactorfusioni,
    Reactorfissioni,
    Reactorchemicali,
    Reactorantimatteri,
    Engineimpulsedrivei,
    Engineiondrivei,
    Engineiondriveii,
    Enginehyperdrivei,
    Modulemineralprocessori,
    Modulecargoholdi,
    Modulecrewquartersi,
    Moduleenvoyquartersi,
    Modulepassengercabini,
    Modulemicrorefineryi,
    Moduleorerefineryi,
    Modulefuelrefineryi,
    Modulesciencelabi,
    Modulejumpdrivei,
    Modulejumpdriveii,
    Modulejumpdriveiii,
    Modulewarpdrivei,
    Modulewarpdriveii,
    Modulewarpdriveiii,
    Moduleshieldgeneratori,
    Moduleshieldgeneratorii,
    Mountgassiphoni,
    Mountgassiphonii,
    Mountgassiphoniii,
    Mountsurveyori,
    Mountsurveyorii,
    Mountsurveyoriii,
    Mountsensorarrayi,
    Mountsensorarrayii,
    Mountsensorarrayiii,
    Mountmininglaseri,
    Mountmininglaserii,
    Mountmininglaseriii,
    Mountlasercannoni,
    Mountmissilelauncheri,
    Mountturreti,
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

// Structs for reponses from spacetrades

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
