use serde::{Deserialize, Serialize};
use std::fmt::{self, Debug};

// USE THE COMMAND 'ccase -t pascal enums' TO TRANSFORM THE DOCS TO RUST ENUMS

// TODO: use macros to impl the display trait instead of all the duplication

#[derive(Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ListContractsType {
    Procurement,
    Transport,
    Shuttle,
}
impl fmt::Display for ListContractsType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

#[derive(Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WaypointTrait {
    Uncharted,
    Marketplace,
    Shipyard,
    Outpost,
    ScatteredSettlements,
    SprawlingCities,
    MegaStructures,
    Overcrowded,
    HighTech,
    Corrupt,
    Bureaucratic,
    TradingHub,
    Industrial,
    BlackMarket,
    ResearchFacility,
    MilitaryBase,
    SurveillanceOutpost,
    ExplorationOutpost,
    MineralDeposits,
    CommonMetalDeposits,
    PreciousMetalDeposits,
    RareMetalDeposits,
    MethanePools,
    IceCrystals,
    ExplosiveGases,
    StrongMagnetosphere,
    VibrantAuroras,
    SaltFlats,
    Canyons,
    PerpetualDaylight,
    PerpetualOvercast,
    DrySeabeds,
    MagmaSeas,
    Supervolcanoes,
    AshClouds,
    VastRuins,
    MutatedFlora,
    Terraformed,
    ExtremeTemperatures,
    ExtremePressure,
    DiverseLife,
    ScarceLife,
    Fossils,
    WeakGravity,
    StrongGravity,
    CrushingGravity,
    ToxicAtmosphere,
    CorrosiveAtmosphere,
    BreathableAtmosphere,
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
impl fmt::Display for WaypointTrait {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

#[derive(Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WaypointType {
    Planet,
    GasGiant,
    Moon,
    OrbitalStation,
    JumpGate,
    AsteroidField,
    Nebula,
    DebrisField,
    GravityWell,
}
impl fmt::Display for WaypointType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

#[derive(Deserialize, PartialEq, Eq, Default, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SystemType {
    NeutronStar,
    RedStar,
    OrangeStar,
    BlueStar,
    YoungStar,
    WhiteDwarf,
    BlackHole,
    Hypergiant,
    Nebula,
    Unstable,
    #[default]
    Default,
}
impl fmt::Display for SystemType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Copy, Clone, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ShipType {
    ShipProbe,
    ShipMiningDrone,
    ShipInterceptor,
    ShipLightHauler,
    ShipCommandFrigate,
    ShipExplorer,
    ShipHeavyFreighter,
    ShipLightShuttle,
    ShipOreHound,
    ShipRefiningFreighter,
}
impl fmt::Display for ShipType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

#[derive(Deserialize, PartialEq, Eq, Debug)]
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
impl fmt::Display for ShipRole {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

#[derive(Deserialize, PartialEq, Eq, Copy, Clone, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ShipReactor {
    ReactorSolarI,
    ReactorFusionI,
    ReactorFissionI,
    ReactorChemicalI,
    ReactorAntimatterI,
}
impl fmt::Display for ShipReactor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

#[derive(Deserialize, PartialEq, Eq, Copy, Clone, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ShipNavStatus {
    InTransit,
    InOrbit,
    Docked,
}
impl fmt::Display for ShipNavStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

#[derive(Deserialize, PartialEq, Eq, Copy, Clone, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ShipMount {
    MountGasSiphonI,
    MountGasSiphonIi,
    MountGasSiphonIii,
    MountSurveyorI,
    MountSurveyorIi,
    MountSurveyorIii,
    MountSensorArrayI,
    MountSensorArrayIi,
    MountSensorArrayIii,
    MountMiningLaserI,
    MountMiningLaserIi,
    MountMiningLaserIii,
    MountLaserCannonI,
    MountMissileLauncherI,
    MountTurretI,
}
impl fmt::Display for ShipMount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Default, Copy, Clone, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TradeSymbol {
    PreciousStones,
    QuartzSand,
    SiliconCrystals,
    AmmoniaIce,
    LiquidHydrogen,
    LiquidNitrogen,
    IceWater,
    ExoticMatter,
    AdvancedCircuitry,
    GravitonEmitters,
    Iron,
    IronOre,
    Copper,
    CopperOre,
    Aluminum,
    AluminumOre,
    Silver,
    SilverOre,
    Gold,
    GoldOre,
    Platinum,
    PlatinumOre,
    Diamonds,
    Uranite,
    UraniteOre,
    Meritium,
    MeritiumOre,
    Hydrocarbon,
    Antimatter,
    Fertilizers,
    Fabrics,
    Food,
    Jewelry,
    Machinery,
    Firearms,
    AssaultRifles,
    MilitaryEquipment,
    Explosives,
    LabInstruments,
    Ammunition,
    Electronics,
    ShipPlating,
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
    AiMainframes,
    QuantumDrives,
    RoboticDrones,
    CyberImplants,
    GeneTherapeutics,
    NeuralChips,
    MoodRegulators,
    ViralAgents,
    MicroFusionGenerators,
    Supergrains,
    LaserRifles,
    Holographics,
    ShipSalvage,
    RelicTech,
    NovelLifeforms,
    BotanicalSpecimens,
    CulturalArtifacts,
    ReactorSolarI,
    ReactorFusionI,
    ReactorFissionI,
    ReactorChemicalI,
    ReactorAntimatterI,
    EngineImpulseDriveI,
    EngineIonDriveI,
    EngineIonDriveIi,
    EngineHyperDriveI,
    ModuleMineralProcessorI,
    ModuleCargoHoldI,
    ModuleCrewQuartersI,
    ModuleEnvoyQuartersI,
    ModulePassengerCabinI,
    ModuleMicroRefineryI,
    ModuleOreRefineryI,
    ModuleFuelRefineryI,
    ModuleScienceLabI,
    ModuleJumpDriveI,
    ModuleJumpDriveIi,
    ModuleJumpDriveIii,
    ModuleWarpDriveI,
    ModuleWarpDriveIi,
    ModuleWarpDriveIii,
    ModuleShieldGeneratorI,
    ModuleShieldGeneratorIi,
    MountGasSiphonI,
    MountGasSiphonIi,
    MountGasSiphonIii,
    MountSurveyorI,
    MountSurveyorIi,
    MountSurveyorIii,
    MountSensorArrayI,
    MountSensorArrayIi,
    MountSensorArrayIii,
    MountMiningLaserI,
    MountMiningLaserIi,
    MountMiningLaserIii,
    MountLaserCannonI,
    MountMissileLauncherI,
    MountTurretI,
    #[default]
    Default,
}
impl fmt::Display for TradeSymbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

#[derive(Deserialize, PartialEq, Eq, Copy, Clone, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ShipModule {
    ModuleMineralProcessorI,
    ModuleCargoHoldI,
    ModuleCrewQuartersI,
    ModuleEnvoyQuartersI,
    ModulePassengerCabinI,
    ModuleMicroRefineryI,
    ModuleOreRefineryI,
    ModuleFuelRefineryI,
    ModuleScienceLabI,
    ModuleJumpDriveI,
    ModuleJumpDriveIi,
    ModuleJumpDriveIii,
    ModuleWarpDriveI,
    ModuleWarpDriveIi,
    ModuleWarpDriveIii,
    ModuleShieldGeneratorI,
    ModuleShieldGeneratorIi,
}
impl fmt::Display for ShipModule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

#[derive(Deserialize, PartialEq, Eq, Copy, Clone, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ShipFrame {
    FrameProbe,
    FrameDrone,
    FrameInterceptor,
    FrameRacer,
    FrameFighter,
    FrameFrigate,
    FrameShuttle,
    FrameExplorer,
    FrameMiner,
    FrameLightFreighter,
    FrameHeavyFreighter,
    FrameTransport,
    FrameDestroyer,
    FrameCruiser,
    FrameCarrier,
}
impl fmt::Display for ShipFrame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

#[derive(Deserialize, PartialEq, Eq, Copy, Clone, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ShipEngine {
    EngineImpulseDriveI,
    EngineIonDriveI,
    EngineIonDriveIi,
    EngineHyperDriveI,
}
impl fmt::Display for ShipEngine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Copy, Clone, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FlightMode {
    Cruise,
    Burn,
    Drift,
    Stealth,
}
impl fmt::Display for FlightMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Copy, Clone, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ShipCrewRotation {
    Strict,
    Relaxed,
}
impl fmt::Display for ShipCrewRotation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Default, Eq, Copy, Clone, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FactionSymbols {
    Cosmic,
    Void,
    Galactic,
    Quantum,
    Dominion,
    Astro,
    Corsairs,
    Obsidian,
    Aegis,
    United,
    Solitary,
    Cobalt,
    Omega,
    Echo,
    Lords,
    Cult,
    Ancients,
    Shadow,
    Ethereal,
    #[default]
    Default,
}
impl fmt::Display for FactionSymbols {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GetMarketType {
    Purchase,
    Sell,
}
impl fmt::Display for GetMarketType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GetMarketSupplyType {
    Scarce,
    Limited,
    Moderate,
    Abundant,
}
impl fmt::Display for GetMarketSupplyType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DepositSize {
    Small,
    Moderate,
    Large,
}
impl fmt::Display for DepositSize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}
