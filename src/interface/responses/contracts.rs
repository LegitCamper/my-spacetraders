use crate::interface::enums::*;
use crate::interface::responses::agents::AgentL1;
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
// Change these name to match their functions // TODO
pub struct ContractTermsL0 {
    // pub data: ContractTermsL1,
    pub data: Vec<ContractTermsL1>,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ContractTermsL1 {
    pub id: String,
    #[serde(alias = "factionSymbol")]
    pub faction_symbol: String,
    pub r#type: ContractTermType,
    pub terms: ContractTermsL2,
    pub accepted: bool,
    pub fulfilled: bool,
    pub expiration: String,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ContractTermsL2 {
    pub deadline: String, // maybe parse this to timestamp
    pub payment: ContractTermsL3,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ContractTermsL3 {
    #[serde(alias = "onAccepted")]
    pub on_accepted: u64,
    #[serde(alias = "onFulfilled")]
    pub on_fulfilled: u64,
    #[serde(default)]
    pub deliver: Vec<ContractTermsL4>,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ContractTermsL4 {
    #[serde(alias = "tradeSymbol")]
    trade_symbol: TradeSymbol,
    #[serde(alias = "destinationSymbol")]
    destination_symbol: String,
    #[serde(alias = "unitesRequired")]
    units_required: u64,
    #[serde(alias = "unitsFulfilled")]
    units_fulfilled: u64,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct AcceptContractL0 {
    data: AcceptContractL1,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct AcceptContractL1 {
    agent: AgentL1,
    contract: ContractTermsL1,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct DeliverContractL0 {
    data: DeliverContractData,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct DeliverContractData {
    contract: ContractTermsL1,
    cargo: DeliverContractDataCargo,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct DeliverContractDataCargo {
    capacity: u32,
    units: u32,
    inventory: Vec<DeliverContractDataCargoInventory>,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct DeliverContractDataCargoInventory {
    symbol: String,
    name: String,
    // description: String
    units: u32,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct FulfillContractL0 {
    data: FulfillContractData,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct FulfillContractData {
    contract: ContractTermsL1,
    agent: FulfillContractDataAgent,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct FulfillContractDataAgent {
    #[serde(alias = "AccountId")]
    account_id: String,
    symbol: String,
    headquarters: String,
    credits: u32,
    #[serde(alias = "startingFaction")]
    starting_faction: String,
}
