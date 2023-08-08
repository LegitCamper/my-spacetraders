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
use thiserror::Error;

use tokio::{
    sync::{mpsc, oneshot},
    task,
    time::interval,
};
use url::Url;

const LIVEURL: &str = "https://api.spacetraders.io/v2";
const MOCKURL: &str = "https://stoplight.io/mocks/spacetraders/spacetraders/96627693";

#[derive(Debug, Clone, Eq, PartialEq)]
enum Method {
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
    pub email: Option<String>,
    pub client: ClientWithMiddleware,
    pub url: String,
    pub enviroment: SpaceTradersEnv,
}

impl SpaceTradersInterface {
    pub fn new(token: String, email: Option<String>, enviroment: SpaceTradersEnv) -> Self {
        let url = match enviroment {
            SpaceTradersEnv::Live => LIVEURL,
            SpaceTradersEnv::Mock => MOCKURL,
        };

        let retry_policy = ExponentialBackoff::builder()
            .build_with_total_retry_duration(std::time::Duration::from_secs(60));

        SpaceTradersInterface {
            token,
            email,
            client: ClientBuilder::new(reqwest::Client::new())
                .with(RetryTransientMiddleware::new_with_policy(retry_policy))
                .build(),
            url: String::from(url),
            enviroment,
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
                error!("Reqwest Client Error: {}", msg);
                Err(msg)
            }
            Ok(msg) => Ok(msg),
        };
        match response {
            Ok(response) => match response.text().await {
                Err(err) => {
                    error!("Error from response: {}", err);
                    None
                }
                Ok(msg) => Some(msg),
            },
            Err(msg) => match msg {
                reqwest_middleware::Error::Middleware(err) => {
                    error!("Error from reqwest middleware: {}", err);
                    None
                }
                reqwest_middleware::Error::Reqwest(err) => {
                    error!("Error from reqwest: {}", err);
                    None
                }
            },
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
    pub async fn new(token: &str, email: Option<&str>, enviroment: SpaceTradersEnv) -> Self {
        let space_trader = match email {
            Some(email) => {
                SpaceTradersInterface::new(token.to_string(), Some(email.to_string()), enviroment)
            }
            None => SpaceTradersInterface::new(token.to_string(), None, enviroment),
        };

        let (channel_sender, mut channel_receiver) = mpsc::channel(120);

        let mut interval = interval(tokio::time::Duration::from_millis(500));

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
                SpaceTraders::new(&registration.data.token, None, SpaceTradersEnv::Live).await
            }
            Err(_) => {
                error!("Transitive error occured creating new agent. Trying again");
                SpaceTraders::default().await
            }
        }
    }

    #[allow(dead_code)]
    async fn testing() -> Self {
        SpaceTraders::new("undefined", None, SpaceTradersEnv::Mock).await
    }

    pub fn diagnose(&self) -> String {
        format!(
            "\nurl: {}\nenviroment: {:#?}\ntoken: {}",
            self.interface.url, self.interface.enviroment, self.interface.token,
        )
    }

    async fn make_request(
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
                error!("Error Request Channel: {}", error);
                None
            }
            Ok(data) => Some(data?),
        }
    }

    // Status
    pub async fn get_status(&self) -> Result<GetStatus, SpacetradersError> {
        handle_response(
            self.make_request(Method::Get, "".to_string(), None)
                .await
                .as_deref(),
        )
    }

    // Agents
    pub async fn agent(&self) -> Result<agents::Agent, SpacetradersError> {
        handle_response(
            self.make_request(Method::Get, "/my/agent".to_string(), None)
                .await
                .as_deref(),
        )
    }

    // Systems

    async fn list_systems_page(
        &self,
        page: Option<u32>,
    ) -> Result<systems::Systems, SpacetradersError> {
        let page_num = page.unwrap_or(1);
        handle_response(
            self.make_request(
                Method::Get,
                format!("/systems?limit=20&page={}", page_num),
                None,
            )
            .await
            .as_deref(),
        )
    }
    pub async fn list_systems(&self) -> Result<systems::Systems, SpacetradersError> {
        match self.list_systems_page(None).await {
            Ok(mut systems) => {
                // if systems.meta.total > 1 {
                for page_num in 2..systems.meta.total {
                    if let Ok(more_waypoints) = self.list_systems_page(Some(page_num)).await {
                        systems.data.extend(more_waypoints.data);
                    }
                }
                // }
                Ok(systems)
            }
            Err(err) => Err(err),
        }
    }
    pub async fn get_system(
        &self,
        system_symbol: &SystemString,
    ) -> Result<systems::System, SpacetradersError> {
        handle_response(
            self.make_request(
                Method::Get,
                format!("/systems/{}", system_symbol.system),
                None,
            )
            .await
            .as_deref(),
        )
    }
    async fn list_waypoints_page(
        &self,
        system_symbol: &SystemString,
        page: Option<u32>,
    ) -> Result<systems::Waypoints, SpacetradersError> {
        let page_num = page.unwrap_or(1);
        handle_response(
            self.make_request(
                Method::Get,
                format!(
                    "/systems/{}/waypoints?limit=20&page={}",
                    system_symbol.system, page_num
                ),
                None,
            )
            .await
            .as_deref(),
        )
    }
    pub async fn list_waypoints(
        &self,
        system_symbol: &SystemString,
    ) -> Result<systems::Waypoints, SpacetradersError> {
        match self.list_waypoints_page(system_symbol, None).await {
            Ok(mut waypoints) => {
                // if waypoints.meta.total > 1 {
                for page_num in 2..waypoints.meta.total {
                    if let Ok(more_waypoints) = self
                        .list_waypoints_page(system_symbol, Some(page_num))
                        .await
                    {
                        waypoints.data.extend(more_waypoints.data);
                    }
                }
                // }
                Ok(waypoints)
            }
            Err(err) => Err(err),
        }
    }
    pub async fn get_waypoint(
        &self,
        system_symbol: &SystemString,
        waypoint_symbol: &WaypointString,
    ) -> Result<systems::Waypoint, SpacetradersError> {
        handle_response(
            self.make_request(
                Method::Get,
                format!(
                    "/systems/{}/waypoints/{}",
                    system_symbol.system, waypoint_symbol.waypoint
                ),
                None,
            )
            .await
            .as_deref(),
        )
    }
    pub async fn get_market(
        &self,
        system_symbol: &SystemString,
        waypoint_symbol: &WaypointString,
    ) -> Result<systems::Market, SpacetradersError> {
        handle_response(
            self.make_request(
                Method::Get,
                format!(
                    "/systems/{}/waypoints/{}/market",
                    system_symbol.system, waypoint_symbol.waypoint
                ),
                None,
            )
            .await
            .as_deref(),
        )
    }
    pub async fn get_shipyard(
        &self,
        system_symbol: &SystemString,
        waypoint_symbol: &WaypointString,
    ) -> Result<systems::Shipyard, SpacetradersError> {
        handle_response(
            self.make_request(
                Method::Get,
                format!(
                    "/systems/{}/waypoints/{}/shipyard",
                    system_symbol.system, waypoint_symbol.waypoint
                ),
                None,
            )
            .await
            .as_deref(),
        )
    }
    pub async fn jump_gate(
        &self,
        symbol: &WaypointString,
    ) -> Result<systems::JumpGate, SpacetradersError> {
        handle_response(
            self.make_request(
                Method::Get,
                format!(
                    "/systems/{}/waypoints/{}/jump-gate",
                    symbol.system, symbol.waypoint
                ),
                None,
            )
            .await
            .as_deref(),
        )
    }

    // Contracts
    async fn list_contracts_page(
        &self,
        page_num: Option<u32>,
    ) -> Result<contracts::Contracts, SpacetradersError> {
        let page_num = page_num.unwrap_or(1);
        handle_response(
            self.make_request(
                Method::Get,
                format!("/my/contracts.as_deref()limit=20&page={}", page_num),
                None,
            )
            .await
            .as_deref(),
        )
    }
    pub async fn list_contracts(&self) -> Result<contracts::Contracts, SpacetradersError> {
        match self.list_contracts_page(None).await {
            Ok(mut contracts) => {
                // if contracts.meta.total > 1 {
                for page_num in 2..contracts.meta.total {
                    if let Ok(more_contracts) = self.list_contracts_page(Some(page_num)).await {
                        contracts.data.extend(more_contracts.data);
                    }
                }
                // }
                Ok(contracts)
            }
            Err(err) => Err(err),
        }
    }
    pub async fn get_contract(
        &self,
        contract_id: &str,
    ) -> Result<contracts::Contract, SpacetradersError> {
        handle_response(
            self.make_request(Method::Get, format!("/my/contracts/{}", contract_id), None)
                .await
                .as_deref(),
        )
    }
    pub async fn accept_contract(
        &self,
        contract_id: &str,
    ) -> Result<contracts::AcceptContract, SpacetradersError> {
        handle_response(
            self.make_request(
                Method::Post,
                format!("/my/contracts/{}/accept", contract_id),
                None,
            )
            .await
            .as_deref(),
        )
    }
    pub async fn deliver_contract(
        &self,
        contract_id: &str,
        data: DeliverCargoToContract,
    ) -> Result<contracts::DeliverContract, SpacetradersError> {
        handle_response(
            self.make_request(
                Method::Post,
                format!("/my/contracts/{}/deliver", contract_id),
                Some(Requests::DeliverCargoToContract(data)),
            )
            .await
            .as_deref(),
        )
    }
    pub async fn fulfill_contract(
        &self,
        contract_id: &str,
    ) -> Result<contracts::FulfillContract, SpacetradersError> {
        handle_response(
            self.make_request(
                Method::Post,
                format!("/my/contracts/{}/fulfill", contract_id),
                None,
            )
            .await
            .as_deref(),
        )
    }

    // Fleet
    pub async fn list_ships(&self) -> Result<fleet::Ships, SpacetradersError> {
        handle_response(
            self.make_request(Method::Get, String::from("/my/ships"), None)
                .await
                .as_deref(),
        )
    }
    pub async fn purchase_ship(
        &self,
        data: PurchaseShip,
    ) -> Result<fleet::PurchaseShip, SpacetradersError> {
        handle_response(
            self.make_request(
                Method::Post,
                String::from("/my/ships"),
                Some(Requests::PurchaseShip(data)),
            )
            .await
            .as_deref(),
        )
    }
    pub async fn get_ship(&self, ship_symbol: &str) -> Result<fleet::Ship, SpacetradersError> {
        handle_response(
            self.make_request(Method::Get, format!("/my/ships/{}", ship_symbol), None)
                .await
                .as_deref(),
        )
    }
    pub async fn get_ship_cargo(
        &self,
        ship_symbol: &str,
    ) -> Result<fleet::ShipCargo, SpacetradersError> {
        handle_response(
            self.make_request(
                Method::Get,
                format!("/my/ships/{}/cargo", ship_symbol),
                None,
            )
            .await
            .as_deref(),
        )
    }
    pub async fn orbit_ship(
        &self,
        ship_symbol: &str,
    ) -> Result<fleet::OrbitShip, SpacetradersError> {
        handle_response(
            self.make_request(
                Method::Post,
                format!("/my/ships/{}/orbit", ship_symbol),
                None,
            )
            .await
            .as_deref(),
        )
    }
    pub async fn ship_refine(
        &self,
        ship_symbol: &str,
        data: ShipRefine,
    ) -> Result<fleet::ShipRefine, SpacetradersError> {
        handle_response(
            self.make_request(
                Method::Post,
                format!("/my/ships/{}/refine", ship_symbol),
                Some(Requests::ShipRefine(data)),
            )
            .await
            .as_deref(),
        )
    }
    pub async fn create_chart(
        &self,
        ship_symbol: &str,
    ) -> Result<fleet::CreateChart, SpacetradersError> {
        handle_response(
            self.make_request(
                Method::Post,
                format!("/my/ships/{}/chart", ship_symbol),
                None,
            )
            .await
            .as_deref(),
        )
    }
    pub async fn get_ship_cooldown(
        &self,
        ship_symbol: &str,
    ) -> Result<fleet::GetShipCooldown, SpacetradersError> {
        handle_response(
            self.make_request(
                Method::Get,
                format!("/my/ships/{}/cooldown", ship_symbol),
                None,
            )
            .await
            .as_deref(),
        )
    }
    pub async fn dock_ship(&self, ship_symbol: &str) -> Result<fleet::DockShip, SpacetradersError> {
        handle_response(
            self.make_request(
                Method::Post,
                format!("/my/ships/{}/dock", ship_symbol),
                None,
            )
            .await
            .as_deref(),
        )
    }
    pub async fn create_survey(
        &self,
        ship_symbol: &str,
    ) -> Result<fleet::CreateSurvey, SpacetradersError> {
        handle_response(
            self.make_request(
                Method::Post,
                format!("/my/ships/{}/survey", ship_symbol),
                None,
            )
            .await
            .as_deref(),
        )
    }
    pub async fn extract_resources(
        &self,
        ship_symbol: &str,
        data: Option<schemas::Survey>,
    ) -> Result<fleet::ExtractResources, SpacetradersError> {
        match data {
            Some(data) => handle_response(
                self.make_request(
                    Method::Post,
                    format!("/my/ships/{}/extract", ship_symbol),
                    Some(Requests::ExtractResources(data)),
                )
                .await
                .as_deref(),
            ),
            None => handle_response(
                self.make_request(
                    Method::Post,
                    format!("/my/ships/{}/extract", ship_symbol),
                    None,
                )
                .await
                .as_deref(),
            ),
        }
    }
    pub async fn jettison_cargo(
        &self,
        ship_symbol: &str,
        data: JettisonCargo,
    ) -> Result<fleet::JettisonCargo, SpacetradersError> {
        handle_response(
            self.make_request(
                Method::Post,
                format!("/my/ships/{}/jettison", ship_symbol),
                Some(Requests::JettisonCargo(data)),
            )
            .await
            .as_deref(),
        )
    }
    pub async fn jump_ship(
        &self,
        ship_symbol: &str,
        data: JumpShip,
    ) -> Result<fleet::JumpShip, SpacetradersError> {
        handle_response(
            self.make_request(
                Method::Post,
                format!("/my/ships/{}/jump", ship_symbol),
                Some(Requests::JumpShip(data)),
            )
            .await
            .as_deref(),
        )
    }
    pub async fn navigate_ship(
        &self,
        ship_symbol: &str,
        data: NavigateShip,
    ) -> Result<fleet::NavigateShip, SpacetradersError> {
        handle_response(
            self.make_request(
                Method::Post,
                format!("/my/ships/{}/navigate", ship_symbol),
                Some(Requests::NavigateShip(data)),
            )
            .await
            .as_deref(),
        )
    }
    pub async fn patch_ship_nav(
        &self,
        ship_symbol: &str,
        data: PatchShipNav,
    ) -> Result<fleet::PatchShipNav, SpacetradersError> {
        handle_response(
            self.make_request(
                Method::Patch,
                format!("/my/ships/{}/nav", ship_symbol),
                Some(Requests::PatchShipNav(data)),
            )
            .await
            .as_deref(),
        )
    }
    pub async fn get_ship_nav(
        &self,
        ship_symbol: &str,
    ) -> Result<fleet::GetShipNav, SpacetradersError> {
        handle_response(
            self.make_request(Method::Get, format!("/my/ships/{}/nav", ship_symbol), None)
                .await
                .as_deref(),
        )
    }
    pub async fn warp_ship(
        &self,
        ship_symbol: &str,
        data: WarpShip,
    ) -> Result<fleet::WarpShip, SpacetradersError> {
        handle_response(
            self.make_request(
                Method::Post,
                format!("/my/ships/{}/warp", ship_symbol),
                Some(Requests::WarpShip(data)),
            )
            .await
            .as_deref(),
        )
    }
    pub async fn sell_cargo(
        &self,
        ship_symbol: &str,
        data: SellCargo,
    ) -> Result<fleet::SellCargo, SpacetradersError> {
        handle_response(
            self.make_request(
                Method::Post,
                format!("/my/ships/{}/sell", ship_symbol),
                Some(Requests::SellCargo(data)),
            )
            .await
            .as_deref(),
        )
    }
    pub async fn scan_systems(
        &self,
        ship_symbol: &str,
    ) -> Result<fleet::ScanSystems, SpacetradersError> {
        handle_response(
            self.make_request(
                Method::Post,
                format!("/my/ships/{}/scan/systems", ship_symbol),
                None,
            )
            .await
            .as_deref(),
        )
    }
    pub async fn scan_waypoints(
        &self,
        ship_symbol: &str,
    ) -> Result<fleet::ScanWaypoints, SpacetradersError> {
        handle_response(
            self.make_request(
                Method::Post,
                format!("/my/ships/{}/scan/waypoints", ship_symbol),
                None,
            )
            .await
            .as_deref(),
        )
    }
    pub async fn scan_ships(
        &self,
        ship_symbol: &str,
    ) -> Result<fleet::ScanShips, SpacetradersError> {
        handle_response(
            self.make_request(
                Method::Post,
                format!("/my/ships/{}/scan/ships", ship_symbol),
                None,
            )
            .await
            .as_deref(),
        )
    }
    pub async fn refuel_ship(
        &self,
        ship_symbol: &str,
        fuel_amount: Result<requests::RefuelShip, SpacetradersError>,
    ) -> Result<fleet::RefuelShip, SpacetradersError> {
        let fuel_amount = fuel_amount.unwrap_or(requests::RefuelShip { units: 1 });
        handle_response(
            self.make_request(
                Method::Post,
                format!("/my/ships/{}/refuel", ship_symbol),
                Some(Requests::RefuelShip(fuel_amount)),
            )
            .await
            .as_deref(),
        )
    }
    pub async fn purchase_cargo(
        &self,
        ship_symbol: &str,
        data: PurchaseCargo,
    ) -> Result<fleet::PurchaseCargo, SpacetradersError> {
        handle_response(
            self.make_request(
                Method::Post,
                format!("/my/ships/{}/purchase", ship_symbol),
                Some(Requests::PurchaseCargo(data)),
            )
            .await
            .as_deref(),
        )
    }
    pub async fn transfer_cargo(
        &self,
        ship_symbol: &str,
        data: TransferCargo,
    ) -> Result<fleet::TransferCargo, SpacetradersError> {
        handle_response(
            self.make_request(
                Method::Post,
                format!("/my/ships/{}/transfer", ship_symbol),
                Some(Requests::TransferCargo(data)),
            )
            .await
            .as_deref(),
        )
    }
    pub async fn negotiate_contract(
        &self,
        ship_symbol: &str,
    ) -> Result<fleet::NegotiateContract, SpacetradersError> {
        handle_response(
            self.make_request(
                Method::Post,
                format!("/my/ships/{}/negotiate/contract", ship_symbol),
                None,
            )
            .await
            .as_deref(),
        )
    }
    pub async fn get_mounts(
        &self,
        ship_symbol: &str,
    ) -> Result<fleet::GetMounts, SpacetradersError> {
        handle_response(
            self.make_request(
                Method::Get,
                format!("/my/ships/{}/mounts", ship_symbol),
                None,
            )
            .await
            .as_deref(),
        )
    }
    pub async fn install_mount(
        &self,
        ship_symbol: &str,
        data: InstallMount,
    ) -> Result<fleet::InstallMounts, SpacetradersError> {
        handle_response(
            self.make_request(
                Method::Post,
                format!("/my/ships/{}/mounts/install", ship_symbol),
                Some(Requests::InstallMount(data)),
            )
            .await
            .as_deref(),
        )
    }
    pub async fn remove_mount(
        &self,
        ship_symbol: &str,
        data: RemoveMount,
    ) -> Result<fleet::RemoveMounts, SpacetradersError> {
        handle_response(
            self.make_request(
                Method::Post,
                format!("/my/ships/{}/mounts/remove", ship_symbol),
                Some(Requests::RemoveMount(data)),
            )
            .await
            .as_deref(),
        )
    }

    // Factions
    pub async fn list_factions(&self) -> Result<factions::Factions, SpacetradersError> {
        handle_response(
            self.make_request(Method::Get, String::from("/factions"), None)
                .await
                .as_deref(),
        )
    }
    pub async fn get_faction(
        &self,
        faction_symbol: &str,
    ) -> Result<factions::Faction, SpacetradersError> {
        handle_response(
            self.make_request(Method::Get, format!("/factions/{}", faction_symbol), None)
                .await
                .as_deref(),
        )
    }
}

// simplifies the error handling of responses
fn handle_response<T: for<'a> Deserialize<'a>>(
    response: Option<&str>,
) -> Result<T, SpacetradersError> {
    match response {
        Some(response_str) => match serde_json::from_str(response_str) {
            Ok(response) => Ok(response),
            Err(error_str) => match parse_error(response_str) {
                Some(response) => {
                    if response != SpacetradersError::Other {
                        error!("SpaceTraders Error: {}", response);
                        Err(response)
                    } else {
                        error!(
                            "SpaceTraders Error: Other - {}\n{}",
                            error_str, response_str
                        );
                        Err(response)
                    }
                }
                None => {
                    error!(
                        "SpaceTraders Error (Could not parse json): {}, details: {}",
                        SpacetradersError::Serde,
                        response_str
                    );
                    Err(SpacetradersError::Serde)
                }
            },
        },
        None => {
            error!(
                "SpaceTraders Error (response): {}",
                SpacetradersError::Serde
            );
            Err(SpacetradersError::Serde)
        }
    }
}

#[derive(Debug, Clone, Error, Eq, PartialEq)]
pub enum SpacetradersError {
    CooldownConflictError,
    WaypointNoAccessError,

    TokenEmptyError,
    TokenMissingSubjectError,
    TokenInvalidSubjectError,
    MissingTokenRequestError,
    InvalidTokenRequestError,
    InvalidTokenSubjectError,
    AccountNotExistsError,
    AgentNotExistsError,
    AccountHasNoAgentError,
    RegisterAgentExistsError,

    NavigateInTransitError,
    NavigateInvalidDestinationError,
    NavigateOutsideSystemError,
    NavigateInsufficientFuelError,
    NavigateSameDestinationError,
    ShipExtractInvalidWaypointError,
    ShipExtractPermissionError,
    ShipJumpNoSystemError,
    ShipJumpSameSystemError,
    ShipJumpMissingModuleError,
    ShipJumpNoValidWaypointError,
    ShipJumpMissingAntimatterError,
    ShipInTransitError,
    ShipMissingSensorArraysError,
    PurchaseShipCreditsError,
    ShipCargoExceedsLimitError,
    ShipCargoMissingError,
    ShipCargoUnitCountError,
    ShipSurveyVerificationError,
    ShipSurveyExpirationError,
    ShipSurveyWaypointTypeError,
    ShipSurveyOrbitError,
    ShipSurveyExhaustedError,
    ShipRefuelDockedError,
    ShipRefuelInvalidWaypointError,
    ShipMissingMountsError,
    ShipCargoFullError,
    ShipJumpFromGateToGateError,
    WaypointChartedError,
    ShipTransferShipNotFound,
    ShipTransferAgentConflict,
    ShipTransferSameShipConflict,
    ShipTransferLocationConflict,
    WarpInsideSystemError,
    ShipNotInOrbitError,
    ShipInvalidRefineryGoodError,
    ShipInvalidRefineryTypeError,
    ShipMissingRefineryError,
    ShipMissingSurveyorError,

    AcceptContractNotAuthorizedError,
    AcceptContractConflictError,
    FulfillContractDeliveryError,
    ContractDeadlineError,
    ContractFulfilledError,
    ContractNotAcceptedError,
    ContractNotAuthorizedError,
    ShipDeliverTermsError,
    ShipDeliverFulfilledError,
    ShipDeliverInvalidLocationError,

    MarketTradeInsufficientCreditsError,
    MarketTradeNoPurchaseError,
    MarketTradeNotSoldError,
    MarketNotFoundError,
    MarketTradeUnitLimitError,

    Reqwest,
    Serde,
    Other,
}
impl std::fmt::Display for SpacetradersError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
fn parse_error(error_str: &str) -> Option<SpacetradersError> {
    if let Ok(error) = serde_json::from_str::<responses::Error>(error_str) {
        if error.error.code == 4000 {
            Some(SpacetradersError::CooldownConflictError)
        } else if error.error.code == 4001 {
            Some(SpacetradersError::WaypointNoAccessError)
        } else if error.error.code == 4100 {
            Some(SpacetradersError::TokenEmptyError)
        } else if error.error.code == 4101 {
            Some(SpacetradersError::TokenMissingSubjectError)
        } else if error.error.code == 4102 {
            Some(SpacetradersError::TokenInvalidSubjectError)
        } else if error.error.code == 4103 {
            Some(SpacetradersError::MissingTokenRequestError)
        } else if error.error.code == 4104 {
            Some(SpacetradersError::InvalidTokenRequestError)
        } else if error.error.code == 4105 {
            Some(SpacetradersError::InvalidTokenSubjectError)
        } else if error.error.code == 4106 {
            Some(SpacetradersError::AccountNotExistsError)
        } else if error.error.code == 4107 {
            Some(SpacetradersError::AgentNotExistsError)
        } else if error.error.code == 4108 {
            Some(SpacetradersError::AccountHasNoAgentError)
        } else if error.error.code == 4109 {
            Some(SpacetradersError::RegisterAgentExistsError)
        } else if error.error.code == 4200 {
            Some(SpacetradersError::NavigateInTransitError)
        } else if error.error.code == 4201 {
            Some(SpacetradersError::NavigateInvalidDestinationError)
        } else if error.error.code == 4202 {
            Some(SpacetradersError::NavigateOutsideSystemError)
        } else if error.error.code == 4203 {
            Some(SpacetradersError::NavigateInsufficientFuelError)
        } else if error.error.code == 4204 {
            Some(SpacetradersError::NavigateSameDestinationError)
        } else if error.error.code == 4205 {
            Some(SpacetradersError::ShipExtractInvalidWaypointError)
        } else if error.error.code == 4206 {
            Some(SpacetradersError::ShipExtractPermissionError)
        } else if error.error.code == 4207 {
            Some(SpacetradersError::ShipJumpNoSystemError)
        } else if error.error.code == 4208 {
            Some(SpacetradersError::ShipJumpSameSystemError)
        } else if error.error.code == 4210 {
            Some(SpacetradersError::ShipJumpMissingModuleError)
        } else if error.error.code == 4211 {
            Some(SpacetradersError::ShipJumpNoValidWaypointError)
        } else if error.error.code == 4212 {
            Some(SpacetradersError::ShipJumpMissingAntimatterError)
        } else if error.error.code == 4214 {
            Some(SpacetradersError::ShipInTransitError)
        } else if error.error.code == 4215 {
            Some(SpacetradersError::ShipMissingSensorArraysError)
        } else if error.error.code == 4216 {
            Some(SpacetradersError::PurchaseShipCreditsError)
        } else if error.error.code == 4217 {
            Some(SpacetradersError::ShipCargoExceedsLimitError)
        } else if error.error.code == 4218 {
            Some(SpacetradersError::ShipCargoMissingError)
        } else if error.error.code == 4219 {
            Some(SpacetradersError::ShipCargoUnitCountError)
        } else if error.error.code == 4220 {
            Some(SpacetradersError::ShipSurveyVerificationError)
        } else if error.error.code == 4221 {
            Some(SpacetradersError::ShipSurveyExpirationError)
        } else if error.error.code == 4222 {
            Some(SpacetradersError::ShipSurveyWaypointTypeError)
        } else if error.error.code == 4223 {
            Some(SpacetradersError::ShipSurveyOrbitError)
        } else if error.error.code == 4224 {
            Some(SpacetradersError::ShipSurveyExhaustedError)
        } else if error.error.code == 4225 {
            Some(SpacetradersError::ShipRefuelDockedError)
        } else if error.error.code == 4226 {
            Some(SpacetradersError::ShipRefuelInvalidWaypointError)
        } else if error.error.code == 4227 {
            Some(SpacetradersError::ShipMissingMountsError)
        } else if error.error.code == 4228 {
            Some(SpacetradersError::ShipCargoFullError)
        } else if error.error.code == 4229 {
            Some(SpacetradersError::ShipJumpFromGateToGateError)
        } else if error.error.code == 4230 {
            Some(SpacetradersError::WaypointChartedError)
        } else if error.error.code == 4231 {
            Some(SpacetradersError::ShipTransferShipNotFound)
        } else if error.error.code == 4232 {
            Some(SpacetradersError::ShipTransferAgentConflict)
        } else if error.error.code == 4233 {
            Some(SpacetradersError::ShipTransferSameShipConflict)
        } else if error.error.code == 4234 {
            Some(SpacetradersError::ShipTransferLocationConflict)
        } else if error.error.code == 4235 {
            Some(SpacetradersError::WarpInsideSystemError)
        } else if error.error.code == 4236 {
            Some(SpacetradersError::ShipNotInOrbitError)
        } else if error.error.code == 4237 {
            Some(SpacetradersError::ShipInvalidRefineryGoodError)
        } else if error.error.code == 4238 {
            Some(SpacetradersError::ShipInvalidRefineryTypeError)
        } else if error.error.code == 4239 {
            Some(SpacetradersError::ShipMissingRefineryError)
        } else if error.error.code == 4240 {
            Some(SpacetradersError::ShipMissingSurveyorError)
        } else if error.error.code == 4500 {
            Some(SpacetradersError::AcceptContractNotAuthorizedError)
        } else if error.error.code == 4501 {
            Some(SpacetradersError::AcceptContractConflictError)
        } else if error.error.code == 4502 {
            Some(SpacetradersError::FulfillContractDeliveryError)
        } else if error.error.code == 4503 {
            Some(SpacetradersError::ContractDeadlineError)
        } else if error.error.code == 4504 {
            Some(SpacetradersError::ContractFulfilledError)
        } else if error.error.code == 4505 {
            Some(SpacetradersError::ContractNotAcceptedError)
        } else if error.error.code == 4506 {
            Some(SpacetradersError::ContractNotAuthorizedError)
        } else if error.error.code == 4508 {
            Some(SpacetradersError::ShipDeliverTermsError)
        } else if error.error.code == 4509 {
            Some(SpacetradersError::ShipDeliverFulfilledError)
        } else if error.error.code == 4510 {
            Some(SpacetradersError::ShipDeliverInvalidLocationError)
        } else if error.error.code == 4600 {
            Some(SpacetradersError::MarketTradeInsufficientCreditsError)
        } else if error.error.code == 4601 {
            Some(SpacetradersError::MarketTradeNoPurchaseError)
        } else if error.error.code == 4602 {
            Some(SpacetradersError::MarketTradeNotSoldError)
        } else if error.error.code == 4603 {
            Some(SpacetradersError::MarketNotFoundError)
        } else if error.error.code == 4604 {
            Some(SpacetradersError::MarketTradeUnitLimitError)
        } else {
            Some(SpacetradersError::Other)
        }
    } else {
        None
    }
}

#[derive(Debug)]
pub struct RequestMessage {
    method: Method,
    url: String,
    data: Option<Requests>,
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
