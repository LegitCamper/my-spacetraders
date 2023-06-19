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
    pub data: AcceptContractL1,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct AcceptContractL1 {
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
    pub contract: Contract,
    pub cargo: DeliverContractDataCargo,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct DeliverContractDataCargo {
    pub capacity: u32,
    pub units: u32,
    pub inventory: Vec<schemas::ShipCargoItem>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct FulfillContract {
    pub data: FulfillContractData,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct FulfillContractData {
    pub contract: Contracts,
    pub agent: schemas::Agent,
}
