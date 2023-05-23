use crate::interface::enums::*;
use crate::interface::responses::agents::AgentL1;
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ListContractsL0 {
    // pub data: ContractTermsL1,
    pub data: Vec<ListContractsL1>,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ListContractsL1 {
    pub id: String,
    #[serde(alias = "factionSymbol")]
    pub faction_symbol: String,
    pub r#type: ListContractsType,
    pub terms: ListContractsL2,
    pub accepted: bool,
    pub fulfilled: bool,
    #[serde(default)]
    pub expiration: String,
    #[serde(default)]
    #[serde(alias = "deadlineToAccept")]
    pub deadline_to_accept: String,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ListContractsL2 {
    pub deadline: String, // maybe parse this to timestamp
    pub payment: ListContractsL3,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ListContractsL3 {
    #[serde(alias = "onAccepted")]
    pub on_accepted: u64,
    #[serde(alias = "onFulfilled")]
    pub on_fulfilled: u64,
    #[serde(default)]
    pub deliver: Vec<ListContractsL4>,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ListContractsL4 {
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
pub struct GetContractsL0 {
    pub data: ListContractsL1,
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
    contract: ListContractsL1,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct DeliverContractL0 {
    data: DeliverContractData,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct DeliverContractData {
    contract: ListContractsL1,
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
    contract: ListContractsL1,
    agent: FulfillContractDataAgent,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct FulfillContractDataAgent {
    #[serde(alias = "accountId")]
    account_id: String,
    symbol: String,
    headquarters: String,
    credits: u32,
    #[serde(alias = "startingFaction")]
    starting_faction: String,
}
