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
    pub agent: schemas::Agent,
    pub contract: schemas::Contract,
    pub faction: schemas::Faction,
    pub ship: schemas::Ship,
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
