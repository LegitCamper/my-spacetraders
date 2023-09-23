#![allow(unused_must_use)]
use crate::{
    enums::{self, FlightMode, ShipType, TradeSymbol},
    requests::{
        ExtractResources, InstallMount, JettisonCargo, JumpShip, NavigateShip, PatchShipNav,
        PurchaseCargo, PurchaseShip, RemoveMount, SellCargo, ShipRefine, TransferCargo, WarpShip,
    },
    responses::schemas::SurveyDeposit,
    tests::log,
    Method, SpaceTraders, SystemString, WaypointString,
};

use once_cell::sync::Lazy;
use std::sync::Mutex;

const TIMES_TO_RUN: i32 = 10;
const STRING: &str = "X1-OE";

static SPACETRADERS: Lazy<Mutex<SpaceTraders>> =
    Lazy::new(|| Mutex::new(SpaceTraders::new_testing()));

#[tokio::test]
async fn recieve_error() {
    log();
    for _ in 0..TIMES_TO_RUN {
        SPACETRADERS
            .lock()
            .unwrap()
            .custom_endpoint(Method::Post, "/doesnotexist", None)
            .await;
    }
}

#[tokio::test]
async fn get_new_registration() {
    log();
    for _ in 0..TIMES_TO_RUN {
        SPACETRADERS
            .lock()
            .unwrap()
            .custom_endpoint(
                Method::Post,
                "/register",
                Some(crate::requests::Requests::RegisterNewAgent(
                    crate::requests::RegisterNewAgent {
                        faction: enums::FactionSymbols::Cosmic,
                        symbol: "placeholder".to_string(),
                        email: None,
                    },
                )),
            )
            .await
            .unwrap();
    }
}

#[tokio::test]
async fn get_status() {
    log();
    for _ in 0..TIMES_TO_RUN {
        SPACETRADERS.lock().unwrap().get_status().await.unwrap();
    }
}

#[tokio::test]
async fn agent() {
    log();
    for _ in 0..TIMES_TO_RUN {
        SPACETRADERS.lock().unwrap().agent().await.unwrap();
    }
}

#[tokio::test]
async fn list_systems() {
    log();
    for _ in 0..TIMES_TO_RUN {
        SPACETRADERS
            .lock()
            .unwrap()
            .list_systems(true)
            .await
            .unwrap();
    }
}
#[tokio::test]
async fn get_systems() {
    log();
    for _ in 0..TIMES_TO_RUN {
        SPACETRADERS
            .lock()
            .unwrap()
            .get_system(&SystemString {
                system: STRING.to_string(),
                sector: STRING.to_string(),
            })
            .await
            .unwrap();
    }
}
#[tokio::test]
async fn list_waypoints() {
    log();
    for _ in 0..TIMES_TO_RUN {
        SPACETRADERS
            .lock()
            .unwrap()
            .list_waypoints(
                &SystemString {
                    system: STRING.to_string(),
                    sector: STRING.to_string(),
                },
                true,
            )
            .await
            .unwrap();
    }
}
#[tokio::test]
async fn get_waypoint() {
    log();
    for _ in 0..TIMES_TO_RUN {
        SPACETRADERS
            .lock()
            .unwrap()
            .get_waypoint(
                &SystemString {
                    system: STRING.to_string(),
                    sector: STRING.to_string(),
                },
                &WaypointString {
                    waypoint: STRING.to_string(),
                    system: STRING.to_string(),
                    sector: STRING.to_string(),
                },
            )
            .await
            .unwrap();
    }
}
#[tokio::test]
async fn get_market() {
    log();
    for _ in 0..TIMES_TO_RUN {
        SPACETRADERS
            .lock()
            .unwrap()
            .get_market(
                &SystemString {
                    system: STRING.to_string(),
                    sector: STRING.to_string(),
                },
                &WaypointString {
                    waypoint: STRING.to_string(),
                    system: STRING.to_string(),
                    sector: STRING.to_string(),
                },
            )
            .await
            .unwrap();
    }
}
#[tokio::test]
async fn get_shipyard() {
    log();
    for _ in 0..TIMES_TO_RUN {
        SPACETRADERS
            .lock()
            .unwrap()
            .get_shipyard(
                &SystemString {
                    system: STRING.to_string(),
                    sector: STRING.to_string(),
                },
                &WaypointString {
                    waypoint: STRING.to_string(),
                    system: STRING.to_string(),
                    sector: STRING.to_string(),
                },
            )
            .await
            .unwrap();
    }
}
#[tokio::test]
async fn get_jump_gate() {
    log();
    for _ in 0..TIMES_TO_RUN {
        SPACETRADERS
            .lock()
            .unwrap()
            .jump_gate(&WaypointString {
                waypoint: STRING.to_string(),
                system: STRING.to_string(),
                sector: STRING.to_string(),
            })
            .await
            .unwrap();
    }
}

#[tokio::test]
async fn list_contracts() {
    log();
    for _ in 0..TIMES_TO_RUN {
        SPACETRADERS
            .lock()
            .unwrap()
            .list_contracts(true)
            .await
            .unwrap();
    }
}
#[tokio::test]
async fn get_contract() {
    log();
    for _ in 0..TIMES_TO_RUN {
        SPACETRADERS
            .lock()
            .unwrap()
            .get_contract(STRING)
            .await
            .unwrap();
    }
}
#[tokio::test]
async fn accept_contracts() {
    log();
    for _ in 0..TIMES_TO_RUN {
        SPACETRADERS
            .lock()
            .unwrap()
            .accept_contract(STRING)
            .await
            .unwrap();
    }
}
#[tokio::test]
async fn deliver_contract() {
    log();
    for _ in 0..TIMES_TO_RUN {
        SPACETRADERS
            .lock()
            .unwrap()
            .deliver_contract(
                STRING,
                crate::requests::DeliverCargoToContract {
                    ship_symbol: ShipType::ShipProbe,
                    trade_symbol: TradeSymbol::PreciousStones,
                    units: 1000,
                },
            )
            .await
            .unwrap();
    }
}
#[tokio::test]
async fn fulfill_contract() {
    log();
    for _ in 0..TIMES_TO_RUN {
        SPACETRADERS
            .lock()
            .unwrap()
            .fulfill_contract(STRING)
            .await
            .unwrap();
    }
}

#[tokio::test]
async fn list_factions() {
    log();
    for _ in 0..TIMES_TO_RUN {
        SPACETRADERS.lock().unwrap().list_factions().await.unwrap();
    }
}
#[tokio::test]
async fn get_faction() {
    log();
    for _ in 0..TIMES_TO_RUN {
        SPACETRADERS
            .lock()
            .unwrap()
            .get_faction(STRING)
            .await
            .unwrap();
    }
}

#[tokio::test]
async fn list_ships() {
    log();
    for _ in 0..TIMES_TO_RUN {
        SPACETRADERS.lock().unwrap().list_ships().await.unwrap();
    }
}
#[tokio::test]
async fn purchase_ship() {
    log();
    for _ in 0..TIMES_TO_RUN {
        SPACETRADERS
            .lock()
            .unwrap()
            .purchase_ship(PurchaseShip {
                waypoint_symbol: STRING.to_string(),
                ship_type: ShipType::ShipMiningDrone,
            })
            .await
            .unwrap();
    }
}
#[tokio::test]
async fn get_ship() {
    log();
    for _ in 0..TIMES_TO_RUN {
        SPACETRADERS.lock().unwrap().get_ship(STRING).await.unwrap();
    }
}
#[tokio::test]
async fn get_ship_cargo() {
    log();
    for _ in 0..TIMES_TO_RUN {
        SPACETRADERS
            .lock()
            .unwrap()
            .get_ship_cargo(STRING)
            .await
            .unwrap();
    }
}
#[tokio::test]
async fn orbit_ship() {
    log();
    for _ in 0..TIMES_TO_RUN {
        SPACETRADERS
            .lock()
            .unwrap()
            .orbit_ship(STRING)
            .await
            .unwrap();
    }
}
#[tokio::test]
async fn ship_refine() {
    log();
    for _ in 0..TIMES_TO_RUN {
        SPACETRADERS
            .lock()
            .unwrap()
            .ship_refine(
                "fsd",
                ShipRefine {
                    produce: TradeSymbol::Iron,
                },
            )
            .await
            .unwrap();
    }
}
#[tokio::test]
async fn create_chart() {
    log();
    for _ in 0..TIMES_TO_RUN {
        SPACETRADERS
            .lock()
            .unwrap()
            .create_chart(STRING)
            .await
            .unwrap();
    }
}
#[tokio::test]
async fn get_ship_cooldown() {
    log();
    for _ in 0..TIMES_TO_RUN {
        SPACETRADERS
            .lock()
            .unwrap()
            .get_ship_cooldown(STRING)
            .await
            .unwrap();
    }
}
#[tokio::test]
async fn dock_ship() {
    log();
    for _ in 0..TIMES_TO_RUN {
        SPACETRADERS
            .lock()
            .unwrap()
            .dock_ship(STRING)
            .await
            .unwrap();
    }
}
#[tokio::test]
async fn create_survey() {
    log();
    for _ in 0..TIMES_TO_RUN {
        SPACETRADERS
            .lock()
            .unwrap()
            .create_survey(STRING)
            .await
            .unwrap();
    }
}
#[tokio::test]
async fn extract_resources() {
    log();
    for _ in 0..TIMES_TO_RUN {
        SPACETRADERS
            .lock()
            .unwrap()
            .extract_resources(
                STRING,
                Some(ExtractResources {
                    signature: STRING.into(),
                    symbol: STRING.into(),
                    deposits: vec![
                        (SurveyDeposit {
                            symbol: enums::TradeSymbol::Aluminum,
                        }),
                    ],
                    // TODO: make into datatime
                    expiration: chrono::Local::now(),
                    size: enums::DepositSize::Small,
                }),
            )
            .await
            .unwrap();
    }
}
#[tokio::test]
async fn jettison_cargo() {
    log();
    for _ in 0..TIMES_TO_RUN {
        SPACETRADERS
            .lock()
            .unwrap()
            .jettison_cargo(
                STRING,
                JettisonCargo {
                    symbol: enums::TradeSymbol::PreciousStones,
                    units: 1000,
                },
            )
            .await
            .unwrap();
    }
}
#[tokio::test]
async fn jump_ship() {
    log();
    for _ in 0..TIMES_TO_RUN {
        SPACETRADERS
            .lock()
            .unwrap()
            .jump_ship(
                STRING,
                JumpShip {
                    system_symbol: STRING.to_string(),
                },
            )
            .await
            .unwrap();
    }
}
#[tokio::test]
async fn navigate_ship() {
    log();
    for _ in 0..TIMES_TO_RUN {
        SPACETRADERS
            .lock()
            .unwrap()
            .navigate_ship(
                STRING,
                NavigateShip {
                    waypoint_symbol: STRING.to_string(),
                },
            )
            .await
            .unwrap();
    }
}
#[tokio::test]
async fn patch_ship_nav() {
    log();
    for _ in 0..TIMES_TO_RUN {
        SPACETRADERS
            .lock()
            .unwrap()
            .patch_ship_nav(
                STRING,
                PatchShipNav {
                    ship_symbol: FlightMode::Cruise,
                },
            )
            .await
            .unwrap();
    }
}
#[tokio::test]
async fn get_ship_nav() {
    log();
    for _ in 0..TIMES_TO_RUN {
        SPACETRADERS
            .lock()
            .unwrap()
            .get_ship_nav(STRING)
            .await
            .unwrap();
    }
}
#[tokio::test]
async fn warp_ship() {
    log();
    for _ in 0..TIMES_TO_RUN {
        SPACETRADERS
            .lock()
            .unwrap()
            .warp_ship(
                STRING,
                WarpShip {
                    waypoint_symbol: STRING.to_string(),
                },
            )
            .await
            .unwrap();
    }
}
#[tokio::test]
async fn sell_cargo() {
    log();
    for _ in 0..TIMES_TO_RUN {
        SPACETRADERS
            .lock()
            .unwrap()
            .sell_cargo(
                STRING,
                SellCargo {
                    symbol: TradeSymbol::PreciousStones,
                    units: 1000,
                },
            )
            .await
            .unwrap();
    }
}
#[tokio::test]
async fn scan_systems() {
    log();
    for _ in 0..TIMES_TO_RUN {
        SPACETRADERS
            .lock()
            .unwrap()
            .scan_systems(STRING)
            .await
            .unwrap();
    }
}
#[tokio::test]
async fn scan_waypoints() {
    log();
    for _ in 0..TIMES_TO_RUN {
        SPACETRADERS
            .lock()
            .unwrap()
            .scan_waypoints(STRING)
            .await
            .unwrap();
    }
}
#[tokio::test]
async fn scan_ships() {
    log();
    for _ in 0..TIMES_TO_RUN {
        SPACETRADERS
            .lock()
            .unwrap()
            .scan_ships(STRING)
            .await
            .unwrap();
    }
}
#[tokio::test]
async fn purchase_cargo() {
    log();
    for _ in 0..TIMES_TO_RUN {
        SPACETRADERS
            .lock()
            .unwrap()
            .purchase_cargo(
                STRING,
                PurchaseCargo {
                    symbol: enums::TradeSymbol::PreciousStones,
                    units: 1000,
                },
            )
            .await
            .unwrap();
    }
}
#[tokio::test]
async fn transfer_cargo() {
    log();
    for _ in 0..TIMES_TO_RUN {
        SPACETRADERS
            .lock()
            .unwrap()
            .transfer_cargo(
                STRING,
                TransferCargo {
                    trade_symbol: TradeSymbol::PreciousStones,
                    units: 1000,
                    ship_symbol: STRING.into(),
                },
            )
            .await
            .unwrap();
    }
}
#[tokio::test]
async fn negotiate_contract() {
    log();
    for _ in 0..TIMES_TO_RUN {
        SPACETRADERS
            .lock()
            .unwrap()
            .negotiate_contract(STRING)
            .await
            .unwrap();
    }
}
#[tokio::test]
async fn get_mounts() {
    log();
    for _ in 0..TIMES_TO_RUN {
        SPACETRADERS
            .lock()
            .unwrap()
            .get_mounts(STRING)
            .await
            .unwrap();
    }
}
#[tokio::test]
async fn install_mount() {
    log();
    for _ in 0..TIMES_TO_RUN {
        SPACETRADERS
            .lock()
            .unwrap()
            .install_mount(
                STRING,
                InstallMount {
                    symbol: STRING.to_string(),
                },
            )
            .await
            .unwrap();
    }
}
#[tokio::test]
async fn remove_mount() {
    log();
    for _ in 0..TIMES_TO_RUN {
        SPACETRADERS
            .lock()
            .unwrap()
            .remove_mount(
                STRING,
                RemoveMount {
                    symbol: STRING.to_string(),
                },
            )
            .await
            .unwrap();
    }
}
