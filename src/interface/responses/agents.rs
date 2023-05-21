use crate::interface::enums::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct AgentL0 {
    pub data: AgentL1,
}
#[derive(Deserialize, Debug)]
pub struct AgentL1 {
    #[serde(alias = "accountId")]
    pub account_id: String,
    pub symbol: String,
    pub headquarters: String,
    pub credits: u64,
    #[serde(alias = "startingFaction")]
    pub starting_faction: String,
}
