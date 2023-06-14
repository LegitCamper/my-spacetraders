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

// use unit test to confirm deserialization before runtime

#[cfg(test)]
mod error_test {
    use super::Error;

    #[test]
    fn recieve_error() {
        let _: Error = serde_json::from_str(
            r#"{
              "error": {
                "code": 400,
                "message": "Invalid JSON data. Review your request body and try again."
              }
            }"#,
        )
        .unwrap();
    }
}

#[cfg(test)]
mod registration_tests {
    use super::GetRegistrationL0;

    #[test]
    fn get_new_registration() {
        let _: GetRegistrationL0 = serde_json::from_str(
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
                },
                "faction": {
                  "symbol": "string",
                  "name": "string",
                  "description": "string",
                  "headquarters": "string",
                  "traits": [
                    {
                      "symbol": "BUREAUCRATIC",
                      "name": "string",
                      "description": "string"
                    }
                  ],
                  "isRecruiting": true
                },
                "ship": {
                  "symbol": "string",
                  "registration": {
                    "name": "string",
                    "factionSymbol": "string",
                    "role": "FABRICATOR"
                  },
                  "nav": {
                    "systemSymbol": "string",
                    "waypointSymbol": "string",
                    "route": {
                      "destination": {
                        "symbol": "string",
                        "type": "PLANET",
                        "systemSymbol": "string",
                        "x": 0,
                        "y": 0
                      },
                      "departure": {
                        "symbol": "string",
                        "type": "PLANET",
                        "systemSymbol": "string",
                        "x": 0,
                        "y": 0
                      },
                      "departureTime": "2019-08-24T14:15:22Z",
                      "arrival": "2019-08-24T14:15:22Z"
                    },
                    "status": "IN_TRANSIT",
                    "flightMode": "CRUISE"
                  },
                  "crew": {
                    "current": 0,
                    "required": 0,
                    "capacity": 0,
                    "rotation": "STRICT",
                    "morale": 0,
                    "wages": 0
                  },
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
                  ],
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
                  },
                  "fuel": {
                    "current": 0,
                    "capacity": 0,
                    "consumed": {
                      "amount": 0,
                      "timestamp": "2019-08-24T14:15:22Z"
                    }
                  }
                },
                "token": "string"
              }
            }"#,
        )
        .unwrap();
    }
}

#[cfg(test)]
mod agent_tests {
    use super::agents::Agent;

    #[test]
    fn agent() {
        let _: Agent = serde_json::from_str(
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
    use super::systems::{JumpGate, Market, Shipyard, System, Systems, Waypoint, Waypoints};

    #[test]
    fn list_systems() {
        let _: Systems = serde_json::from_str(
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
    fn get_system() {
        let _: System = serde_json::from_str(
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
        let _: Waypoints = serde_json::from_str(
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
        let _: Waypoint = serde_json::from_str(
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
        let _: Market = serde_json::from_str(
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
        let _: Shipyard = serde_json::from_str(
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
        let _: JumpGate = serde_json::from_str(
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
    use super::contracts::{AcceptContract, Contract, Contracts, DeliverContract, FulfillContract};

    #[test]
    fn list_contracts() {
        let _: Contracts = serde_json::from_str(
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
        let _: Contract = serde_json::from_str(
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
        let _: AcceptContract = serde_json::from_str(
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
        let _: DeliverContract = serde_json::from_str(
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
        let _: FulfillContract = serde_json::from_str(
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
mod faction_tests {
    use super::factions::{Faction, Factions};

    #[test]
    fn list_factions() {
        let _: Factions = serde_json::from_str(
            r#"{
              "data": [
                {
                  "symbol": "string",
                  "name": "string",
                  "description": "string",
                  "headquarters": "string",
                  "traits": [
                    {
                      "symbol": "BUREAUCRATIC",
                      "name": "string",
                      "description": "string"
                    }
                  ],
                  "isRecruiting": true
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
    fn get_factions() {
        let _: Faction = serde_json::from_str(
            r#"{
              "data": {
                "symbol": "string",
                "name": "string",
                "description": "string",
                "headquarters": "string",
                "traits": [
                  {
                    "symbol": "BUREAUCRATIC",
                    "name": "string",
                    "description": "string"
                  }
                ],
                "isRecruiting": true
              }
            }"#,
        )
        .unwrap();
    }
}

#[cfg(test)]
mod fleet_tests {
    use super::fleet::{CreateChart, OrbitShip, PurchaseShip, Ship, ShipCargo, ShipRefine, Ships};

    #[test]
    fn list_ships() {
        let _: Ships = serde_json::from_str(
            r#"{
              "data": [
                {
                  "symbol": "string",
                  "registration": {
                    "name": "string",
                    "factionSymbol": "string",
                    "role": "FABRICATOR"
                  },
                  "nav": {
                    "systemSymbol": "string",
                    "waypointSymbol": "string",
                    "route": {
                      "destination": {
                        "symbol": "string",
                        "type": "PLANET",
                        "systemSymbol": "string",
                        "x": 0,
                        "y": 0
                      },
                      "departure": {
                        "symbol": "string",
                        "type": "PLANET",
                        "systemSymbol": "string",
                        "x": 0,
                        "y": 0
                      },
                      "departureTime": "2019-08-24T14:15:22Z",
                      "arrival": "2019-08-24T14:15:22Z"
                    },
                    "status": "IN_TRANSIT",
                    "flightMode": "CRUISE"
                  },
                  "crew": {
                    "current": 0,
                    "required": 0,
                    "capacity": 0,
                    "rotation": "STRICT",
                    "morale": 0,
                    "wages": 0
                  },
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
                  ],
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
                  },
                  "fuel": {
                    "current": 0,
                    "capacity": 0,
                    "consumed": {
                      "amount": 0,
                      "timestamp": "2019-08-24T14:15:22Z"
                    }
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
    fn purchase_ship() {
        let _: PurchaseShip = serde_json::from_str(
            r#"{
              "data": {
                "agent": {
                  "accountId": "string",
                  "symbol": "string",
                  "headquarters": "string",
                  "credits": 0,
                  "startingFaction": "string"
                },
                "ship": {
                  "symbol": "string",
                  "registration": {
                    "name": "string",
                    "factionSymbol": "string",
                    "role": "FABRICATOR"
                  },
                  "nav": {
                    "systemSymbol": "string",
                    "waypointSymbol": "string",
                    "route": {
                      "destination": {
                        "symbol": "string",
                        "type": "PLANET",
                        "systemSymbol": "string",
                        "x": 0,
                        "y": 0
                      },
                      "departure": {
                        "symbol": "string",
                        "type": "PLANET",
                        "systemSymbol": "string",
                        "x": 0,
                        "y": 0
                      },
                      "departureTime": "2019-08-24T14:15:22Z",
                      "arrival": "2019-08-24T14:15:22Z"
                    },
                    "status": "IN_TRANSIT",
                    "flightMode": "CRUISE"
                  },
                  "crew": {
                    "current": 0,
                    "required": 0,
                    "capacity": 0,
                    "rotation": "STRICT",
                    "morale": 0,
                    "wages": 0
                  },
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
                  ],
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
                  },
                  "fuel": {
                    "current": 0,
                    "capacity": 0,
                    "consumed": {
                      "amount": 0,
                      "timestamp": "2019-08-24T14:15:22Z"
                    }
                  }
                },
                "transaction": {
                  "waypointSymbol": "string",
                  "shipSymbol": "string",
                  "price": 0,
                  "agentSymbol": "string",
                  "timestamp": "2019-08-24T14:15:22Z"
                }
              }
            }"#,
        )
        .unwrap();
    }
    #[test]
    fn get_ship() {
        let _: Ship = serde_json::from_str(
            r#"{
              "data": {
                "symbol": "string",
                "registration": {
                  "name": "string",
                  "factionSymbol": "string",
                  "role": "FABRICATOR"
                },
                "nav": {
                  "systemSymbol": "string",
                  "waypointSymbol": "string",
                  "route": {
                    "destination": {
                      "symbol": "string",
                      "type": "PLANET",
                      "systemSymbol": "string",
                      "x": 0,
                      "y": 0
                    },
                    "departure": {
                      "symbol": "string",
                      "type": "PLANET",
                      "systemSymbol": "string",
                      "x": 0,
                      "y": 0
                    },
                    "departureTime": "2019-08-24T14:15:22Z",
                    "arrival": "2019-08-24T14:15:22Z"
                  },
                  "status": "IN_TRANSIT",
                  "flightMode": "CRUISE"
                },
                "crew": {
                  "current": 0,
                  "required": 0,
                  "capacity": 0,
                  "rotation": "STRICT",
                  "morale": 0,
                  "wages": 0
                },
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
                ],
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
                },
                "fuel": {
                  "current": 0,
                  "capacity": 0,
                  "consumed": {
                    "amount": 0,
                    "timestamp": "2019-08-24T14:15:22Z"
                  }
                }
              }
            }"#,
        )
        .unwrap();
    }
    #[test]
    fn get_ship_cargo() {
        let _: ShipCargo = serde_json::from_str(
            r#"{
              "data": {
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
            }"#,
        )
        .unwrap();
    }
    #[test]
    fn orbit_ship() {
        let _: OrbitShip = serde_json::from_str(
            r#"{
              "data": {
                "nav": {
                  "systemSymbol": "string",
                  "waypointSymbol": "string",
                  "route": {
                    "destination": {
                      "symbol": "string",
                      "type": "PLANET",
                      "systemSymbol": "string",
                      "x": 0,
                      "y": 0
                    },
                    "departure": {
                      "symbol": "string",
                      "type": "PLANET",
                      "systemSymbol": "string",
                      "x": 0,
                      "y": 0
                    },
                    "departureTime": "2019-08-24T14:15:22Z",
                    "arrival": "2019-08-24T14:15:22Z"
                  },
                  "status": "IN_TRANSIT",
                  "flightMode": "CRUISE"
                }
              }
            }"#,
        )
        .unwrap();
    }
    #[test]
    fn ship_refine() {
        let _: ShipRefine = serde_json::from_str(
            r#"{
              "data": {
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
                },
                "cooldown": {
                  "shipSymbol": "string",
                  "totalSeconds": 0,
                  "remainingSeconds": 0,
                  "expiration": "2019-08-24T14:15:22Z"
                },
                "produced": [
                  {
                    "tradeSymbol": "string",
                    "units": 0
                  }
                ],
                "consumed": [
                  {
                    "tradeSymbol": "string",
                    "units": 0
                  }
                ]
              }
            }"#,
        )
        .unwrap();
    }
    #[test]
    fn create_chart() {
        let _: CreateChart = serde_json::from_str(
            r#"{
              "data": {
                "chart": {
                  "waypointSymbol": "string",
                  "submittedBy": "string",
                  "submittedOn": "2019-08-24T14:15:22Z"
                },
                "waypoint": {
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
              }
            }"#,
        )
        .unwrap();
    }
}

// #[test]
// fn get_systems() {
//     let _: AgentL0 = serde_json::from_str(r#""#).unwrap();
// }
