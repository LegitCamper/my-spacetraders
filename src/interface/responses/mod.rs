use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Meta {
    // metadata for responses
    total: u32,
    page: u32,
    limit: u32,
}

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

pub mod agents;
pub mod contracts;
pub mod fleet;
pub mod systems;

// use unit test to confirm deserialization before runtime

#[cfg(test)]
mod agent_tests {
    use super::agents::AgentL0;

    #[test]
    fn agent() {
        let _: AgentL0 = serde_json::from_str(
            r#"{
  "data": {
    "accountId": "string",
    "symbol": "string",
    "headquarters": "string",
    "credits": 0,
    "startingFaction": "string"
  }
}"#,
        )
        .unwrap();
    }
}

#[cfg(test)]
mod system_tests {
    use super::systems::{
        GetJumpGateL0, GetMarketL0, GetShipyardL0, GetSystemL0, GetWaypointL0, ListSystemsL0,
        ListWaypointsL0,
    };

    #[test]
    fn list_systems() {
        let _: ListSystemsL0 = serde_json::from_str(
            r#"{
  "data": [
    {
      "symbol": "string",
      "sectorSymbol": "string",
      "type": "NEUTRON_STAR",
      "x": 0,
      "y": 0,
      "waypoints": [
        {
          "symbol": "string",
          "type": "PLANET",
          "x": 0,
          "y": 0
        }
      ],
      "factions": [
        {
          "symbol": "string"
        }
      ]
    }
  ],
  "meta": {
    "total": 0,
    "page": 0,
    "limit": 0
  }
}"#,
        )
        .unwrap();
    }
    #[test]
    fn get_systems() {
        let _: GetSystemL0 = serde_json::from_str(
            r#"{
  "data": {
    "symbol": "string",
    "sectorSymbol": "string",
    "type": "NEUTRON_STAR",
    "x": 0,
    "y": 0,
    "waypoints": [
      {
        "symbol": "string",
        "type": "PLANET",
        "x": 0,
        "y": 0
      }
    ],
    "factions": [
      {
        "symbol": "string"
      }
    ]
  }
}"#,
        )
        .unwrap();
    }
    #[test]
    fn list_waypoints() {
        let _: ListWaypointsL0 = serde_json::from_str(
            r#"{
  "data": [
    {
      "symbol": "string",
      "type": "PLANET",
      "systemSymbol": "string",
      "x": 0,
      "y": 0,
      "orbitals": [
        {
          "symbol": "string"
        }
      ],
      "faction": {
        "symbol": "string"
      },
      "traits": [
        {
          "symbol": "UNCHARTED",
          "name": "string",
          "description": "string"
        }
      ],
      "chart": {
        "waypointSymbol": "string",
        "submittedBy": "string",
        "submittedOn": "2019-08-24T14:15:22Z"
      }
    }
  ],
  "meta": {
    "total": 0,
    "page": 0,
    "limit": 0
  }
}"#,
        )
        .unwrap();
    }
    #[test]
    fn get_waypoint() {
        let _: GetWaypointL0 = serde_json::from_str(
            r#"{
  "data": {
    "symbol": "string",
    "type": "PLANET",
    "systemSymbol": "string",
    "x": 0,
    "y": 0,
    "orbitals": [
      {
        "symbol": "string"
      }
    ],
    "faction": {
      "symbol": "string"
    },
    "traits": [
      {
        "symbol": "UNCHARTED",
        "name": "string",
        "description": "string"
      }
    ],
    "chart": {
      "waypointSymbol": "string",
      "submittedBy": "string",
      "submittedOn": "2019-08-24T14:15:22Z"
    }
  }
}"#,
        )
        .unwrap();
    }
    #[test]
    fn get_market() {
        let _: GetMarketL0 = serde_json::from_str(
            r#"{
  "data": {
    "symbol": "string",
    "exports": [
      {
        "symbol": "PRECIOUS_STONES",
        "name": "string",
        "description": "string"
      }
    ],
    "imports": [
      {
        "symbol": "PRECIOUS_STONES",
        "name": "string",
        "description": "string"
      }
    ],
    "exchange": [
      {
        "symbol": "PRECIOUS_STONES",
        "name": "string",
        "description": "string"
      }
    ],
    "transactions": [
      {
        "waypointSymbol": "string",
        "shipSymbol": "string",
        "tradeSymbol": "string",
        "type": "PURCHASE",
        "units": 0,
        "pricePerUnit": 0,
        "totalPrice": 0,
        "timestamp": "2019-08-24T14:15:22Z"
      }
    ],
    "tradeGoods": [
      {
        "symbol": "string",
        "tradeVolume": 1,
        "supply": "SCARCE",
        "purchasePrice": 0,
        "sellPrice": 0
      }
    ]
  }
}"#,
        )
        .unwrap();
    }
    #[test]
    fn get_shipyard() {
        let _: GetShipyardL0 = serde_json::from_str(
            r#"{
  "data": {
    "symbol": "string",
    "shipTypes": [
      {
        "type": "SHIP_PROBE"
      }
    ],
    "transactions": [
      {
        "waypointSymbol": "string",
        "shipSymbol": "string",
        "price": 0,
        "agentSymbol": "string",
        "timestamp": "2019-08-24T14:15:22Z"
      }
    ],
    "ships": [
      {
        "type": "SHIP_PROBE",
        "name": "string",
        "description": "string",
        "purchasePrice": 0,
        "frame": {
          "symbol": "FRAME_PROBE",
          "name": "string",
          "description": "string",
          "condition": 0,
          "moduleSlots": 0,
          "mountingPoints": 0,
          "fuelCapacity": 0,
          "requirements": {
            "power": 0,
            "crew": 0,
            "slots": 0
          }
        },
        "reactor": {
          "symbol": "REACTOR_SOLAR_I",
          "name": "string",
          "description": "string",
          "condition": 0,
          "powerOutput": 1,
          "requirements": {
            "power": 0,
            "crew": 0,
            "slots": 0
          }
        },
        "engine": {
          "symbol": "ENGINE_IMPULSE_DRIVE_I",
          "name": "string",
          "description": "string",
          "condition": 0,
          "speed": 1,
          "requirements": {
            "power": 0,
            "crew": 0,
            "slots": 0
          }
        },
        "modules": [
          {
            "symbol": "MODULE_MINERAL_PROCESSOR_I",
            "capacity": 0,
            "range": 0,
            "name": "string",
            "description": "string",
            "requirements": {
              "power": 0,
              "crew": 0,
              "slots": 0
            }
          }
        ],
        "mounts": [
          {
            "symbol": "MOUNT_GAS_SIPHON_I",
            "name": "string",
            "description": "string",
            "strength": 0,
            "deposits": [
              "QUARTZ_SAND"
            ],
            "requirements": {
              "power": 0,
              "crew": 0,
              "slots": 0
            }
          }
        ]
      }
    ]
  }
}"#,
        )
        .unwrap();
    }
    #[test]
    fn get_jump_gate() {
        let _: GetJumpGateL0 = serde_json::from_str(
            r#"{
  "data": {
    "jumpRange": 0,
    "factionSymbol": "string",
    "connectedSystems": [
      {
        "symbol": "string",
        "sectorSymbol": "string",
        "type": "NEUTRON_STAR",
        "factionSymbol": "string",
        "x": 0,
        "y": 0,
        "distance": 0
      }
    ]
  }
}"#,
        )
        .unwrap();
    }
}

#[cfg(test)]
mod contract_tests {
    use super::contracts::{
        AcceptContractL0, ContractTermsL0, DeliverContractL0, FulfillContractL0,
    };

    #[test]
    fn list_contracts() {
        let _: ContractTermsL0 = serde_json::from_str(
            r#"{
  "data": [
    {
      "id": "string",
      "factionSymbol": "string",
      "type": "PROCUREMENT",
      "terms": {
        "deadline": "2019-08-24T14:15:22Z",
        "payment": {
          "onAccepted": 0,
          "onFulfilled": 0
        },
        "deliver": [
          {
            "tradeSymbol": "string",
            "destinationSymbol": "string",
            "unitsRequired": 0,
            "unitsFulfilled": 0
          }
        ]
      },
      "accepted": false,
      "fulfilled": false,
      "expiration": "2019-08-24T14:15:22Z",
      "deadlineToAccept": "2019-08-24T14:15:22Z"
    }
  ],
  "meta": {
    "total": 0,
    "page": 0,
    "limit": 0
  }
}"#,
        )
        .unwrap();
    }
    #[test]
    fn get_contracts() {
        let _: ContractTermsL0 = serde_json::from_str(
            r#"{
  "data": {
    "id": "string",
    "factionSymbol": "string",
    "type": "PROCUREMENT",
    "terms": {
      "deadline": "2019-08-24T14:15:22Z",
      "payment": {
        "onAccepted": 0,
        "onFulfilled": 0
      },
      "deliver": [
        {
          "tradeSymbol": "string",
          "destinationSymbol": "string",
          "unitsRequired": 0,
          "unitsFulfilled": 0
        }
      ]
    },
    "accepted": false,
    "fulfilled": false,
    "expiration": "2019-08-24T14:15:22Z",
    "deadlineToAccept": "2019-08-24T14:15:22Z"
  }
}"#,
        )
        .unwrap();
    }
    #[test]
    fn accept_contracts() {
        let _: AcceptContractL0 = serde_json::from_str(
            r#"{
  "data": {
    "agent": {
      "accountId": "string",
      "symbol": "string",
      "headquarters": "string",
      "credits": 0,
      "startingFaction": "string"
    },
    "contract": {
      "id": "string",
      "factionSymbol": "string",
      "type": "PROCUREMENT",
      "terms": {
        "deadline": "2019-08-24T14:15:22Z",
        "payment": {
          "onAccepted": 0,
          "onFulfilled": 0
        },
        "deliver": [
          {
            "tradeSymbol": "string",
            "destinationSymbol": "string",
            "unitsRequired": 0,
            "unitsFulfilled": 0
          }
        ]
      },
      "accepted": false,
      "fulfilled": false,
      "expiration": "2019-08-24T14:15:22Z",
      "deadlineToAccept": "2019-08-24T14:15:22Z"
    }
  }
}"#,
        )
        .unwrap();
    }
    #[test]
    fn deliver_contract() {
        let _: DeliverContractL0 = serde_json::from_str(
            r#"{
  "data": {
    "contract": {
      "id": "string",
      "factionSymbol": "string",
      "type": "PROCUREMENT",
      "terms": {
        "deadline": "2019-08-24T14:15:22Z",
        "payment": {
          "onAccepted": 0,
          "onFulfilled": 0
        },
        "deliver": [
          {
            "tradeSymbol": "string",
            "destinationSymbol": "string",
            "unitsRequired": 0,
            "unitsFulfilled": 0
          }
        ]
      },
      "accepted": false,
      "fulfilled": false,
      "expiration": "2019-08-24T14:15:22Z",
      "deadlineToAccept": "2019-08-24T14:15:22Z"
    },
    "cargo": {
      "capacity": 0,
      "units": 0,
      "inventory": [
        {
          "symbol": "string",
          "name": "string",
          "description": "string",
          "units": 1
        }
      ]
    }
  }
}"#,
        )
        .unwrap();
    }
    #[test]
    fn fulfill_contract() {
        let _: FulfillContractL0 = serde_json::from_str(
            r#"{
  "data": {
    "agent": {
      "accountId": "string",
      "symbol": "string",
      "headquarters": "string",
      "credits": 0,
      "startingFaction": "string"
    },
    "contract": {
      "id": "string",
      "factionSymbol": "string",
      "type": "PROCUREMENT",
      "terms": {
        "deadline": "2019-08-24T14:15:22Z",
        "payment": {
          "onAccepted": 0,
          "onFulfilled": 0
        },
        "deliver": [
          {
            "tradeSymbol": "string",
            "destinationSymbol": "string",
            "unitsRequired": 0,
            "unitsFulfilled": 0
          }
        ]
      },
      "accepted": false,
      "fulfilled": false,
      "expiration": "2019-08-24T14:15:22Z",
      "deadlineToAccept": "2019-08-24T14:15:22Z"
    }
  }
}"#,
        )
        .unwrap();
    }
}

#[cfg(test)]
mod fleet_tests {}

#[cfg(test)]
mod faction_tests {}

// #[test]
// fn get_systems() {
//     let _: AgentL0 = serde_json::from_str(r#""#).unwrap();
// }
