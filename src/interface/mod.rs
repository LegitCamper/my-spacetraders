pub mod lib;

use lib::*;

use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE},
    Client,
};
use serde::{Deserialize, Serialize};
use tokio::{
    sync::broadcast,
    task,
    time::{sleep, Duration},
};
use url::Url;

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

    fn get_header(&self) -> HeaderMap {
        let mut headers = HeaderMap::with_capacity(1);
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_bytes(format!("Bearer {}", self.credentials.token).as_bytes())
                .unwrap(),
        );
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        headers
    }

    pub fn make_json<T: Serialize>(&self, data: T) -> String {
        serde_json::to_string(&data).unwrap()
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
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct SpaceTradersHandler {
    info: SpaceTraders,
    channel: broadcast::Sender<Broadcaster>,
    task: task::JoinHandle<()>,
}

impl SpaceTradersHandler {
    pub async fn new(credentials: Credentials) -> Self {
        let space_trader = SpaceTraders::new(credentials);

        let (channel, _receiver) = broadcast::channel(100);

        let mut reciver_channel = channel.subscribe();

        let task_sender = channel.clone();
        let task_space_trader = space_trader.clone();

        SpaceTradersHandler {
            info: space_trader,
            channel,
            task: task::spawn(async move {
                loop {
                    sleep(Duration::from_millis(500)).await; // only 2 requests per second
                    let response = reciver_channel.recv().await.unwrap();
                    if let Broadcaster::Interface(method, url, data) = response {
                        task_sender
                            .send(Broadcaster::Caller(
                                task_space_trader
                                    .make_reqwest(method, url.as_str(), data)
                                    .await,
                            ))
                            .unwrap();
                    }
                }
            }),
        }
    }

    pub fn make_json<T: Serialize>(&self, data: T) -> String {
        serde_json::to_string(&data).unwrap()
    }

    pub async fn make_request(&self, message: Broadcaster) -> String {
        // set up listener for response
        let mut channel = self.channel.subscribe();

        // make request
        self.channel.send(message).unwrap();

        loop {
            if let Broadcaster::Caller(msg) = channel.recv().await.unwrap() {
                return msg;
            }
        }
    }

    pub async fn agent(&self) -> Option<AgentL0> {
        serde_json::from_str(
            &self
                .make_request(Broadcaster::Interface(
                    Method::Get,
                    "/v2/my/agent".to_string(),
                    None,
                ))
                .await,
        )
        .unwrap()
    }

    pub async fn list_systems(&self) {
        serde_json::from_str(
            &self
                .info
                .make_reqwest(Method::Get, "/v2/systems", None)
                .await,
        )
        .unwrap()
    }

    pub async fn get_system(&self, system_symbol: &str) {
        serde_json::from_str(
            &self
                .info
                .make_reqwest(Method::Get, &format!("/v2/systems/{}", system_symbol), None)
                .await,
        )
        .unwrap()
    }
    pub async fn list_waypoints(&self, system_symbol: &str) -> WaypointsListedL0 {
        serde_json::from_str(
            &self
                .info
                .make_reqwest(
                    Method::Get,
                    &format!("/v2/systems/{}/waypoints", system_symbol),
                    None,
                )
                .await,
        )
        .unwrap()
    }
    pub async fn get_waypoint(&self, system_symbol: &str, waypoint_symbol: &str) {
        serde_json::from_str(
            &self
                .info
                .make_reqwest(
                    Method::Get,
                    &format!(
                        "/v2/systems/{}/waypoints/{}",
                        system_symbol, waypoint_symbol
                    ),
                    None,
                )
                .await,
        )
        .unwrap()
    }
    pub async fn get_market(&self, system_symbol: &str, waypoint_symbol: &str) {
        serde_json::from_str(
            &self
                .info
                .make_reqwest(
                    Method::Get,
                    &format!(
                        "/v2/systems/{}/waypoints/{}/market",
                        system_symbol, waypoint_symbol
                    ),
                    None,
                )
                .await,
        )
        .unwrap()
    }
    pub async fn get_shipyard(&self, system_symbol: &str, waypoint_symbol: &str) {
        serde_json::from_str(
            &self
                .info
                .make_reqwest(
                    Method::Get,
                    &format!(
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
                .info
                .make_reqwest(
                    Method::Get,
                    &format!(
                        "/v2/systems/{}/waypoints/{}/jump-gate",
                        system_symbol, waypoint_symbol
                    ),
                    None,
                )
                .await,
        )
        .unwrap()
    }
    // this is trash below
    pub async fn contract_list(&self) -> Option<ContractTermsL0> {
        serde_json::from_str(
            &self
                .make_request(Broadcaster::Interface(
                    Method::Get,
                    String::from("/v2/my/contracts"),
                    None,
                ))
                .await,
        )
        .unwrap()
    }

    pub async fn contract_terms(&self, contract_id: &str) -> ContractTermsL0 {
        serde_json::from_str(
            &self
                .make_request(Broadcaster::Interface(
                    Method::Get,
                    format!("/v2/my/contracts/{}", contract_id),
                    None,
                ))
                .await,
        )
        .unwrap()
    }
}

// struct to handle the dataflow through broadcast
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Broadcaster {
    Caller(String),
    Interface(Method, String, Option<String>),
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BroadcasterMessage {
    pub method: Method,
    pub url: String,
    pub data: Option<String>,
}

// Other helpful structs and enums

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

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Method {
    Post,
    Get,
    Patch,
    // Delete,
}
