pub use super::schemas;

use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Contracts {
    pub data: Vec<schemas::Contract>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Contract {
    pub data: schemas::Contract,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct AcceptContract {
    pub data: AcceptContractData,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct AcceptContractData {
    pub agent: schemas::Agent,
    pub contract: schemas::Contract,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct DeliverContract {
    pub data: DeliverContractData,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct DeliverContractData {
    pub contract: schemas::Contract,
    pub cargo: schemas::ShipCargo,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct FulfillContract {
    pub data: FulfillContractData,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct FulfillContractData {
    pub contract: schemas::Contract,
    pub agent: schemas::Agent,
}
