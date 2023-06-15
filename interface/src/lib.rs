pub mod enums;
pub mod requests;
pub mod responses;
mod tests;

use requests::*;
use responses::{agents, contracts, factions, fleet, systems};

use convert_case::{Case, Casing};
use get_size::GetSize;
use random_string::generate;
use reqwest::{
    header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE},
    Client,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use tokio::{
    sync::{mpsc, oneshot},
    task,
    time::{sleep, Duration},
};
use url::Url;

const LIVEURL: &str = "https://api.spacetraders.io/v2";
const MOCKURL: &str = "https://stoplight.io/mocks/spacetraders/spacetraders/96627693";

// use self::responses::factions::{Faction, Factions};

// TODO: better error handling
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Error {
    InsufficientFunds(HashMap<String, u32>),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Method {
    Post,
    Get,
    Patch,
    // Delete,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum SpaceTradersEnv {
    Live,
    Mock,
}

#[derive(Debug, Clone)]
struct SpaceTraders {
    credentials: Credentials,
    client: Client,
    url: String,
    enviroment: SpaceTradersEnv,
}

impl SpaceTraders {
    pub fn new(credentials: Credentials, enviroment: SpaceTradersEnv) -> Self {
        let url = match enviroment {
            SpaceTradersEnv::Live => LIVEURL,
            SpaceTradersEnv::Mock => MOCKURL,
        };

        SpaceTraders {
            credentials,
            client: reqwest::Client::new(),
            url: String::from(url),
            enviroment,
        }
    }

    fn get_header(&self, json: Option<HashMap<String, String>>) -> HeaderMap {
        let mut headers = HeaderMap::with_capacity(4);
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_bytes(format!("Bearer {}", self.credentials.token).as_bytes())
                .unwrap(),
        );
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        headers.insert(ACCEPT, "application/json".parse().unwrap());
        if let Some(data) = json {
            headers.insert(CONTENT_LENGTH, GetSize::get_heap_size(&data).into());
        } else {
            headers.insert(CONTENT_LENGTH, 0.into());
        }
        // adds dynamic header for unit tests
        if self.enviroment == SpaceTradersEnv::Mock {
            headers.insert("Prefer", "dynamic=true".parse().unwrap());
        }
        headers
    }

    fn get_url(&self, endpoint: &str) -> Url {
        Url::parse(format!("{}{}", self.url, endpoint).as_str()).unwrap()
    }

    pub async fn make_reqwest(
        &self,
        method: Method,
        url: &str,
        data: Option<HashMap<String, String>>,
    ) -> String {
        let response = match method {
            Method::Get => match data {
                Some(json) => {
                    self.client
                        .get(self.get_url(url))
                        .form(&json)
                        .headers(self.get_header(Some(json)))
                        .send()
                        .await
                        .unwrap()
                        .text()
                        .await
                }
                None => {
                    self.client
                        .get(self.get_url(url))
                        .headers(self.get_header(None))
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
                        .post(self.get_url(url))
                        .form(&json)
                        .headers(self.get_header(Some(json)))
                        .send()
                        .await
                        .unwrap()
                        .text()
                        .await
                }
                None => {
                    self.client
                        .post(self.get_url(url))
                        .headers(self.get_header(None))
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
                        .form(&json)
                        .headers(self.get_header(Some(json)))
                        .send()
                        .await
                        .unwrap()
                        .text()
                        .await
                }
                None => {
                    self.client
                        .patch(self.get_url(url))
                        .headers(self.get_header(None))
                        .send()
                        .await
                        .unwrap()
                        .text()
                        .await
                }
            },
        };

        match response {
            Ok(response) => {
                // check if server error - Expects error here
                let error: Result<responses::Error, _> = serde_json::from_str(&response);

                // if error print
                if let Ok(error) = error {
                    panic!(
                        "\nerror: {:?}\nurl: {}\ncreds: {:?}\n",
                        error, url, self.credentials
                    );
                } else {
                    response
                }
            }
            // client error
            Err(error) => panic!("{}", error),
        }
    }

    pub async fn custom_endpoint(&self, endpoint: &str, method: Method) -> String {
        match method {
            Method::Post => self
                .client
                .post(&format!("{}{}", self.url, endpoint))
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap(),
            Method::Get => self
                .client
                .get(&format!("{}{}", self.url, endpoint))
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap(),
            Method::Patch => todo!(),
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct SpaceTradersHandler {
    info: SpaceTraders,
    channel: mpsc::Sender<ChannelMessage>,
    task: task::JoinHandle<()>,
}

impl SpaceTradersHandler {
    pub async fn new(token: &str, enviroment: SpaceTradersEnv) -> Self {
        let space_trader = SpaceTraders::new(Credentials::new(token), enviroment);

        let (channel_sender, mut channel_receiver) = mpsc::channel(500);

        SpaceTradersHandler {
            info: space_trader.clone(),
            channel: channel_sender,
            task: task::spawn(async move {
                loop {
                    sleep(Duration::from_millis(500)).await; // only 2 requests per second
                    let msg = channel_receiver.recv().await.unwrap();
                    msg.oneshot
                        .send(
                            space_trader
                                .make_reqwest(
                                    msg.message.method,
                                    msg.message.url.as_str(),
                                    msg.message.data,
                                )
                                .await,
                        )
                        .unwrap();
                }
            }),
        }
    }

    pub async fn default() -> Self {
        let username = generate(14, "abcdefghijklmnopqrstuvwxyz1234567890_");
        let post_message = json!({"faction": "QUANTUM", "symbol": username});

        let registration = reqwest::Client::new()
            .post(&format!("{}/register", LIVEURL))
            .header(CONTENT_LENGTH, post_message.to_string().chars().count())
            .json(&post_message)
            .send()
            .await
            .unwrap()
            .json::<responses::GetRegistrationL0>()
            .await
            .unwrap();

        SpaceTradersHandler::new(&registration.data.token, SpaceTradersEnv::Live).await
    }

    async fn new_testing() -> Self {
        let username = generate(14, "abcdefghijklmnopqrstuvwxyz1234567890_");
        let post_message = json!({"faction": "QUANTUM", "symbol": username});

        let registration = reqwest::Client::new()
            .post(&format!("{}/register", MOCKURL))
            .header(CONTENT_LENGTH, post_message.to_string().chars().count())
            .json(&post_message)
            .send()
            .await
            .unwrap()
            .json::<responses::GetRegistrationL0>()
            .await
            .unwrap();

        SpaceTradersHandler::new(&registration.data.token, SpaceTradersEnv::Mock).await
    }

    pub fn diagnose(&self) {
        panic!(
            "\ntoken: {}\nurl: {}\nenviroment: {:#?}",
            self.info.credentials.token, self.info.url, self.info.enviroment
        )
    }

    pub fn make_json<T: Serialize>(&self, data: T) -> HashMap<String, String> {
        let string = serde_json::to_string(&data).unwrap();
        serde_json::from_str(&string).unwrap()
    }

    pub async fn make_request(
        &self,
        method: Method,
        url: String,
        data: Option<HashMap<String, String>>,
    ) -> String {
        // make oneshot channel
        let (oneshot_sender, oneshot_receiver) = oneshot::channel();

        // make request
        self.channel
            .send(ChannelMessage {
                message: RequestMessage { method, url, data },
                oneshot: oneshot_sender,
            })
            .await
            .unwrap(); //.await.unwrap();

        // listen to oneshot for response
        match oneshot_receiver.await {
            Ok(res) => res,
            Err(err) => {
                self.diagnose();
                panic!("Interface failed to send back a correct response: {}", err);
            }
        }
    }

    // Agents
    pub async fn agent(&self) -> agents::Agent {
        serde_json::from_str(
            &self
                .make_request(Method::Get, "/my/agent".to_string(), None)
                .await,
        )
        .unwrap()
    }

    // Systems
    pub async fn list_systems(&self) -> systems::Systems {
        serde_json::from_str(
            &self
                .make_request(Method::Get, "/systems".to_string(), None)
                .await,
        )
        .unwrap()
    }
    pub async fn get_system(&self, system_symbol: &str) -> systems::System {
        serde_json::from_str(
            &self
                .make_request(Method::Get, format!("/systems/{}", system_symbol), None)
                .await,
        )
        .unwrap()
    }
    pub async fn list_waypoints(&self, system_symbol: &str) -> systems::Waypoints {
        serde_json::from_str(
            &self
                .make_request(
                    Method::Get,
                    format!("/systems/{}/waypoints", system_symbol),
                    None,
                )
                .await,
        )
        .unwrap()
    }
    pub async fn get_waypoint(
        &self,
        system_symbol: &str,
        waypoint_symbol: &str,
    ) -> systems::Waypoint {
        serde_json::from_str(
            &self
                .make_request(
                    Method::Get,
                    format!("/systems/{}/waypoints/{}", system_symbol, waypoint_symbol),
                    None,
                )
                .await,
        )
        .unwrap()
    }
    pub async fn get_market(&self, system_symbol: &str, waypoint_symbol: &str) -> systems::Market {
        serde_json::from_str(
            &self
                .make_request(
                    Method::Get,
                    format!(
                        "/systems/{}/waypoints/{}/market",
                        system_symbol, waypoint_symbol
                    ),
                    None,
                )
                .await,
        )
        .unwrap()
    }
    pub async fn get_shipyard(
        &self,
        system_symbol: &str,
        waypoint_symbol: &str,
    ) -> systems::Shipyard {
        serde_json::from_str(
            &self
                .make_request(
                    Method::Get,
                    format!(
                        "/systems/{}/waypoints/{}/shipyard",
                        system_symbol, waypoint_symbol
                    ),
                    None,
                )
                .await,
        )
        .unwrap()
    }
    pub async fn jump_gate(&self, system_symbol: &str, waypoint_symbol: &str) -> systems::JumpGate {
        serde_json::from_str(
            &self
                .make_request(
                    Method::Get,
                    format!(
                        "/systems/{}/waypoints/{}/jump-gate",
                        system_symbol, waypoint_symbol
                    ),
                    None,
                )
                .await,
        )
        .unwrap()
    }

    // Contracts
    pub async fn list_contracts(&self) -> contracts::Contracts {
        serde_json::from_str(
            &self
                .make_request(Method::Get, String::from("/my/contracts"), None)
                .await,
        )
        .unwrap()
    }
    pub async fn get_contract(&self, contract_id: &str) -> contracts::Contract {
        serde_json::from_str(
            &self
                .make_request(Method::Get, format!("/my/contracts/{}", contract_id), None)
                .await,
        )
        .unwrap()
    }
    pub async fn accept_contract(&self, contract_id: &str) -> contracts::AcceptContract {
        serde_json::from_str(
            &self
                .make_request(
                    Method::Post,
                    format!("/my/contracts/{}/accept", contract_id),
                    None,
                )
                .await,
        )
        .unwrap()
    }
    pub async fn deliver_contract(&self, contract_id: &str) -> contracts::DeliverContract {
        serde_json::from_str(
            &self
                .make_request(
                    Method::Post,
                    format!("/my/contracts/{}/deliver", contract_id),
                    None,
                )
                .await,
        )
        .unwrap()
    }
    pub async fn fulfill_contract(&self, contract_id: &str) -> contracts::FulfillContract {
        serde_json::from_str(
            &self
                .make_request(
                    Method::Post,
                    format!("/my/contracts/{}/fulfill", contract_id),
                    None,
                )
                .await,
        )
        .unwrap()
    }

    // Fleet
    pub async fn list_ships(&self) -> fleet::Ships {
        serde_json::from_str(
            &self
                .make_request(Method::Get, String::from("/my/ships"), None)
                .await,
        )
        .unwrap()
    }
    pub async fn purchase_ship(&self, data: BuyShip) -> fleet::PurchaseShip {
        serde_json::from_str(
            &self
                .make_request(
                    Method::Post,
                    String::from("/my/ships"),
                    Some(self.make_json(data)),
                )
                .await,
        )
        .unwrap()
    }
    pub async fn get_ship(&self, ship_symbol: &str) -> fleet::Ship {
        // the ship symbol might be an enum I already have
        serde_json::from_str(
            &self
                .make_request(Method::Get, format!("/my/ships/{}", ship_symbol), None)
                .await,
        )
        .unwrap()
    }
    pub async fn get_ship_cargo(&self, ship_symbol: &str) -> fleet::ShipCargo {
        serde_json::from_str(
            &self
                .make_request(Method::Get, format!("/my/ships{}/cargo", ship_symbol), None)
                .await,
        )
        .unwrap()
    }
    pub async fn orbit_ship(&self, ship_symbol: &str) -> fleet::OrbitShip {
        serde_json::from_str(
            &self
                .make_request(
                    Method::Post,
                    format!("/my/ships/{}/orbit", ship_symbol),
                    None,
                )
                .await,
        )
        .unwrap()
    }
    pub async fn ship_refine(&self, ship_symbol: &str, data: ShipRefine) -> fleet::ShipRefine {
        serde_json::from_str(
            &self
                .make_request(
                    Method::Post,
                    format!("/my/ships/{}/refine", ship_symbol),
                    Some(self.make_json(data)),
                )
                .await,
        )
        .unwrap()
    }
    pub async fn create_chart(&self, ship_symbol: &str) -> fleet::CreateChart {
        serde_json::from_str(
            &self
                .make_request(
                    Method::Post,
                    format!("/my/ships/{}/chart", ship_symbol),
                    None,
                )
                .await,
        )
        .unwrap()
    }

    // Factions
    pub async fn list_factions(&self) -> factions::Factions {
        serde_json::from_str(
            &self
                .make_request(Method::Get, String::from("/factions"), None)
                .await,
        )
        .unwrap()
    }
    pub async fn get_faction(&self, faction_symbol: &str) -> factions::Factions {
        serde_json::from_str(
            &self
                .make_request(Method::Get, format!("/factions/{}", faction_symbol), None)
                .await,
        )
        .unwrap()
    }
}

// struct to handle the dataflow through channel
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RequestMessage {
    pub method: Method,
    pub url: String,
    pub data: Option<HashMap<String, String>>,
}
#[derive(Debug)]
pub struct ChannelMessage {
    message: RequestMessage,
    oneshot: oneshot::Sender<String>,
}

// Other helpful functions

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Coordinates {
    x: f64,
    y: f64,
}

pub fn parse_waypoint(waypoint: &str) -> Waypoint {
    let waypoint_split: Vec<&str> = waypoint.split('-').collect();
    Waypoint {
        sector: waypoint_split[0].to_string(), // X1
        system: format!("{}-{}", waypoint_split[0], waypoint_split[1]), // X1-DF55
        waypoint: waypoint.to_string(),        // X1-DF55-20250Z
    }
}

#[derive(Debug)]
pub struct Waypoint {
    pub sector: String,
    pub system: String,
    pub waypoint: String,
}

pub fn enum_to_string<T: std::fmt::Display>(name: T) -> String {
    name.to_string().to_case(Case::UpperSnake)
}
