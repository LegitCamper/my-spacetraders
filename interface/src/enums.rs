use serde::{Deserialize, Serialize};

// USE THE COMMAND 'ccase -t pascal enums' TO TRANSFORM THE DOCS TO RUST ENUMS

// TODO: use macros to impl the display trait instead of all the duplication

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ListContractsType {
    Procurement,
    Transport,
    Shuttle,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
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

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
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

#[derive(Serialize, Deserialize, PartialEq, Eq, Default, Debug)]
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

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
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

#[derive(Serialize, Deserialize, PartialEq, Eq, Copy, Clone, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ShipReactor {
    ReactorSolarI,
    ReactorFusionI,
    ReactorFissionI,
    ReactorChemicalI,
    ReactorAntimatterI,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Copy, Clone, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ShipNavStatus {
    InTransit,
    InOrbit,
    Docked,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Copy, Clone, Debug)]
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

#[derive(Serialize, Deserialize, PartialEq, Eq, Copy, Clone, Debug)]
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

#[derive(Serialize, Deserialize, PartialEq, Eq, Copy, Clone, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ShipEngine {
    EngineImpulseDriveI,
    EngineIonDriveI,
    EngineIonDriveIi,
    EngineHyperDriveI,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Copy, Clone, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FlightMode {
    Cruise,
    Burn,
    Drift,
    Stealth,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Copy, Clone, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ShipCrewRotation {
    Strict,
    Relaxed,
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GetMarketType {
    Purchase,
    Sell,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GetMarketSupplyType {
    Scarce,
    Limited,
    Moderate,
    Abundant,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DepositSize {
    Small,
    Moderate,
    Large,
}
