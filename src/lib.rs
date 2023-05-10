use reqwest::Client;
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

    fn get_header(&self) -> String {
        format!("Bearer {}", self.credentials.token)
    }

    fn get_url(&self, endpoint: &str) -> Url {
        Url::parse(format!("{}{}", self.url, endpoint).as_str()).unwrap()
    }

    pub async fn agent_details(&self) -> AgentInfoL0 {
        match self
            .client
            .get(self.get_url("/v2/my/agent"))
            .header("Authorization", self.get_header())
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

    pub async fn ship_waypoint(&self) -> String {
        match self
            .client
            .get(self.get_url("/v2/systems/:systemSymbol/waypoints/:waypointSymbol"))
            .header("Authorization", self.get_header())
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

    pub async fn contracts(&self, contracts: Contracts) -> String {
        let url = match contracts {
            Contracts::Accept => self.get_url("/v2/my/contracts/:contractId/accept"),
        };
        match self
            .client
            .get(url)
            .header("Authorization", self.get_header())
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
}

// Other helpful structs

pub struct Coordinates {
    x: f64,
    y: f64,
}

pub enum Contracts {
    Accept,
    // deny
    // cancel
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
