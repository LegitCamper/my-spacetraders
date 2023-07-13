pub mod enums;
pub mod requests;
pub mod responses;
mod tests;

use requests::{
    DeliverCargoToContract, ExtractResources, InstallMount, JettisonCargo, JumpShip, NavigateShip,
    PatchShipNav, PurchaseCargo, PurchaseShip, RegisterNewAgent, RemoveMount, Requests, SellCargo,
    ShipRefine, TransferCargo, WarpShip,
};
use responses::{
    GetStatus, {agents, contracts, factions, fleet, systems},
};

use async_recursion::async_recursion;
use core::panic;
use log::error;
use rand::Rng;
use random_string::generate;
use reqwest::{
    header::{HeaderValue, AUTHORIZATION, CONTENT_LENGTH},
    Client,
};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_retry::{policies::ExponentialBackoff, RetryTransientMiddleware};
use serde::{
    de::{Error as OtherError, Unexpected},
    Deserialize, Deserializer, Serialize,
};
use std::{
    collections::HashMap,
    sync::{atomic::AtomicBool, Arc},
};
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
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum SpaceTradersEnv {
    Live,
    Mock,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SpaceTradersInterface {
    token: String,
    pub client: ClientWithMiddleware,
    pub url: String,
    pub enviroment: SpaceTradersEnv,
    has_errored: Arc<AtomicBool>,
}

impl SpaceTradersInterface {
    pub fn new(token: String, enviroment: SpaceTradersEnv) -> Self {
        let url = match enviroment {
            SpaceTradersEnv::Live => LIVEURL,
            SpaceTradersEnv::Mock => MOCKURL,
        };

        let retry_policy = ExponentialBackoff::builder().build_with_max_retries(3);

        SpaceTradersInterface {
            token,
            client: ClientBuilder::new(reqwest::Client::new())
                .with(RetryTransientMiddleware::new_with_policy(retry_policy))
                .build(),
            url: String::from(url),
            enviroment,
            has_errored: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn diagnose(&self) {
        panic!(
            "\ntoken: {}\nurl: {}\nenviroment: {:#?}\nclient: {:?}",
            self.token, self.url, self.enviroment, self.client
        );
    }

    fn get_url(&self, endpoint: &str) -> Url {
        Url::parse(format!("{}{}", self.url, endpoint).as_str()).unwrap()
    }

    #[async_recursion]
    async fn make_reqwest(&self, method: Method, url: &str, data: Option<Requests>) -> String {
        let mut client = match method {
            Method::Get => self.client.get(self.get_url(url)),
            Method::Post => self.client.post(self.get_url(url)),
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
                Requests::RegisterNewAgent(json) => client.json(&json),
                Requests::PurchaseShip(json) => client.json(&json),
                Requests::ExtractResources(json) => client.json(&json),
                Requests::ShipRefine(json) => client.json(&json),
                Requests::JettisonCargo(json) => client.json(&json),
                Requests::JumpShip(json) => client.json(&json),
                Requests::NavigateShip(json) => client.json(&json),
                Requests::PatchShipNav(json) => client.json(&json),
                Requests::WarpShip(json) => client.json(&json),
                Requests::SellCargo(json) => client.json(&json),
                Requests::PurchaseCargo(json) => client.json(&json),
                Requests::RefuelShip(json) => client.json(&json),
                Requests::TransferCargo(json) => client.json(&json),
                Requests::InstallMount(json) => client.json(&json),
                Requests::RemoveMount(json) => client.json(&json),
                Requests::DeliverCargoToContract(json) => client.json(&json),
            },
            None => client.header(CONTENT_LENGTH, "0"),
        };

        let response = client.send().await.unwrap();
        if response.status().is_success() {
            response.text().await.unwrap()
        // } else if response.status().as_u16() == 429 {
        // let time_to_sleep: error_429 = response.json().await.unwrap();
        // self.make_reqwest(method, url, data).await
        } else {
            panic!(
                "status: {:?}, error: {}",
                response.status(),
                response.text().await.unwrap()
            )
        }
    }

    #[allow(dead_code)]
    async fn custom_endpoint(
        &self,
        method: Method,
        endpoint: &str,
        data: Option<Requests>,
    ) -> String {
        self.make_reqwest(method, endpoint, data).await
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct SpaceTraders {
    interface: SpaceTradersInterface,
    channel: mpsc::Sender<ChannelMessage>,
    pub task: task::JoinHandle<()>,
}

impl SpaceTraders {
    pub async fn new(token: &str, enviroment: SpaceTradersEnv) -> Self {
        let space_trader = SpaceTradersInterface::new(token.to_string(), enviroment);

        let (channel_sender, mut channel_receiver) = mpsc::channel(120);

        let mut interval = interval(Duration::from_millis(500));

        SpaceTraders {
            interface: space_trader.clone(),
            channel: channel_sender,
            task: task::spawn(async move {
                interval.tick().await; // inits tick
                while let Some(msg) = channel_receiver.recv().await {
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
                    interval.tick().await; // avoids rate limiting - waits 500 millis
                }
            }),
        }
    }

    pub async fn default() -> Self {
        let mut rng = rand::thread_rng();
        let username = generate(14, "abcdefghijklmnopqrstuvwxyz1234567890_");
        let post_message = RegisterNewAgent {
            faction: rng.gen::<enums::FactionSymbols>(),
            symbol: username,
            email: None,
        };

        match Client::new()
            .post(&format!("{}/register", LIVEURL))
            .json(&post_message)
            .send()
            .await
            .unwrap()
            .json::<responses::RegisterNewAgent>()
            .await
        {
            Ok(registration) => {
                SpaceTraders::new(&registration.data.token, SpaceTradersEnv::Live).await
            }
            Err(error) => panic!("{}\nTransitive error occured - please re-run", error),
        }
    }

    #[allow(dead_code)]
    async fn testing() -> Self {
        SpaceTraders::new("undefined", SpaceTradersEnv::Mock).await
    }

    pub fn diagnose(&self) -> String {
        format!(
            "\ntoken: {}\nurl: {}\nenviroment: {:#?}\nclient: {:?}",
            self.interface.token,
            self.interface.url,
            self.interface.enviroment,
            self.interface.client,
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
                error!("closed: {}", r);
            }
            Ok(s) => s,
        }

        // listen to oneshot for response
        match oneshot_receiver.await {
            Ok(res) => res,
            Err(err) => {
                // TODO: implement error check to make sure I dont error 100 times
                error!("Interface failed to send back a correct response: {}", err);
                panic!("{}", self.diagnose());
            }
        }
    }

    // Status
    pub async fn get_status(&self) -> GetStatus {
        serde_json::from_str(&self.make_request(Method::Get, "".to_string(), None).await).unwrap()
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
    pub async fn list_systems(&self, page: Option<u32>) -> systems::Systems {
        let page_num = page.unwrap_or(1);
        serde_json::from_str(
            &self
                .make_request(
                    Method::Get,
                    format!("/systems?limit=20&page={}", page_num),
                    None,
                )
                .await,
        )
        .unwrap()
    }
    pub async fn get_system(&self, system_symbol: System) -> systems::System {
        serde_json::from_str(
            &self
                .make_request(
                    Method::Get,
                    format!("/systems/{}", system_symbol.system),
                    None,
                )
                .await,
        )
        .unwrap()
    }
    pub async fn list_waypoints(
        &self,
        system_symbol: System,
        page: Option<u32>,
    ) -> systems::Waypoints {
        let page_num = page.unwrap_or(1);
        serde_json::from_str(
            &self
                .make_request(
                    Method::Get,
                    format!(
                        "/systems/{}/waypoints?limit=20&page={}",
                        system_symbol.system, page_num
                    ),
                    None,
                )
                .await,
        )
        .unwrap()
    }
    pub async fn get_waypoint(
        &self,
        system_symbol: System,
        waypoint_symbol: Waypoint,
    ) -> systems::Waypoint {
        serde_json::from_str(
            &self
                .make_request(
                    Method::Get,
                    format!(
                        "/systems/{}/waypoints/{}",
                        system_symbol.system, waypoint_symbol.waypoint
                    ),
                    None,
                )
                .await,
        )
        .unwrap()
    }
    pub async fn get_market(
        &self,
        system_symbol: System,
        waypoint_symbol: Waypoint,
    ) -> systems::Market {
        serde_json::from_str(
            &self
                .make_request(
                    Method::Get,
                    format!(
                        "/systems/{}/waypoints/{}/market",
                        system_symbol.system, waypoint_symbol.waypoint
                    ),
                    None,
                )
                .await,
        )
        .unwrap()
    }
    pub async fn get_shipyard(
        &self,
        system_symbol: System,
        waypoint_symbol: Waypoint,
    ) -> systems::Shipyard {
        serde_json::from_str(
            &self
                .make_request(
                    Method::Get,
                    format!(
                        "/systems/{}/waypoints/{}/shipyard",
                        system_symbol.system, waypoint_symbol.waypoint
                    ),
                    None,
                )
                .await,
        )
        .unwrap()
    }
    pub async fn jump_gate(
        &self,
        system_symbol: System,
        waypoint_symbol: Waypoint,
    ) -> systems::JumpGate {
        serde_json::from_str(
            &self
                .make_request(
                    Method::Get,
                    format!(
                        "/systems/{}/waypoints/{}/jump-gate",
                        system_symbol.system, waypoint_symbol.waypoint
                    ),
                    None,
                )
                .await,
        )
        .unwrap()
    }

    // Contracts
    pub async fn list_contracts(&self, page_num: Option<u32>) -> contracts::Contracts {
        let page_num = page_num.unwrap_or(1);
        serde_json::from_str(
            &self
                .make_request(
                    Method::Get,
                    format!("/my/contracts?limit=20&page={}", page_num),
                    None,
                )
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
        data: DeliverCargoToContract,
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
    pub async fn purchase_ship(&self, data: PurchaseShip) -> fleet::PurchaseShip {
        serde_json::from_str(
            &self
                .make_request(
                    Method::Post,
                    String::from("/my/ships"),
                    Some(Requests::PurchaseShip(data)),
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
                .make_request(
                    Method::Get,
                    format!("/my/ships/{}/cargo", ship_symbol),
                    None,
                )
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
    pub async fn extract_resources(
        &self,
        ship_symbol: &str,
        data: Option<ExtractResources>,
    ) -> fleet::ExtractResources {
        match data {
            Some(data) => serde_json::from_str(
                &self
                    .make_request(
                        Method::Post,
                        format!("/my/ships/{}/extract", ship_symbol),
                        Some(Requests::ExtractResources(data)),
                    )
                    .await,
            )
            .unwrap(),
            None => serde_json::from_str(
                &self
                    .make_request(
                        Method::Post,
                        format!("/my/ships/{}/extract", ship_symbol),
                        None,
                    )
                    .await,
            )
            .unwrap(),
        }
    }
    pub async fn jettison_cargo(
        &self,
        ship_symbol: &str,
        data: JettisonCargo,
    ) -> fleet::JettisonCargo {
        serde_json::from_str(
            &self
                .make_request(
                    Method::Post,
                    format!("/my/ships/{}/jettison", ship_symbol),
                    Some(Requests::JettisonCargo(data)),
                )
                .await,
        )
        .unwrap()
    }
    pub async fn jump_ship(&self, ship_symbol: &str, data: JumpShip) -> fleet::JumpShip {
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
        data: NavigateShip,
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
        data: PatchShipNav,
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
    pub async fn warp_ship(&self, ship_symbol: &str, data: WarpShip) -> fleet::WarpShip {
        serde_json::from_str(
            &self
                .make_request(
                    Method::Post,
                    format!("/my/ships/{}/warp", ship_symbol),
                    Some(Requests::WarpShip(data)),
                )
                .await,
        )
        .unwrap()
    }
    pub async fn sell_cargo(&self, ship_symbol: &str, data: SellCargo) -> fleet::SellCargo {
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
    pub async fn refuel_ship(
        &self,
        ship_symbol: &str,
        fuel_amount: Option<requests::RefuelShip>,
    ) -> fleet::RefuelShip {
        let fuel_amount = fuel_amount.unwrap_or(requests::RefuelShip { units: 1 });
        serde_json::from_str(
            &self
                .make_request(
                    Method::Post,
                    format!("/my/ships/{}/refuel", ship_symbol),
                    Some(Requests::RefuelShip(fuel_amount)),
                )
                .await,
        )
        .unwrap()
    }
    pub async fn purchase_cargo(
        &self,
        ship_symbol: &str,
        data: PurchaseCargo,
    ) -> fleet::PurchaseCargo {
        serde_json::from_str(
            &self
                .make_request(
                    Method::Post,
                    format!("/my/ships/{}/purchase", ship_symbol),
                    Some(Requests::PurchaseCargo(data)),
                )
                .await,
        )
        .unwrap()
    }
    pub async fn transfer_cargo(
        &self,
        ship_symbol: &str,
        data: TransferCargo,
    ) -> fleet::TransferCargo {
        serde_json::from_str(
            &self
                .make_request(
                    Method::Post,
                    format!("/my/ships/{}/transfer", ship_symbol),
                    Some(Requests::TransferCargo(data)),
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
                    format!("/my/ships/{}/mounts", ship_symbol),
                    None,
                )
                .await,
        )
        .unwrap()
    }
    pub async fn install_mount(
        &self,
        ship_symbol: &str,
        data: InstallMount,
    ) -> fleet::InstallMounts {
        serde_json::from_str(
            &self
                .make_request(
                    Method::Post,
                    format!("/my/ships/{}/mounts/install", ship_symbol),
                    Some(Requests::InstallMount(data)),
                )
                .await,
        )
        .unwrap()
    }
    pub async fn remove_mount(&self, ship_symbol: &str, data: RemoveMount) -> fleet::RemoveMounts {
        serde_json::from_str(
            &self
                .make_request(
                    Method::Post,
                    format!("/my/ships/{}/mounts/remove", ship_symbol),
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
    pub async fn get_faction(&self, faction_symbol: &str) -> factions::Faction {
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

// Waypoint handlers //
#[derive(Clone, Serialize, PartialEq, Eq, Hash, Debug)]
pub struct Waypoint {
    pub waypoint: String,
    pub system: String,
    pub sector: String,
}
impl Waypoint {
    pub fn to_waypoint(&self) -> Self {
        Self {
            waypoint: self.waypoint.clone(),
            system: self.system.clone(),
            sector: self.sector.clone(),
        }
    }
    pub fn to_system(&self) -> System {
        System {
            system: self.system.clone(),
            sector: self.sector.clone(),
        }
    }
    pub fn to_sector(&self) -> Sector {
        Sector {
            sector: self.sector.clone(),
        }
    }
}
impl<'de> Deserialize<'de> for Waypoint {
    fn deserialize<D>(deserializer: D) -> Result<Waypoint, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(deserializer)?;
        if s.contains('-') {
            let waypoint_split: Vec<&str> = s.split('-').collect();
            if waypoint_split.len() == 3 {
                Ok(Waypoint {
                    waypoint: s.to_string(),
                    system: format!("{}-{}", waypoint_split[0], waypoint_split[1]),
                    sector: waypoint_split[0].to_string(),
                })
            } else {
                Err(D::Error::invalid_value(
                    Unexpected::Str(s),
                    &"a String as Waypoint",
                ))
            }
        } else {
            Ok(Waypoint {
                waypoint: "None".to_string(),
                system: "None".to_string(),
                sector: "None".to_string(),
            })
        }
    }
}
#[derive(Clone, Serialize, PartialEq, Eq, Hash, Debug)]
pub struct System {
    pub system: String,
    pub sector: String,
}
impl System {
    pub fn to_system(&self) -> Self {
        Self {
            system: self.system.clone(),
            sector: self.sector.clone(),
        }
    }
    pub fn to_sector(&self) -> Sector {
        Sector {
            sector: self.sector.clone(),
        }
    }
}
impl<'de> Deserialize<'de> for System {
    fn deserialize<D>(deserializer: D) -> Result<System, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(deserializer)?;
        if s.contains('-') {
            let split: Vec<&str> = s.split('-').collect();
            if split.len() == 2 {
                Ok(System {
                    sector: split[0].to_string(),
                    system: format!("{}-{}", split[0], split[1]),
                })
            } else {
                Err(D::Error::invalid_value(
                    Unexpected::Str(s),
                    &"a String as System",
                ))
            }
        } else {
            Ok(System {
                system: "None".to_string(),
                sector: "None".to_string(),
            })
        }
    }
}
#[derive(Clone, Serialize, PartialEq, Eq, Hash, Debug)]
pub struct Sector {
    pub sector: String,
}
#[allow(dead_code)]
impl Sector {
    fn to_sector(&self) -> Self {
        Self {
            sector: self.sector.clone(),
        }
    }
}
impl<'de> Deserialize<'de> for Sector {
    fn deserialize<D>(deserializer: D) -> Result<Sector, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(deserializer)?;
        if s.contains('-') {
            let split: Vec<&str> = s.split('-').collect();
            if split.len() == 1 {
                Ok(Sector {
                    sector: split[0].to_string(),
                })
            } else {
                Err(D::Error::invalid_value(
                    Unexpected::Str(s),
                    &"a String as Sector",
                ))
            }
        } else {
            Ok(Sector {
                sector: "None".to_string(),
            })
        }
    }
}

pub mod spacetraders_datetime_format {
    use chrono::{DateTime, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &'static str = "%+";

    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Utc.datetime_from_str(&s, FORMAT)
            .map_err(serde::de::Error::custom)
    }
}
pub mod spacetraders_date_format {
    use chrono::{DateTime, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &'static str = "%+";

    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let s = format!("{}{}", s, "T01:00:00Z");
        Utc.datetime_from_str(&s, FORMAT)
            .map_err(serde::de::Error::custom)
    }
}
// pub mod spacetraders_time_format {
//     use chrono::{DateTime, TimeZone, Utc};
//     use serde::{self, Deserialize, Deserializer, Serializer};

// const FORMAT: &'static str = "%Y-%m-%dT%I:%M:%S%.3fZ";

//     pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         let s = format!("{}", date.format(FORMAT));
//         serializer.serialize_str(&s)
//     }

//     pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         let s = String::deserialize(deserializer)?;
//         Utc.datetime_from_str(&s, FORMAT)
//             .map_err(serde::de::Error::custom)
//     }
// }
