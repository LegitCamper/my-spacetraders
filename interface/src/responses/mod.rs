pub mod agents;
pub mod contracts;
pub mod factions;
pub mod fleet;
pub mod schemas;
pub mod systems;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct GetRegistrationL0 {
    pub data: GetRegistrationData,
}
#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct GetRegistrationData {
    // there is more data - I only want the token
    pub token: String,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Error {
    pub error: ErrorError,
}
#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct ErrorError {
    pub code: u32,
    pub message: String,
}
