use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug)]
pub struct Credentials {
    pub token: String,
}

impl Credentials {
    pub fn new(token: &str) -> Self {
        Credentials {
            token: String::from(token),
        }
    }
}

#[derive(Debug)]
pub struct SpaceTraders {
    credentials: Credentials,
    pub client: Client,
    pub url: String,
}

impl SpaceTraders {
    pub fn new(credentials: Credentials) -> Self {
        SpaceTraders {
            credentials,
            client: reqwest::Client::new(),
            url: String::from("https://api.spacetraders.io"),
        }
    }

    fn get_header(&self) -> HeaderMap {
        let mut headers = HeaderMap::with_capacity(1);
        headers.insert(
            "Authorization",
            HeaderValue::from_bytes(format!("Bearer {}", self.credentials.token).as_bytes())
                .unwrap(),
        );
        headers
    }

    pub async fn make_reqwest(&self, method: Method, url: &str) -> String {
        let response = match method {
            Method::Get => {
                self.client
                    .get(self.get_url(url))
                    .headers(self.get_header())
                    .send()
                    .await
                    .unwrap()
                    .text()
                    .await
            }

            Method::Post => {
                self.client
                    .post(self.get_url(url))
                    .headers(self.get_header())
                    .send()
                    .await
                    .unwrap()
                    .text()
                    .await
            }
        };

        match response {
            Ok(response) => response,
            Err(error) => panic!("{}", error),
        }
    }
    fn get_url(&self, endpoint: &str) -> Url {
        Url::parse(format!("{}{}", self.url, endpoint).as_str()).unwrap()
    }

    pub async fn agent_details(&self) -> AgentInfoL0 {
        serde_json::from_str(&self.make_reqwest(Method::Get, "/v2/my/agent").await).unwrap()
    }

    pub async fn waypoint_details(&self, system_symbol: String, waypoint_symbol: String) -> String {
        self.make_reqwest(
            Method::Get,
            &format!(
                "/v2/systems/{}/waypoints/{}",
                system_symbol, waypoint_symbol
            ),
        )
        .await
    }

    pub async fn waypoint_list(&self, system_symbol: String) -> WaypointsListedL0 {
        serde_json::from_str(
            &self
                .make_reqwest(
                    Method::Get,
                    &format!("/v2/systems/{}/waypoints", system_symbol),
                )
                .await,
        )
        .unwrap()
    }

    pub async fn contract_accept(&self, contract_id: &str) -> String {
        self.make_reqwest(
            Method::Post,
            &format!("/v2/my/contracts/{}/accept", contract_id),
        )
        .await
    }

    pub async fn contract_terms(&self, contract_id: &str) -> ContractTermsL0 {
        serde_json::from_str(
            &self
                .make_reqwest(Method::Get, &format!("/v2/my/contracts/{}", contract_id))
                .await,
        )
        .unwrap()
    }
}

// Other helpful structs and enums

#[derive(Deserialize, Debug)]
pub enum Item {
    // not sure this is a good idea
    #[serde(alias = "ALUMINUM_ORE")]
    AluminumOre,
}

#[derive(Deserialize, Debug)]
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
pub enum faction {
    #[serde(alias = "COSMIC")]
    Cosmic,
}

#[derive(Deserialize, Debug)]
pub struct Coordinates {
    x: f64,
    y: f64,
}

pub fn parse_waypoint(waypoint: String) -> Waypoint {
    let waypoint_split: Vec<&str> = waypoint.split('-').collect();
    Waypoint {
        sector: waypoint_split[0].to_string(),
        system: format!("{}-{}", waypoint_split[0], waypoint_split[1]),
        waypoint,
    }
}
#[derive(Debug)]
pub struct Waypoint {
    pub sector: String,
    pub system: String,
    pub waypoint: String,
}

pub enum Method {
    Post,
    Get,
    // Delete,
}

// Other structs for reponses from spacetrades

#[derive(Deserialize, Debug)]
pub struct AgentInfoL0 {
    pub data: AgentInfoL1,
}
#[derive(Deserialize, Debug)]
pub struct AgentInfoL1 {
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
    data: ContractTermsL1,
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
    data: Vec<WaypointsListedL1>,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct WaypointsListedL1 {
    #[serde(alias = "systemSymbol")]
    system_symbol: String,
    symbol: String,
    r#type: WaypointTrait,
    x: i64,
    y: i64,
    orbitals: Vec<WaypointsListedOrbitals>,
    traits: Vec<WaypointsListedTraits>,
    chart: WaypointsListedCharts,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct WaypointsListedOrbitals {
    symbol: String,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct WaypointsListedTraits {
    symbol: WaypointTrait,
    name: String,
    #[serde(default)]
    desciption: String,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct WaypointsListedCharts {
    #[serde(alias = "submittedBy")]
    submitted_by: faction,
    #[serde(alias = "submittedOn")]
    submitted_on: String,
    // desciption: String,
    // faction: Vec<faction>,
}
