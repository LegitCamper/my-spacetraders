pub mod enums;
pub mod requests;
pub mod responses;
mod tests;

use requests::{
    DeliverCargoToContract, InstallMount, JettisonCargo, JumpShip, NavigateShip, PatchShipNav,
    PurchaseCargo, PurchaseShip, RegisterNewAgent, RemoveMount, Requests, SellCargo, ShipRefine,
    TransferCargo, WarpShip,
};
use responses::{
    schemas, GetStatus, {agents, contracts, factions, fleet, systems},
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
            "\nurl: {}\nenviroment: {:#?}\ntoken: {}",
            self.url, self.enviroment, self.token,
        );
    }

    fn get_url(&self, endpoint: &str) -> Url {
        Url::parse(format!("{}{}", self.url, endpoint).as_str()).unwrap()
    }

    // #[async_recursion]
    async fn make_reqwest(
        &self,
        method: Method,
        url: &str,
        data: Option<Requests>,
    ) -> Option<String> {
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

        let response = client.send().await;
        let response = match response {
            Err(msg) => {
                error!("{}", msg);
                None
            }
            Ok(msg) => Some(msg),
        }?
        .text()
        .await;
        match response {
            Err(msg) => {
                error!("{}", msg);
                None
            }
            Ok(msg) => Some(msg),
        }
    }

    #[allow(dead_code)]
    async fn custom_endpoint(
        &self,
        method: Method,
        endpoint: &str,
        data: Option<Requests>,
    ) -> Option<String> {
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

    // TODO: find most efficient starting faction
    #[async_recursion]
    #[allow(clippy::should_implement_trait)]
    pub async fn default() -> Self {
        let username = generate(14, "abcdefghijklmnopqrstuvwxyz1234567890_");
        let post_message = RegisterNewAgent {
            faction: rand::thread_rng().gen::<enums::FactionSymbols>(),
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
            Err(_) => {
                error!("Transitive error occured creating new agent. Trying again");
                SpaceTraders::default().await
            }
        }
    }

    #[allow(dead_code)]
    async fn testing() -> Self {
        SpaceTraders::new("undefined", SpaceTradersEnv::Mock).await
    }

    pub fn diagnose(&self) -> String {
        format!(
            "\nurl: {}\nenviroment: {:#?}\ntoken: {}",
            self.interface.url, self.interface.enviroment, self.interface.token,
        )
    }

    pub async fn make_request(
        &self,
        method: Method,
        url: String,
        data: Option<Requests>,
    ) -> Option<String> {
        let (oneshot_sender, oneshot_receiver) = oneshot::channel();

        // make request
        self.channel
            .send(ChannelMessage {
                message: RequestMessage { method, url, data },
                oneshot: oneshot_sender,
            })
            .await
            .unwrap();

        match oneshot_receiver.await {
            Err(error) => {
                error!("{}", error);
                None
            }
            Ok(data) => Some(data?),
        }
    }

    // Status
    pub async fn get_status(&self) -> Option<GetStatus> {
        serde_json::from_str(&self.make_request(Method::Get, "".to_string(), None).await?).ok()
    }

    // Agents
    pub async fn agent(&self) -> Option<agents::Agent> {
        serde_json::from_str(
            &self
                .make_request(Method::Get, "/my/agent".to_string(), None)
                .await?,
        )
        .ok()
    }

    // Systems
    pub async fn list_systems(&self, page: Option<u32>) -> Option<systems::Systems> {
        let page_num = page.unwrap_or(1);
        serde_json::from_str(
            &self
                .make_request(
                    Method::Get,
                    format!("/systems?limit=20&page={}", page_num),
                    None,
                )
                .await?,
        )
        .ok()
    }
    pub async fn get_system(&self, system_symbol: &SystemString) -> Option<systems::System> {
        serde_json::from_str(
            &self
                .make_request(
                    Method::Get,
                    format!("/systems/{}", system_symbol.system),
                    None,
                )
                .await?,
        )
        .ok()
    }
    pub async fn list_waypoints(
        &self,
        system_symbol: &SystemString,
        page: Option<u32>,
    ) -> Option<systems::Waypoints> {
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
                .await?,
        )
        .ok()
    }
    pub async fn get_waypoint(
        &self,
        system_symbol: &SystemString,
        waypoint_symbol: &WaypointString,
    ) -> Option<systems::Waypoint> {
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
                .await?,
        )
        .ok()
    }
    pub async fn get_market(
        &self,
        system_symbol: &SystemString,
        waypoint_symbol: &WaypointString,
    ) -> Option<systems::Market> {
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
                .await?,
        )
        .ok()
    }
    pub async fn get_shipyard(
        &self,
        system_symbol: &SystemString,
        waypoint_symbol: &WaypointString,
    ) -> Option<systems::Shipyard> {
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
                .await?,
        )
        .ok()
    }
    pub async fn jump_gate(
        &self,
        system_symbol: &SystemString,
        waypoint_symbol: &WaypointString,
    ) -> Option<systems::JumpGate> {
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
                .await?,
        )
        .ok()
    }

    // Contracts
    pub async fn list_contracts(&self, page_num: Option<u32>) -> Option<contracts::Contracts> {
        let page_num = page_num.unwrap_or(1);
        serde_json::from_str(
            &self
                .make_request(
                    Method::Get,
                    format!("/my/contracts?limit=20&page={}", page_num),
                    None,
                )
                .await?,
        )
        .ok()
    }
    pub async fn get_contract(&self, contract_id: &str) -> Option<contracts::Contract> {
        serde_json::from_str(
            &self
                .make_request(Method::Get, format!("/my/contracts/{}", contract_id), None)
                .await?,
        )
        .ok()
    }
    pub async fn accept_contract(&self, contract_id: &str) -> Option<contracts::AcceptContract> {
        serde_json::from_str(
            &self
                .make_request(
                    Method::Post,
                    format!("/my/contracts/{}/accept", contract_id),
                    None,
                )
                .await?,
        )
        .ok()
    }
    pub async fn deliver_contract(
        &self,
        contract_id: &str,
        data: DeliverCargoToContract,
    ) -> Option<contracts::DeliverContract> {
        serde_json::from_str(
            &self
                .make_request(
                    Method::Post,
                    format!("/my/contracts/{}/deliver", contract_id),
                    Some(Requests::DeliverCargoToContract(data)),
                )
                .await?,
        )
        .ok()
    }
    pub async fn fulfill_contract(&self, contract_id: &str) -> Option<contracts::FulfillContract> {
        serde_json::from_str(
            &self
                .make_request(
                    Method::Post,
                    format!("/my/contracts/{}/fulfill", contract_id),
                    None,
                )
                .await?,
        )
        .ok()
    }

    // Fleet
    pub async fn list_ships(&self) -> Option<fleet::Ships> {
        serde_json::from_str(
            &self
                .make_request(Method::Get, String::from("/my/ships"), None)
                .await?,
        )
        .ok()
    }
    pub async fn purchase_ship(&self, data: PurchaseShip) -> Option<fleet::PurchaseShip> {
        serde_json::from_str(
            &self
                .make_request(
                    Method::Post,
                    String::from("/my/ships"),
                    Some(Requests::PurchaseShip(data)),
                )
                .await?,
        )
        .ok()
    }
    pub async fn get_ship(&self, ship_symbol: &str) -> Option<fleet::Ship> {
        serde_json::from_str(
            &self
                .make_request(Method::Get, format!("/my/ships/{}", ship_symbol), None)
                .await?,
        )
        .ok()
    }
    pub async fn get_ship_cargo(&self, ship_symbol: &str) -> Option<fleet::ShipCargo> {
        serde_json::from_str(
            &self
                .make_request(
                    Method::Get,
                    format!("/my/ships/{}/cargo", ship_symbol),
                    None,
                )
                .await?,
        )
        .ok()
    }
    pub async fn orbit_ship(&self, ship_symbol: &str) -> Option<fleet::OrbitShip> {
        serde_json::from_str(
            &self
                .make_request(
                    Method::Post,
                    format!("/my/ships/{}/orbit", ship_symbol),
                    None,
                )
                .await?,
        )
        .ok()
    }
    pub async fn ship_refine(
        &self,
        ship_symbol: &str,
        data: ShipRefine,
    ) -> Option<fleet::ShipRefine> {
        serde_json::from_str(
            &self
                .make_request(
                    Method::Post,
                    format!("/my/ships/{}/refine", ship_symbol),
                    Some(Requests::ShipRefine(data)),
                )
                .await?,
        )
        .ok()
    }
    pub async fn create_chart(&self, ship_symbol: &str) -> Option<fleet::CreateChart> {
        serde_json::from_str(
            &self
                .make_request(
                    Method::Post,
                    format!("/my/ships/{}/chart", ship_symbol),
                    None,
                )
                .await?,
        )
        .ok()
    }
    pub async fn get_ship_cooldown(&self, ship_symbol: &str) -> Option<fleet::GetShipCooldown> {
        serde_json::from_str(
            &self
                .make_request(
                    Method::Get,
                    format!("/my/ships/{}/cooldown", ship_symbol),
                    None,
                )
                .await?,
        )
        .ok()
    }
    pub async fn dock_ship(&self, ship_symbol: &str) -> Option<fleet::DockShip> {
        serde_json::from_str(
            &self
                .make_request(
                    Method::Post,
                    format!("/my/ships/{}/dock", ship_symbol),
                    None,
                )
                .await?,
        )
        .ok()
    }
    pub async fn create_survey(&self, ship_symbol: &str) -> Option<fleet::CreateSurvey> {
        serde_json::from_str(
            &self
                .make_request(
                    Method::Post,
                    format!("/my/ships/{}/survey", ship_symbol),
                    None,
                )
                .await?,
        )
        .ok()
    }
    pub async fn extract_resources(
        &self,
        ship_symbol: &str,
        data: Option<schemas::Survey>,
    ) -> Option<fleet::ExtractResources> {
        match data {
            Some(data) => serde_json::from_str(
                &self
                    .make_request(
                        Method::Post,
                        format!("/my/ships/{}/extract", ship_symbol),
                        Some(Requests::ExtractResources(data)),
                    )
                    .await?,
            )
            .ok(),
            None => serde_json::from_str(
                &self
                    .make_request(
                        Method::Post,
                        format!("/my/ships/{}/extract", ship_symbol),
                        None,
                    )
                    .await?,
            )
            .ok(),
        }
    }
    pub async fn jettison_cargo(
        &self,
        ship_symbol: &str,
        data: JettisonCargo,
    ) -> Option<fleet::JettisonCargo> {
        serde_json::from_str(
            &self
                .make_request(
                    Method::Post,
                    format!("/my/ships/{}/jettison", ship_symbol),
                    Some(Requests::JettisonCargo(data)),
                )
                .await?,
        )
        .ok()
    }
    pub async fn jump_ship(&self, ship_symbol: &str, data: JumpShip) -> Option<fleet::JumpShip> {
        serde_json::from_str(
            &self
                .make_request(
                    Method::Post,
                    format!("/my/ships/{}/jump", ship_symbol),
                    Some(Requests::JumpShip(data)),
                )
                .await?,
        )
        .ok()
    }
    pub async fn navigate_ship(
        &self,
        ship_symbol: &str,
        data: NavigateShip,
    ) -> Option<fleet::NavigateShip> {
        serde_json::from_str(
            &self
                .make_request(
                    Method::Post,
                    format!("/my/ships/{}/navigate", ship_symbol),
                    Some(Requests::NavigateShip(data)),
                )
                .await?,
        )
        .ok()
    }
    pub async fn patch_ship_nav(
        &self,
        ship_symbol: &str,
        data: PatchShipNav,
    ) -> Option<fleet::PatchShipNav> {
        serde_json::from_str(
            &self
                .make_request(
                    Method::Patch,
                    format!("/my/ships/{}/nav", ship_symbol),
                    Some(Requests::PatchShipNav(data)),
                )
                .await?,
        )
        .ok()
    }
    pub async fn get_ship_nav(&self, ship_symbol: &str) -> Option<fleet::GetShipNav> {
        serde_json::from_str(
            &self
                .make_request(Method::Get, format!("/my/ships/{}/nav", ship_symbol), None)
                .await?,
        )
        .ok()
    }
    pub async fn warp_ship(&self, ship_symbol: &str, data: WarpShip) -> Option<fleet::WarpShip> {
        serde_json::from_str(
            &self
                .make_request(
                    Method::Post,
                    format!("/my/ships/{}/warp", ship_symbol),
                    Some(Requests::WarpShip(data)),
                )
                .await?,
        )
        .ok()
    }
    pub async fn sell_cargo(&self, ship_symbol: &str, data: SellCargo) -> Option<fleet::SellCargo> {
        serde_json::from_str(
            &self
                .make_request(
                    Method::Post,
                    format!("/my/ships/{}/sell", ship_symbol),
                    Some(Requests::SellCargo(data)),
                )
                .await?,
        )
        .ok()
    }
    pub async fn scan_systems(&self, ship_symbol: &str) -> Option<fleet::ScanSystems> {
        serde_json::from_str(
            &self
                .make_request(
                    Method::Post,
                    format!("/my/ships/{}/scan/systems", ship_symbol),
                    None,
                )
                .await?,
        )
        .ok()
    }
    pub async fn scan_waypoints(&self, ship_symbol: &str) -> Option<fleet::ScanWaypoints> {
        serde_json::from_str(
            &self
                .make_request(
                    Method::Post,
                    format!("/my/ships/{}/scan/waypoints", ship_symbol),
                    None,
                )
                .await?,
        )
        .ok()
    }
    pub async fn scan_ships(&self, ship_symbol: &str) -> Option<fleet::ScanShips> {
        serde_json::from_str(
            &self
                .make_request(
                    Method::Post,
                    format!("/my/ships/{}/scan/ships", ship_symbol),
                    None,
                )
                .await?,
        )
        .ok()
    }
    pub async fn refuel_ship(
        &self,
        ship_symbol: &str,
        fuel_amount: Option<requests::RefuelShip>,
    ) -> Option<fleet::RefuelShip> {
        let fuel_amount = fuel_amount.unwrap_or(requests::RefuelShip { units: 1 });
        serde_json::from_str(
            &self
                .make_request(
                    Method::Post,
                    format!("/my/ships/{}/refuel", ship_symbol),
                    Some(Requests::RefuelShip(fuel_amount)),
                )
                .await?,
        )
        .ok()
    }
    pub async fn purchase_cargo(
        &self,
        ship_symbol: &str,
        data: PurchaseCargo,
    ) -> Option<fleet::PurchaseCargo> {
        serde_json::from_str(
            &self
                .make_request(
                    Method::Post,
                    format!("/my/ships/{}/purchase", ship_symbol),
                    Some(Requests::PurchaseCargo(data)),
                )
                .await?,
        )
        .ok()
    }
    pub async fn transfer_cargo(
        &self,
        ship_symbol: &str,
        data: TransferCargo,
    ) -> Option<fleet::TransferCargo> {
        serde_json::from_str(
            &self
                .make_request(
                    Method::Post,
                    format!("/my/ships/{}/transfer", ship_symbol),
                    Some(Requests::TransferCargo(data)),
                )
                .await?,
        )
        .ok()
    }
    pub async fn negotiate_contract(&self, ship_symbol: &str) -> Option<fleet::NegotiateContract> {
        serde_json::from_str(
            &self
                .make_request(
                    Method::Post,
                    format!("/my/ships/{}/negotiate/contract", ship_symbol),
                    None,
                )
                .await?,
        )
        .ok()
    }
    pub async fn get_mounts(&self, ship_symbol: &str) -> Option<fleet::GetMounts> {
        serde_json::from_str(
            &self
                .make_request(
                    Method::Get,
                    format!("/my/ships/{}/mounts", ship_symbol),
                    None,
                )
                .await?,
        )
        .ok()
    }
    pub async fn install_mount(
        &self,
        ship_symbol: &str,
        data: InstallMount,
    ) -> Option<fleet::InstallMounts> {
        serde_json::from_str(
            &self
                .make_request(
                    Method::Post,
                    format!("/my/ships/{}/mounts/install", ship_symbol),
                    Some(Requests::InstallMount(data)),
                )
                .await?,
        )
        .ok()
    }
    pub async fn remove_mount(
        &self,
        ship_symbol: &str,
        data: RemoveMount,
    ) -> Option<fleet::RemoveMounts> {
        serde_json::from_str(
            &self
                .make_request(
                    Method::Post,
                    format!("/my/ships/{}/mounts/remove", ship_symbol),
                    Some(Requests::RemoveMount(data)),
                )
                .await?,
        )
        .ok()
    }

    // Factions
    pub async fn list_factions(&self) -> Option<factions::Factions> {
        serde_json::from_str(
            &self
                .make_request(Method::Get, String::from("/factions"), None)
                .await?,
        )
        .ok()
    }
    pub async fn get_faction(&self, faction_symbol: &str) -> Option<factions::Faction> {
        serde_json::from_str(
            &self
                .make_request(Method::Get, format!("/factions/{}", faction_symbol), None)
                .await?,
        )
        .ok()
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
    oneshot: oneshot::Sender<Option<String>>,
}

// Waypoint handlers //
#[derive(Clone, Serialize, PartialEq, Eq, Hash, Debug)]
pub struct WaypointString {
    pub waypoint: String,
    pub system: String,
    pub sector: String,
}
impl WaypointString {
    pub fn to_system(&self) -> SystemString {
        SystemString {
            system: self.system.clone(),
            sector: self.sector.clone(),
        }
    }
    pub fn to_sector(&self) -> SectorString {
        SectorString {
            sector: self.sector.clone(),
        }
    }
}
impl<'de> Deserialize<'de> for WaypointString {
    fn deserialize<D>(deserializer: D) -> Result<WaypointString, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(deserializer)?;
        if s.contains('-') {
            let waypoint_split: Vec<&str> = s.split('-').collect();
            if waypoint_split.len() == 3 {
                Ok(WaypointString {
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
            Ok(WaypointString {
                waypoint: "None".to_string(),
                system: "None".to_string(),
                sector: "None".to_string(),
            })
        }
    }
}
#[derive(Clone, Serialize, PartialEq, Eq, Hash, Debug)]
pub struct SystemString {
    pub system: String,
    pub sector: String,
}
impl SystemString {
    pub fn to_sector(&self) -> SectorString {
        SectorString {
            sector: self.sector.clone(),
        }
    }
}
impl<'de> Deserialize<'de> for SystemString {
    fn deserialize<D>(deserializer: D) -> Result<SystemString, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(deserializer)?;
        if s.contains('-') {
            let split: Vec<&str> = s.split('-').collect();
            if split.len() == 2 {
                Ok(SystemString {
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
            Ok(SystemString {
                system: "None".to_string(),
                sector: "None".to_string(),
            })
        }
    }
}
#[derive(Clone, Serialize, PartialEq, Eq, Hash, Debug)]
pub struct SectorString {
    pub sector: String,
}
#[allow(dead_code)]
impl<'de> Deserialize<'de> for SectorString {
    fn deserialize<D>(deserializer: D) -> Result<SectorString, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(deserializer)?;
        if s.contains('-') {
            let split: Vec<&str> = s.split('-').collect();
            if split.len() == 1 {
                Ok(SectorString {
                    sector: split[0].to_string(),
                })
            } else {
                Err(D::Error::invalid_value(
                    Unexpected::Str(s),
                    &"a String as Sector",
                ))
            }
        } else {
            Ok(SectorString {
                sector: "None".to_string(),
            })
        }
    }
}

pub mod spacetraders_datetime_format {
    use chrono::{DateTime, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &str = "%+";

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

    const FORMAT: &str = "%+";

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
