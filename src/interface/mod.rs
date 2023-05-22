pub mod responses;
use responses::{agents::*, contracts::*, fleet::*, systems::*}; //factions::*
pub mod requests;
use requests::*;
pub mod enums;
use enums::*;

use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE},
    Client,
};
use serde::{Deserialize, Serialize};
use tokio::{
    sync::{mpsc, oneshot},
    task,
    time::{sleep, Duration},
};
use url::Url;

use crate::interface::responses::GetRegistrationL0;

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

#[derive(Debug, Clone)]
struct SpaceTraders {
    credentials: Credentials,
    client: Client,
    url: String,
}

impl SpaceTraders {
    pub fn new(credentials: Credentials) -> Self {
        SpaceTraders {
            credentials,
            client: reqwest::Client::new(),
            url: String::from("https://api.spacetraders.io"),
        }
    }

    fn get_header(&self, json: Option<String>) -> HeaderMap {
        let mut headers = HeaderMap::with_capacity(1);
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_bytes(format!("Bearer {}", self.credentials.token).as_bytes())
                .unwrap(),
        );
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        if let Some(data) = json {
            print!("{}", data.len());
            headers.insert(CONTENT_LENGTH, data.len().to_string().parse().unwrap());
        }
        headers
    }

    fn get_url(&self, endpoint: &str) -> Url {
        Url::parse(format!("{}{}", self.url, endpoint).as_str()).unwrap()
    }

    pub async fn make_reqwest(&self, method: Method, url: &str, data: Option<String>) -> String {
        let response = match method {
            Method::Get => match data {
                Some(json) => {
                    self.client
                        .get(self.get_url(url))
                        .json(&json)
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
                        .json(&json)
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
                        .json(&json)
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
                println!("{}", response);
                response
            }

            Err(error) => panic!("{}", error),
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
    pub async fn new(token: &str) -> Self {
        let space_trader = SpaceTraders::new(Credentials::new(token));

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
        let client = reqwest::Client::new();
        let response = client
            .post("https://api.spacetraders.io/v2/register")
            .json(r#""#)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
        let token: GetRegistrationL0 = serde_json::from_str(&response).unwrap();

        let credentials = Credentials::new(&token.data.token);
        let space_trader = SpaceTraders::new(credentials);

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

    pub fn make_json<T: Serialize>(&self, data: T) -> String {
        serde_json::to_string(&data).unwrap()
    }

    pub async fn make_request(&self, method: Method, url: String, data: Option<String>) -> String {
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
        let Ok(res) = oneshot_receiver.await else { panic!("Reponse was bad!")
        };
        res
    }

    // Agents
    pub async fn agent(&self) -> AgentL0 {
        serde_json::from_str(
            &self
                .make_request(Method::Get, "/v2/my/agent".to_string(), None)
                .await,
        )
        .unwrap()
    }

    // Systems
    pub async fn list_systems(&self) -> ListSystemsL0 {
        serde_json::from_str(
            &self
                .make_request(Method::Get, "/v2/systems".to_string(), None)
                .await,
        )
        .unwrap()
    }
    pub async fn get_system(&self, system_symbol: &str) -> GetSystemL0 {
        serde_json::from_str(
            &self
                .make_request(Method::Get, format!("/v2/systems/{}", system_symbol), None)
                .await,
        )
        .unwrap()
    }
    pub async fn list_waypoints(&self, system_symbol: &str) -> ListWaypointsL0 {
        serde_json::from_str(
            &self
                .make_request(
                    Method::Get,
                    format!("/v2/systems/{}/waypoints", system_symbol),
                    None,
                )
                .await,
        )
        .unwrap()
    }
    pub async fn get_waypoint(&self, system_symbol: &str, waypoint_symbol: &str) -> GetWaypointL0 {
        serde_json::from_str(
            &self
                .make_request(
                    Method::Get,
                    format!(
                        "/v2/systems/{}/waypoints/{}",
                        system_symbol, waypoint_symbol
                    ),
                    None,
                )
                .await,
        )
        .unwrap()
    }
    pub async fn get_market(&self, system_symbol: &str, waypoint_symbol: &str) -> GetMarketL0 {
        serde_json::from_str(
            &self
                .make_request(
                    Method::Get,
                    format!(
                        "/v2/systems/{}/waypoints/{}/market",
                        system_symbol, waypoint_symbol
                    ),
                    None,
                )
                .await,
        )
        .unwrap()
    }
    pub async fn get_shipyard(&self, system_symbol: &str, waypoint_symbol: &str) -> GetShipyardL0 {
        serde_json::from_str(
            &self
                .make_request(
                    Method::Get,
                    format!(
                        "/v2/systems/{}/waypoints/{}/shipyard",
                        system_symbol, waypoint_symbol
                    ),
                    None,
                )
                .await,
        )
        .unwrap()
    }
    pub async fn jump_gate(&self, system_symbol: &str, waypoint_symbol: &str) {
        serde_json::from_str(
            &self
                .make_request(
                    Method::Get,
                    format!(
                        "/v2/systems/{}/waypoints/{}/jump-gate",
                        system_symbol, waypoint_symbol
                    ),
                    None,
                )
                .await,
        )
        .unwrap()
    }

    // Contracts
    pub async fn list_contracts(&self) -> ContractTermsL0 {
        serde_json::from_str(
            &self
                .make_request(Method::Get, String::from("/v2/my/contracts"), None)
                .await,
        )
        .unwrap()
    }

    pub async fn get_contract(&self, contract_id: &str) -> ContractTermsL0 {
        serde_json::from_str(
            &self
                .make_request(
                    Method::Get,
                    format!("/v2/my/contracts/{}", contract_id),
                    None,
                )
                .await,
        )
        .unwrap()
    }

    pub async fn accept_contract(&self, contract_id: &str) -> AcceptContractL0 {
        serde_json::from_str(
            &self
                .make_request(
                    Method::Post,
                    format!("/v2/my/contracts/{}/accept", contract_id),
                    None,
                )
                .await,
        )
        .unwrap()
    }

    pub async fn deliver_contract(&self, contract_id: &str) {
        serde_json::from_str(
            &self
                .make_request(
                    Method::Post,
                    format!("/v2/my/contracts/{}/deliver", contract_id),
                    None,
                )
                .await,
        )
        .unwrap()
    }

    pub async fn fulfill_contract(&self, contract_id: &str) {
        serde_json::from_str(
            &self
                .make_request(
                    Method::Post,
                    format!("/v2/my/contracts/{}/fulfill", contract_id),
                    None,
                )
                .await,
        )
        .unwrap()
    }

    // Fleet
    pub async fn list_ships(&self) -> ListShipsL0 {
        serde_json::from_str(
            &self
                .make_request(Method::Get, String::from("/v2/my/ships"), None)
                .await,
        )
        .unwrap()
    }
    pub async fn purchase_ship(&self, data: ShipType) {
        serde_json::from_str(
            &self
                .make_request(
                    Method::Post,
                    String::from("/v2/my/ships"),
                    Some(self.make_json(data)),
                )
                .await,
        )
        .unwrap()
    }
    pub async fn get_ship(&self, ship_symbol: String) {
        // the ship symbol might be an enum I already have
        serde_json::from_str(
            &self
                .make_request(Method::Get, format!("/v2/my/ships/{}", ship_symbol), None)
                .await,
        )
        .unwrap()
    }
    pub async fn get_ship_cargo(&self, ship_symbol: String) {
        serde_json::from_str(
            &self
                .make_request(
                    Method::Get,
                    format!("/v2/my/ships{}/cargo", ship_symbol),
                    None,
                )
                .await,
        )
        .unwrap()
    }
    pub async fn orbit_ship(&self, ship_symbol: String) {
        serde_json::from_str(
            &self
                .make_request(
                    Method::Post,
                    format!("/v2/my/ships/{}/orbit", ship_symbol),
                    None,
                )
                .await,
        )
        .unwrap()
    }
    pub async fn ship_refine(&self, ship_symbol: String, data: ShipRefine) {
        serde_json::from_str(
            &self
                .make_request(
                    Method::Post,
                    format!("/v2/my/ships/{}/refine", ship_symbol),
                    Some(self.make_json(data)),
                )
                .await,
        )
        .unwrap()
    }
    pub async fn create_chart(&self, ship_symbol: String) {
        serde_json::from_str(
            &self
                .make_request(
                    Method::Post,
                    format!("/v2/my/ships/{}/chart", ship_symbol),
                    None,
                )
                .await,
        )
        .unwrap()
    }

    // Factions
    pub async fn list_factions(&self) {
        serde_json::from_str(
            &self
                .make_request(Method::Get, String::from("/v2/factions"), None)
                .await,
        )
        .unwrap()
    }
    pub async fn get_faction(&self, faction_symbol: &str) {
        serde_json::from_str(
            &self
                .make_request(
                    Method::Get,
                    format!("/v2/factions/{}", faction_symbol),
                    None,
                )
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
    pub data: Option<String>,
}
#[derive(Debug)]
pub struct ChannelMessage {
    message: RequestMessage,
    oneshot: oneshot::Sender<String>,
}

// Other helpful structs and enums

#[allow(dead_code)]
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
