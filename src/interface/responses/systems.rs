pub use super::schemas;

use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Systems {
    pub data: Vec<schemas::System>,
    pub meta: schemas::Meta,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct System {
    pub data: schemas::System,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Waypoints {
    pub data: Vec<schemas::Waypoint>,
    pub meta: schemas::Meta,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Waypoint {
    pub data: schemas::Waypoint,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Market {
    pub data: schemas::Market,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Shipyard {
    pub data: schemas::Shipyard,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct JumpGate {
    pub data: schemas::JumpGate,
}
