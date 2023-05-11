use reqwest::{
    header::{HeaderMap, HeaderValue, CONTENT_TYPE},
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
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        headers
    }

    pub fn make_json<T: Serialize>(&self, data: T) -> String{
        serde_json::to_string(&data).unwrap()
        
    }

    pub async fn make_reqwest(&self, method: Method, url: &str, data: Option<String>) -> String {
        let response = match method {
            Method::Get => match data {
                Some(json) => {
                    self.client
                        .get(self.get_url(url))
                        .json(&json)
                        .headers(self.get_header())
                        .send()
                        .await
                        .unwrap()
                        .text()
                        .await
                }
                None => {
                    self.client
                        .get(self.get_url(url))
                        .headers(self.get_header())
                        .send()
                        .await
                        .unwrap()
                        .text()
                        .await
                }
            },

            Method::Post => match data {
                Some(json) => {
                    self.client
                        .get(self.get_url(url))
                        .json(&json)
                        .headers(self.get_header())
                        .send()
                        .await
                        .unwrap()
                        .text()
                        .await
                }
                None => {
                    self.client
                        .post(self.get_url(url))
                        .headers(self.get_header())
                        .send()
                        .await
                        .unwrap()
                        .text()
                        .await
                }
            },
            Method::Patch => match data {
                Some(json) => {
                    self.client
                        .patch(self.get_url(url))
                        .json(&json)
                        .headers(self.get_header())
                        .send()
                        .await
                        .unwrap()
                        .text()
                        .await
                }
                None => {
                    self.client
                        .post(self.get_url(url))
                        .headers(self.get_header())
                        .send()
                        .await
                        .unwrap()
                        .text()
                        .await
                }
            },
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
        serde_json::from_str(&self.make_reqwest(Method::Get, "/v2/my/agent", None).await).unwrap()
    }

    pub async fn waypoint_details(&self, system_symbol: String, waypoint_symbol: String) -> String {
        self.make_reqwest(
            Method::Get,
            &format!(
                "/v2/systems/{}/waypoints/{}",
                system_symbol, waypoint_symbol
            ),
            None,
        )
        .await
    }

    pub async fn waypoint_custom(
        &self,
        system_symbol: String,
        waypoint_symbol: String,
        endpoint: &str,
    ) -> String {
        self.make_reqwest(
            Method::Get,
            &format!(
                "/v2/systems/{}/waypoints/{}/{}",
                system_symbol, waypoint_symbol, endpoint
            ),
            None,
        )
        .await
    }

    pub async fn waypoint_list(&self, system_symbol: String) -> WaypointsListedL0 {
        serde_json::from_str(
            &self
                .make_reqwest(
                    Method::Get,
                    &format!("/v2/systems/{}/waypoints", system_symbol),
                    None,
                )
                .await,
        )
        .unwrap()
    }

    pub async fn contract_accept(&self, contract_id: &str) -> String {
        self.make_reqwest(
            Method::Post,
            &format!("/v2/my/contracts/{}/accept", contract_id),
            None,
        )
        .await
    }

    pub async fn contract_terms(&self, contract_id: &str) -> ContractTermsL0 {
        serde_json::from_str(
            &self
                .make_reqwest(
                    Method::Get,
                    &format!("/v2/my/contracts/{}", contract_id),
                    None,
                )
                .await,
        )
        .unwrap()
    }

    pub async fn flight_mode_change(&self, ship_symbol: &str, data: ChangeFlightMode) {
        serde_json::from_str(
            &self
                .make_reqwest(
                    Method::Patch,
                    &format!("/v2/my/ships/{}/nav", ship_symbol),
                    Some(self.make_json(data)),
                )
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

#[derive(Serialize, Deserialize, Debug)]
pub enum FlightMode {
    #[serde(alias = "CRUISE")]
    Cruise,
    #[serde(alias = "BURN")]
    Burn,
    #[serde(alias = "DRIFT")]
    Drift,
    #[serde(alias = "STEALTH")]
    Stealth
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
    Patch,
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
