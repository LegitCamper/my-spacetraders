use reqwest::Client;
use url::Url;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Credentials {
    pub token: String
}

impl Credentials {
    pub fn new(token: &str) -> Self {
        Credentials {
            token: String::from(token)
        }
    }
}

#[derive(Debug)]
pub struct SpaceTraders {
    credentials: Credentials,
    pub client: Client,
    pub url: String
}

impl SpaceTraders {
    pub fn new(credentials: Credentials) -> Self {
        SpaceTraders {
            credentials,
            client: reqwest::Client::new(),
            url: String::from("https://api.spacetraders.io")
        }
        
    }

    fn get_header(&self) -> String {
        format!("Bearer {}", self.credentials.token)
    }

    pub async fn agent_details(&self) -> AgentInfoL0 {
        let endpoint = Url::parse(format!("{}{}", self.url, "/v2/my/agent").as_str()).unwrap();
        match self.client.get(endpoint)
            .header("Authorization", self.get_header())
            .send()
            .await
            .unwrap()
            // .json::<AgentInfoL0>()
        .text()
            .await
            // .unwrap()
        {
            // Ok(response) => response,
            Ok(response) => serde_json::from_str(response.as_str()).unwrap(),
            Err(error) => panic!("{}", error)
        }
    }

    pub async fn ship_waypoint(&self) -> String {
        let endpoint = Url::parse(format!("{}{}", self.url, "/v2/systems/:systemSymbol/waypoints/:waypointSymbol").as_str()).unwrap();
        println!("{}", self.get_header());
        match self.client.get(endpoint)
            .header("Authorization", self.get_header())
            .send()
            .await
            .unwrap()
            .text()
            .await
        {
            Ok(response) => response,
            Err(error) => panic!("{}", error)
        }
    }
}


// Other structs for reponses from spacetrades

#[warn(non_snake_case)]
// #[serde(rename_all = "snake_case")]
#[derive(Deserialize, Debug)]
pub struct AgentInfoL0 {
    pub data: AgentInfoL1
}

#[derive(Deserialize, Debug)]
pub struct AgentInfoL1 {
    pub Account_Id: String,
    pub symbol: String,
    pub headquarters: String,
    pub credits: u64
}