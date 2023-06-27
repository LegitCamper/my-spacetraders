pub mod enums;
pub mod requests;
pub mod responses;
mod tests;

use requests::*;
use responses::{agents, contracts, factions, fleet, systems};

use core::panic;
use random_string::generate;
use reqwest::{
    header::{HeaderValue, AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE},
    Client, //blocking::Client
};
use serde_json::json;
use std::collections::HashMap;
use tokio::{
    sync::{mpsc, oneshot},
    task,
    time::{interval, Duration},
};
use url::Url;

const LIVEURL: &str = "https://api.spacetraders.io/v2";
const MOCKURL: &str = "https://stoplight.io/mocks/spacetraders/spacetraders/96627693";

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

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum SpaceTradersEnv {
    Live,
    Mock,
}

#[derive(Debug, Clone)]
pub struct SpaceTradersInterface {
    token: String,
    pub client: Client,
    pub url: String,
    pub enviroment: SpaceTradersEnv,
}

impl SpaceTradersInterface {
    pub fn new(token: String, enviroment: SpaceTradersEnv) -> Self {
        let url = match enviroment {
            SpaceTradersEnv::Live => LIVEURL,
            SpaceTradersEnv::Mock => MOCKURL,
        };

        SpaceTradersInterface {
            token,
            client: Client::new(),
            url: String::from(url),
            enviroment,
        }
    }

    pub fn diagnose(&self) {
        panic!(
            "\ntoken: {}\nurl: {}\nenviroment: {:#?}\nclient: {:?}",
            self.token, self.url, self.enviroment, self.client
        )
    }

    fn get_url(&self, endpoint: &str) -> Url {
        Url::parse(format!("{}{}", self.url, endpoint).as_str()).unwrap()
    }

    async fn make_reqwest(&self, method: Method, url: &str, data: Option<Requests>) -> String {
        let mut client = match method {
            Method::Get => self.client.get(self.get_url(url)),
            Method::Post => self.client.post(self.get_url(url)),
            // .header(CONTENT_TYPE, "application/json"),
            Method::Patch => self.client.patch(self.get_url(url)),
        };

        client = match self.enviroment {
            SpaceTradersEnv::Live => client.header(
                AUTHORIZATION,
                HeaderValue::from_bytes(format!("Bearer {}", self.token).as_bytes()).unwrap(),
            ),
            SpaceTradersEnv::Mock => client.header("Prefer", "dynamic=true").header(
                AUTHORIZATION,
                HeaderValue::from_bytes(format!("Bearer {}", self.token).as_bytes()).unwrap(),
            ),
        };

        client = match data {
            Some(dataenum) => match dataenum {
                Requests::RegisterNewAgent(json) => client
                    .form(&json)
                    .header(CONTENT_LENGTH, std::mem::size_of_val(&json)),
                Requests::BuyShip(json) => client
                    .form(&json)
                    .header(CONTENT_LENGTH, std::mem::size_of_val(&json)),
                Requests::ShipRefine(json) => client
                    .form(&json)
                    .header(CONTENT_LENGTH, std::mem::size_of_val(&json)),
                Requests::JettisonCargo(json) => client
                    .form(&json)
                    .header(CONTENT_LENGTH, std::mem::size_of_val(&json)),
                Requests::JumpShip(json) => client
                    .form(&json)
                    .header(CONTENT_LENGTH, std::mem::size_of_val(&json)),
                Requests::NavigateShip(json) => client
                    .form(&json)
                    .header(CONTENT_LENGTH, std::mem::size_of_val(&json)),
                Requests::PatchShipNav(json) => client
                    .form(&json)
                    .header(CONTENT_LENGTH, std::mem::size_of_val(&json)),
                Requests::WarpShip(json) => client
                    .form(&json)
                    .header(CONTENT_LENGTH, std::mem::size_of_val(&json)),
                Requests::SellCargo(json) => client
                    .form(&json)
                    .header(CONTENT_LENGTH, std::mem::size_of_val(&json)),
                Requests::InstallMount(json) => client
                    .form(&json)
                    .header(CONTENT_LENGTH, std::mem::size_of_val(&json)),
                Requests::RemoveMount(json) => client
                    .form(&json)
                    .header(CONTENT_LENGTH, std::mem::size_of_val(&json)),
                Requests::DeliverCargoToContract(json) => client
                    .form(&json)
                    .header(CONTENT_LENGTH, std::mem::size_of_val(&json)),
            },
            None => client.header(CONTENT_LENGTH, "0"),
        };

        println!("{:?}", client);

        let response = client.send().await.unwrap().text().await;

        match response {
            Ok(response) => {
                // check if server error - Expects error here
                let error: Result<responses::Error, _> = serde_json::from_str(&response);

                // if error print
                if let Ok(error) = error {
                    eprintln!("error: {:?}", error);
                    panic!("{:?}", self.diagnose());
                } else {
                    println!("response error: {}", response);
                    response
                }
            }
            // client error
            Err(error) => panic!("{}", error),
        }
    }

    #[allow(dead_code)]
    async fn custom_endpoint(
        &self,
        endpoint: &str,
        method: Method,
        data: Option<Requests>,
    ) -> String {
        let client = match method {
            Method::Post => self.client.post(format!("{}{}", self.url, endpoint)),
            Method::Get => self.client.get(format!("{}{}", self.url, endpoint)),
            Method::Patch => todo!(),
        };
        let client = match data {
            Some(dataenum) => match dataenum {
                Requests::RegisterNewAgent(json) => client
                    .form(&json)
                    .header(CONTENT_LENGTH, std::mem::size_of_val(&json)),
                Requests::BuyShip(json) => client
                    .form(&json)
                    .header(CONTENT_LENGTH, std::mem::size_of_val(&json)),
                Requests::ShipRefine(json) => client
                    .form(&json)
                    .header(CONTENT_LENGTH, std::mem::size_of_val(&json)),
                Requests::JettisonCargo(json) => client
                    .form(&json)
                    .header(CONTENT_LENGTH, std::mem::size_of_val(&json)),
                Requests::JumpShip(json) => client
                    .form(&json)
                    .header(CONTENT_LENGTH, std::mem::size_of_val(&json)),
                Requests::NavigateShip(json) => client
                    .form(&json)
                    .header(CONTENT_LENGTH, std::mem::size_of_val(&json)),
                Requests::PatchShipNav(json) => client
                    .form(&json)
                    .header(CONTENT_LENGTH, std::mem::size_of_val(&json)),
                Requests::WarpShip(json) => client
                    .form(&json)
                    .header(CONTENT_LENGTH, std::mem::size_of_val(&json)),
                Requests::SellCargo(json) => client
                    .form(&json)
                    .header(CONTENT_LENGTH, std::mem::size_of_val(&json)),
                Requests::InstallMount(json) => client
                    .form(&json)
                    .header(CONTENT_LENGTH, std::mem::size_of_val(&json)),
                Requests::RemoveMount(json) => client
                    .form(&json)
                    .header(CONTENT_LENGTH, std::mem::size_of_val(&json)),
                Requests::DeliverCargoToContract(json) => client
                    .form(&json)
                    .header(CONTENT_LENGTH, std::mem::size_of_val(&json)),
            },
            None => client.header(CONTENT_LENGTH, "0"),
        };
        client.send().await.unwrap().text().await.unwrap()
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct SpaceTraders {
    interface: SpaceTradersInterface,
    channel: mpsc::Sender<ChannelMessage>,
    task: task::JoinHandle<()>,
}

impl SpaceTraders {
    pub async fn new(token: &str, enviroment: SpaceTradersEnv) -> Self {
        let space_trader = SpaceTradersInterface::new(token.to_string(), enviroment);

        let (channel_sender, mut channel_receiver) = mpsc::channel(10000);

        let mut interval = interval(Duration::from_millis(500));

        SpaceTraders {
            interface: space_trader.clone(),
            channel: channel_sender,
            task: task::spawn(async move {
                while let Some(msg) = channel_receiver.recv().await {
                    interval.tick().await; // avoids rate limiting
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

        let registration = Client::new()
            .post(&format!("{}/register", LIVEURL))
            .header(CONTENT_LENGTH, post_message.to_string().chars().count())
            .form(&post_message)
            .send()
            .await
            .unwrap()
            .json::<responses::RegisterNewAgent>()
            .await
            .unwrap();

        SpaceTraders::new(&registration.data.token, SpaceTradersEnv::Live).await
    }

    #[allow(dead_code)]
    async fn testing() -> Self {
        SpaceTraders::new("undefined", SpaceTradersEnv::Mock).await
    }

    pub fn diagnose(&self) -> String {
        format!(
            "\ntoken: {}\nurl: {}\nenviroment: {:#?}\nclient: {:?}", //, task: {:#?}, channel: {:#?}",
            self.interface.token,
            self.interface.url,
            self.interface.enviroment,
            self.interface.client,
            // self.task,
            // self.channel
        )
    }

    pub async fn make_request(
        &self,
        method: Method,
        url: String,
        data: Option<Requests>,
    ) -> String {
        let (oneshot_sender, oneshot_receiver) = oneshot::channel();

        // make request
        match self
            .channel
            .send(ChannelMessage {
                message: RequestMessage { method, url, data },
                oneshot: oneshot_sender,
            })
            .await
        {
            Err(r) => {
                eprintln!("closed: {}", r);
                panic!("{}", self.diagnose())
            }
            Ok(s) => s,
        }

        // listen to oneshot for response
        match oneshot_receiver.await {
            Ok(res) => res,
            Err(err) => {
                eprintln!("Interface failed to send back a correct response: {}", err);
                panic!("{}", self.diagnose());
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
    pub async fn deliver_contract(
        &self,
        contract_id: &str,
        data: requests::DeliverCargoToContract,
    ) -> contracts::DeliverContract {
        serde_json::from_str(
            &self
                .make_request(
                    Method::Post,
                    format!("/my/contracts/{}/deliver", contract_id),
                    Some(Requests::DeliverCargoToContract(data)),
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
                    Some(Requests::BuyShip(data)),
                )
                .await,
        )
        .unwrap()
    }
    pub async fn get_ship(&self, ship_symbol: &str) -> fleet::Ship {
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
                    Some(Requests::ShipRefine(data)),
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
    pub async fn get_ship_cooldown(&self, ship_symbol: &str) -> fleet::GetShipCooldown {
        serde_json::from_str(
            &self
                .make_request(
                    Method::Get,
                    format!("/my/ships/{}/cooldown", ship_symbol),
                    None,
                )
                .await,
        )
        .unwrap()
    }
    pub async fn dock_ship(&self, ship_symbol: &str) -> fleet::DockShip {
        serde_json::from_str(
            &self
                .make_request(
                    Method::Post,
                    format!("/my/ships/{}/dock", ship_symbol),
                    None,
                )
                .await,
        )
        .unwrap()
    }
    pub async fn create_survey(&self, ship_symbol: &str) -> fleet::CreateSurvey {
        serde_json::from_str(
            &self
                .make_request(
                    Method::Post,
                    format!("/my/ships/{}/survey", ship_symbol),
                    None,
                )
                .await,
        )
        .unwrap()
    }
    pub async fn extract_resources(&self, ship_symbol: &str) -> fleet::ExtractResources {
        serde_json::from_str(
            &self
                .make_request(
                    Method::Post,
                    format!("/my/ships/{}/extract", ship_symbol),
                    None,
                )
                .await,
        )
        .unwrap()
    }
    pub async fn jettison_cargo(&self, ship_symbol: &str) -> fleet::JettisonCargo {
        serde_json::from_str(
            &self
                .make_request(
                    Method::Post,
                    format!("/my/ships/{}/jettison", ship_symbol),
                    None,
                )
                .await,
        )
        .unwrap()
    }
    pub async fn jump_ship(&self, ship_symbol: &str, data: requests::JumpShip) -> fleet::JumpShip {
        serde_json::from_str(
            &self
                .make_request(
                    Method::Post,
                    format!("/my/ships/{}/jump", ship_symbol),
                    Some(Requests::JumpShip(data)),
                )
                .await,
        )
        .unwrap()
    }
    pub async fn navigate_ship(
        &self,
        ship_symbol: &str,
        data: requests::NavigateShip,
    ) -> fleet::NavigateShip {
        serde_json::from_str(
            &self
                .make_request(
                    Method::Post,
                    format!("/my/ships/{}/navigate", ship_symbol),
                    Some(Requests::NavigateShip(data)),
                )
                .await,
        )
        .unwrap()
    }
    pub async fn patch_ship_nav(
        &self,
        ship_symbol: &str,
        data: requests::PatchShipNav,
    ) -> fleet::PatchShipNav {
        serde_json::from_str(
            &self
                .make_request(
                    Method::Patch,
                    format!("/my/ships/{}/nav", ship_symbol),
                    Some(Requests::PatchShipNav(data)),
                )
                .await,
        )
        .unwrap()
    }
    pub async fn get_ship_nav(&self, ship_symbol: &str) -> fleet::GetShipNav {
        serde_json::from_str(
            &self
                .make_request(Method::Get, format!("/my/ships/{}/nav", ship_symbol), None)
                .await,
        )
        .unwrap()
    }
    pub async fn warp_ship(&self, ship_symbol: &str) -> fleet::WarpShip {
        serde_json::from_str(
            &self
                .make_request(
                    Method::Post,
                    format!("/my/ships/{}/warp", ship_symbol),
                    None,
                )
                .await,
        )
        .unwrap()
    }
    pub async fn sell_cargo(
        &self,
        ship_symbol: &str,
        data: requests::SellCargo,
    ) -> fleet::SellCargo {
        serde_json::from_str(
            &self
                .make_request(
                    Method::Post,
                    format!("/my/ships/{}/sell", ship_symbol),
                    Some(Requests::SellCargo(data)),
                )
                .await,
        )
        .unwrap()
    }
    pub async fn scan_systems(&self, ship_symbol: &str) -> fleet::ScanSystems {
        serde_json::from_str(
            &self
                .make_request(
                    Method::Post,
                    format!("/my/ships/{}/scan/systems", ship_symbol),
                    None,
                )
                .await,
        )
        .unwrap()
    }
    pub async fn scan_waypoints(&self, ship_symbol: &str) -> fleet::ScanWaypoints {
        serde_json::from_str(
            &self
                .make_request(
                    Method::Post,
                    format!("/my/ships/{}/scan/waypoints", ship_symbol),
                    None,
                )
                .await,
        )
        .unwrap()
    }
    pub async fn scan_ships(&self, ship_symbol: &str) -> fleet::ScanShips {
        serde_json::from_str(
            &self
                .make_request(
                    Method::Post,
                    format!("/my/ships/{}/scan/ships", ship_symbol),
                    None,
                )
                .await,
        )
        .unwrap()
    }
    pub async fn refuel_ship(&self, ship_symbol: &str) -> fleet::RefuelShip {
        serde_json::from_str(
            &self
                .make_request(
                    Method::Post,
                    format!("/my/ships/{}/refuel", ship_symbol),
                    None,
                )
                .await,
        )
        .unwrap()
    }
    pub async fn purchase_cargo(&self, ship_symbol: &str) -> fleet::PurchaseCargo {
        serde_json::from_str(
            &self
                .make_request(
                    Method::Post,
                    format!("/my/ships/{}/purchase", ship_symbol),
                    None,
                )
                .await,
        )
        .unwrap()
    }
    pub async fn transfer_cargo(&self, ship_symbol: &str) -> fleet::TransferCargo {
        serde_json::from_str(
            &self
                .make_request(
                    Method::Post,
                    format!("/my/ships/{}/transfer", ship_symbol),
                    None,
                )
                .await,
        )
        .unwrap()
    }
    pub async fn negotiate_contract(&self, ship_symbol: &str) -> fleet::NegotiateContract {
        serde_json::from_str(
            &self
                .make_request(
                    Method::Post,
                    format!("/my/ships/{}/negotiate/contract", ship_symbol),
                    None,
                )
                .await,
        )
        .unwrap()
    }
    pub async fn get_mounts(&self, ship_symbol: &str) -> fleet::GetMounts {
        serde_json::from_str(
            &self
                .make_request(
                    Method::Get,
                    format!("/my/ships/{}/negotiate/mounts", ship_symbol),
                    None,
                )
                .await,
        )
        .unwrap()
    }
    pub async fn install_mount(
        &self,
        ship_symbol: &str,
        data: requests::InstallMount,
    ) -> fleet::InstallMounts {
        serde_json::from_str(
            &self
                .make_request(
                    Method::Post,
                    format!("/my/ships/{}/negotiate/mounts/install", ship_symbol),
                    Some(Requests::InstallMount(data)),
                )
                .await,
        )
        .unwrap()
    }
    pub async fn remove_mount(
        &self,
        ship_symbol: &str,
        data: requests::RemoveMount,
    ) -> fleet::RemoveMounts {
        serde_json::from_str(
            &self
                .make_request(
                    Method::Post,
                    format!("/my/ships/{}/negotiate/mounts/remove", ship_symbol),
                    Some(Requests::RemoveMount(data)),
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

#[derive(Debug)]
pub struct RequestMessage {
    pub method: Method,
    pub url: String,
    pub data: Option<Requests>,
}
#[derive(Debug)]
pub struct ChannelMessage {
    message: RequestMessage,
    oneshot: oneshot::Sender<String>,
}

// Other helpful functions
// #[derive(Debug)]
// enum WaypointKind {
//     Sector(String, String),
//     System(String),
//     Waypoint(String, String, String),
// }

pub fn parse_waypoint(waypoint: &str) -> Waypoint {
    let waypoint_split: Vec<&str> = waypoint.split('-').collect();
    Waypoint {
        sector: waypoint_split[0].to_string(), // X1
        system: format!("{}-{}", waypoint_split[0], waypoint_split[1]), // X1-DF55
        waypoint: waypoint.to_string(),        // X1-DF55-20250Z
    }
} // TODO
#[derive(Debug)]
pub struct Waypoint {
    pub sector: String,
    pub system: String,
    pub waypoint: String,
}
impl<'de> serde::Deserialize<'de> for Waypoint {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = String::deserialize(d)?;
        Ok(parse_waypoint(&s))
    }
}
