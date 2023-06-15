use serde::Deserialize;

#[cfg(test)]
pub mod error {
    use crate::interface::responses::Error;

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
pub mod registration {
    use crate::interface::responses::GetRegistrationL0;

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
      "symbol": "COSMIC",
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
pub mod agent {
    use crate::interface::SpaceTradersHandler;

    #[tokio::test]
    async fn agent() {
        let interface = SpaceTradersHandler::default().await;
        let _ = interface.agent().await;
    }
}

#[cfg(test)]
pub mod system {
    use crate::interface::SpaceTradersHandler;

    #[tokio::test]
    async fn list_systems() {
        let interface = SpaceTradersHandler::default().await;
        let _ = interface.list_systems().await;
    }
    #[tokio::test]
    async fn get_systems() {
        let interface = SpaceTradersHandler::default().await;
        let _ = interface.get_system("").await;
    }
    #[tokio::test]
    async fn list_waypoints() {
        let interface = SpaceTradersHandler::default().await;
        let _ = interface.list_waypoints("").await;
    }
    #[tokio::test]
    async fn get_waypoint() {
        let interface = SpaceTradersHandler::default().await;
        let _ = interface.get_waypoint("", "").await;
    }
    #[tokio::test]
    async fn get_market() {
        let interface = SpaceTradersHandler::default().await;
        let _ = interface.get_market("", "").await;
    }
    #[tokio::test]
    async fn get_shipyard() {
        let interface = SpaceTradersHandler::default().await;
        let _ = interface.get_shipyard("", "").await;
    }
    #[tokio::test]
    async fn get_jump_gate() {
        let interface = SpaceTradersHandler::default().await;
        let _ = interface.jump_gate("", "").await;
    }
}

#[cfg(test)]
pub mod contract {
    use crate::interface::contracts::{
        AcceptContract, Contract, Contracts, DeliverContract, FulfillContract,
    };

    #[test]
    fn list_contracts() {
        let _: Contracts = serde_json::from_str().unwrap();
    }
    #[test]
    fn get_contracts() {
        let _: Contract = serde_json::from_str().unwrap();
    }
    #[test]
    fn accept_contracts() {
        let _: AcceptContract = serde_json::from_str().unwrap();
    }
    #[test]
    fn deliver_contract() {
        let _: DeliverContract = serde_json::from_str().unwrap();
    }
    #[test]
    fn fulfill_contract() {
        let _: FulfillContract = serde_json::from_str().unwrap();
    }
}

#[cfg(test)]
pub mod faction {
    use super::factions::{Faction, Factions};

    #[test]
    fn list_factions() {
        let _: Factions = serde_json::from_str().unwrap();
    }

    #[test]
    fn get_factions() {
        let _: Faction = serde_json::from_str().unwrap();
    }
}

#[cfg(test)]
pub mod fleet {
    use crate::interface::fleet::{
        CreateChart, OrbitShip, PurchaseShip, Ship, ShipCargo, ShipRefine, Ships,
    };

    #[test]
    fn list_ships() {
        let _: Ships = serde_json::from_str().unwrap();
    }
    #[test]
    fn purchase_ship() {
        let _: PurchaseShip = serde_json::from_str().unwrap();
    }
    #[test]
    fn get_ship() {
        let _: Ship = serde_json::from_str().unwrap();
    }
    #[test]
    fn get_ship_cargo() {
        let _: ShipCargo = serde_json::from_str().unwrap();
    }
    #[test]
    fn orbit_ship() {
        let _: OrbitShip = serde_json::from_str().unwrap();
    }
    #[test]
    fn ship_refine() {
        let _: ShipRefine = serde_json::from_str().unwrap();
    }
    #[test]
    fn create_chart() {
        let _: CreateChart = serde_json::from_str().unwrap();
    }
}

// #[test]
// fn get_systems() {
//     let _: AgentL0 = serde_json::from_str(r#""#).unwrap();
// }
