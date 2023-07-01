pub mod agents;
pub mod contracts;
pub mod factions;
pub mod fleet;
pub mod schemas;
pub mod systems;

use chrono::{DateTime, Utc};
use serde::Deserialize;

// currently fails tests because it trys "string" :facepalm:
#[derive(Deserialize, Debug)]
pub struct GetStatus {
    pub status: String,
    pub version: String,
    #[serde(alias = "resetDate")]
    pub reset_date: DateTime<Utc>,
    // pub description: String,
    pub stats: GetStatusStats,
    pub leaderboards: GetStatusLeaderboards,
    #[serde(alias = "serverResets")]
    pub server_resets: GetStatusServerResets,
    pub announcements: Vec<GetStatusAnnouncements>,
    pub links: Vec<GetStatusLinks>,
}
#[derive(Deserialize, Debug)]
pub struct GetStatusStats {
    pub agents: i64,
    pub ships: i64,
    pub systems: u32,
    pub waypoints: i64,
}
#[derive(Deserialize, Debug)]
pub struct GetStatusLeaderboards {
    #[serde(alias = "mostCredits")]
    pub most_credits: Vec<GetStatusLeaderboardsMostCredits>,
    #[serde(alias = "mostSubmittedCharts")]
    pub most_submitted_charts: Vec<GetStatusLeaderboardsMostSubmittedCharts>,
}
#[derive(Deserialize, Debug)]
pub struct GetStatusLeaderboardsMostCredits {
    #[serde(alias = "agentSymbol")]
    pub agent_symbol: String,
    pub credits: i64,
}
#[derive(Deserialize, Debug)]
pub struct GetStatusLeaderboardsMostSubmittedCharts {
    #[serde(alias = "agentSymbol")]
    pub agent_symbol: String,
    #[serde(alias = "chartCount")]
    pub chart_count: i64,
}
#[derive(Deserialize, Debug)]
pub struct GetStatusServerResets {
    // #[serde(with = "my_date_format")]
    pub next: DateTime<Utc>,
    pub frequency: String,
}
#[derive(Deserialize, Debug)]
pub struct GetStatusAnnouncements {
    title: String,
    body: String,
}
#[derive(Deserialize, Debug)]
pub struct GetStatusLinks {
    name: String,
    url: String,
}

#[derive(Deserialize, Debug)]
pub struct RegisterNewAgent {
    pub data: RegisterNewAgentData,
}
#[derive(Deserialize, Debug)]
pub struct RegisterNewAgentData {
    pub agent: schemas::Agent,
    pub contract: schemas::Contract,
    pub faction: schemas::Faction,
    pub ship: schemas::Ship,
    pub token: String,
}

#[derive(Deserialize, Debug)]
pub struct Error {
    pub error: ErrorData,
}
#[derive(Deserialize, Debug)]
pub struct ErrorData {
    pub code: u32,
    pub message: String,
}

pub mod my_date_format {
    use chrono::{DateTime, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

    // The signature of a serialize_with function must follow the pattern:
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // although it may also be generic over the input types T.
    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Utc.datetime_from_str(&s, FORMAT)
            .map_err(serde::de::Error::custom)
    }
}
