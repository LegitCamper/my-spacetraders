use crate::interface::enums::*;
use crate::interface::responses::Meta;
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ListFactionsL0 {
    data: Vec<ListFactionsL1>,
    meta: Meta,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ListFactionsL1 {
    symbol: String,
    name: String,
    // description: String,
    headquarters: String,
    traits: Vec<ListFactionTraits>,
    #[serde(alias = "isRecruiting")]
    is_recruiting: bool,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ListFactionTraits {
    symbol: WaypointTrait,
    name: String,
    // description: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct GetFactionsL0 {
    data: ListFactionsL1,
}
