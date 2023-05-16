pub mod lib;

use lib::*;

use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE},
    Client,
};
use serde::{Deserialize, Serialize};
use tokio::{
    sync::{mpsc, oneshot},
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
    channel: mpsc::Sender<ChannelMessage>,
    task: task::JoinHandle<()>,
}

impl SpaceTradersHandler {
    pub async fn new(credentials: Credentials) -> Self {
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

    pub async fn make_request(&self, message: RequestMessage) -> String {
        // make oneshot channel
        let (oneshot_sender, oneshot_receiver) = oneshot::channel();

        // make request
        self.channel
            .send(ChannelMessage {
                message,
                oneshot: oneshot_sender,
            })
            .await
            .unwrap(); //.await.unwrap();

        // listen to oneshot for response
        let Ok(res) = oneshot_receiver.await else { panic!("Reponse was bad!")
        };
        res
    }

    pub async fn agent(&self) -> Option<AgentL0> {
        serde_json::from_str(
            &self
                .make_request(RequestMessage {
                    method: Method::Get,
                    url: "/v2/my/agent".to_string(),
                    data: None,
                })
                .await,
        )
        .unwrap()
    }

    pub async fn list_systems(&self) -> ListSystemsL0 {
        serde_json::from_str(
            &self
                .make_request(RequestMessage {
                    method: Method::Get,
                    url: "/v2/systems".to_string(),
                    data: None,
                })
                .await,
        )
        .unwrap()
    }
    pub async fn get_system(&self, system_symbol: &str) -> GetSystemL0 {
        serde_json::from_str(
            &self
                .make_request(RequestMessage {
                    method: Method::Get,
                    url: format!("/v2/systems/{}", system_symbol),
                    data: None,
                })
                .await,
        )
        .unwrap()
    }
    pub async fn list_waypoints(&self, system_symbol: &str) -> ListWaypointsL0 {
        serde_json::from_str(
            &self
                .make_reqwest(RequestMessage {
                    method: Method::Get,
                    url: format!("/v2/systems/{}/waypoints", system_symbol),
                    data: None,
                })
                .await,
        )
        .unwrap()
    }
    pub async fn get_waypoint(&self, system_symbol: &str, waypoint_symbol: &str) -> GetWaypointL0 {
        serde_json::from_str(
            &self
                .make_reqwest(RequestMessage {
                    method: Method::Get,
                    url: format!(
                        "/v2/systems/{}/waypoints/{}",
                        system_symbol, waypoint_symbol
                    ),
                    data: None,
                })
                .await,
        )
        .unwrap()
    }
    pub async fn get_market(&self, system_symbol: &str, waypoint_symbol: &str) -> GetMarketL0 {
        serde_json::from_str(
            &self
                .make_reqwest(RequestMessage {
                    method: Method::Get,
                    url: format!(
                        "/v2/systems/{}/waypoints/{}/market",
                        system_symbol, waypoint_symbol
                    ),
                    data: None,
                })
                .await,
        )
        .unwrap()
    }
    pub async fn get_shipyard(&self, system_symbol: &str, waypoint_symbol: &str) {
        serde_json::from_str(
            &self
                .make_reqwest(RequestMessage {
                    method: Method::Get,
                    url: format!(
                        "/v2/systems/{}/waypoints/{}/shipyard",
                        system_symbol, waypoint_symbol
                    ),
                    data: None,
                })
                .await,
        )
        .unwrap()
    }
    pub async fn jump_gate(&self, system_symbol: &str, waypoint_symbol: &str) {
        serde_json::from_str(
            &self
                .make_reqwest(RequestMessage {
                    method: Method::Get,
                    url: format!(
                        "/v2/systems/{}/waypoints/{}/jump-gate",
                        system_symbol, waypoint_symbol
                    ),
                    data: None,
                })
                .await,
        )
        .unwrap()
    }
    // this is trash below
    pub async fn contract_list(&self) -> Option<ContractTermsL0> {
        serde_json::from_str(
            &self
                .make_request(RequestMessage {
                    method: Method::Get,
                    url: String::from("/v2/my/contracts"),
                    data: None,
                })
                .await,
        )
        .unwrap()
    }

    pub async fn contract_terms(&self, contract_id: &str) -> ContractTermsL0 {
        serde_json::from_str(
            &&self
                .make_request(RequestMessage {
                    method: Method::Get,
                    url: format!("/v2/my/contracts/{}", contract_id),
                    data: None,
                })
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
