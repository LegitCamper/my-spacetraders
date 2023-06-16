pub mod agents;
pub mod contracts;
pub mod factions;
pub mod fleet;
pub mod schemas;
pub mod systems;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct RegisterNewAgent {
    pub data: RegisterNewAgentData,
}
#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct RegisterNewAgentData {
    pub agent: agents::Agent,
    pub contract: contracts::Contract,
    pub faction: factions::Faction,
    pub ship: fleet::Ship,
    pub token: String,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Error {
    pub error: ErrorData,
}
#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct ErrorData {
    pub code: u32,
    pub message: String,
}
