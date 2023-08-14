pub mod agents;
pub mod contracts;
pub mod factions;
pub mod fleet;
pub mod schemas;
pub mod systems;

use crate::{spacetraders_date_format, spacetraders_datetime_format};

use chrono::{offset::Utc, DateTime};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct GetStatus {
    pub status: String,
    pub version: String,
    #[serde(alias = "resetDate")]
    #[serde(with = "spacetraders_date_format")]
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
    pub systems: i64,
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
    #[serde(with = "spacetraders_datetime_format")]
    pub next: DateTime<Utc>,
    pub frequency: String,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct GetStatusAnnouncements {
    title: String,
    body: String,
}
#[allow(dead_code)]
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
