pub use super::schemas;
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Factions {
    pub data: Vec<schemas::Faction>,
    pub meta: schemas::Meta,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Faction {
    pub data: schemas::Faction,
}
