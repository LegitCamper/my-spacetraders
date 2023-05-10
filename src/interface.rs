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

    fn get_url(&self, endpoint: &str) -> Url {
        Url::parse(format!("{}{}", self.url, endpoint).as_str()).unwrap()
    }

    pub async fn agent_details(&self) -> AgentInfoL0 {
        match self
            .client
            .get(self.get_url("/v2/my/agent"))
            .headers(self.get_header())
            .send()
            .await
            .unwrap()
            .json::<AgentInfoL0>()
            .await
        {
            Ok(response) => response,
            Err(error) => panic!("{}", error),
        }
    }

    pub async fn ship_waypoint(&self, system_symbol: String, waypoint_symbol: String) -> String {
        match self
            .client
            .get(self.get_url(&format!(
                "/v2/systems/{}/waypoints/{}",
                system_symbol, waypoint_symbol
            )))
            .headers(self.get_header())
            .send()
            .await
            .unwrap()
            .text()
            .await
        {
            Ok(response) => response,
            Err(error) => panic!("{}", error),
        }
    }

    pub async fn contract_accept(&self, contract_id: &str) -> String {
        match self
            .client
            .post(self.get_url(&format!("/v2/my/contracts/{}/accept", contract_id)))
            .headers(self.get_header())
            .send()
            .await
            .unwrap()
            .text()
            .await
        {
            Ok(response) => response,
            Err(error) => panic!("{}", error),
        }
    }
    pub async fn contract_terms(&self, contract_id: &str) -> ContractTermsL0 {
        match self
            .client
            .get(self.get_url(&format!("/v2/my/contracts/{}", contract_id)))
            .headers(self.get_header())
            .send()
            .await
            .unwrap()
            .json::<ContractTermsL0>()
            .await
        {
            Ok(response) => response,
            Err(error) => panic!("{}", error),
        }
    }
}

// Items :(

#[derive(Deserialize, Debug)]
pub enum Item {
    // not sure this is a good idea
    #[serde(alias = "ALUMINUM_ORE")]
    AluminumOre,
}

// Other helpful structs and enums

pub struct Coordinates {
    x: f64,
    y: f64,
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
#[derive(Deserialize, Debug)]
pub struct ContractTermsL0 {
    data: ContractTermsL1,
}
#[derive(Deserialize, Debug)]
pub struct ContractTermsL1 {
    id: String,
    #[serde(alias = "factionSymbol")]
    faction_symbol: String,
    r#type: ContractTermType,
    terms: ContractTermsL2,
}
#[derive(Deserialize, Debug)]
pub struct ContractTermsL2 {
    deadline: String, // maybe parse this to timestamp
    payment: ContractTermsL3,
}
#[derive(Deserialize, Debug)]
pub struct ContractTermsL3 {
    #[serde(alias = "onAccepted")]
    on_accepted: u64,
    #[serde(alias = "onFulfilled")]
    on_fulfilled: u64,
    #[serde(default)]
    deliver: Vec<ContractTermsL4>,
}
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
