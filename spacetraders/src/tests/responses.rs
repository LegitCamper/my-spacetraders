#![allow(unused_must_use)]
use crate::{
    enums::{self, FlightMode, ShipType, TradeSymbol},
    requests::{
        ExtractResources, InstallMount, JettisonCargo, JumpShip, NavigateShip, PatchShipNav,
        PurchaseCargo, PurchaseShip, RemoveMount, SellCargo, ShipRefine, TransferCargo, WarpShip,
    },
    responses::schemas::SurveyDeposit,
    Method, SpaceTraders, SystemString, WaypointString,
    tests::log,
};

use serial_test::serial;

const TIMES_TO_RUN: i32 = 10;
const STRING: &str = "X1-OE";

#[tokio::test]
#[serial]
async fn recieve_error() {
    log();
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders
            .interface
            .custom_endpoint(Method::Post, "/doesnotexist", None)
            .await;
    }
}

#[tokio::test]
#[serial]
async fn get_new_registration() {
    log();
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders
            .interface
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
#[serial]
async fn get_status() {
    log();
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders.get_status().await.unwrap();
    }
}

#[tokio::test]
#[serial]
async fn agent() {
    log();
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders.agent().await.unwrap();
    }
}

#[tokio::test]
#[serial]
async fn list_systems() {
    log();
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders.list_systems(true).await.unwrap();
    }
}
#[tokio::test]
#[serial]
async fn get_systems() {
    log();
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders
            .get_system(&SystemString {
                system: STRING.to_string(),
                sector: STRING.to_string(),
            })
            .await
            .unwrap();
    }
}
#[tokio::test]
#[serial]
async fn list_waypoints() {
    log();
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders
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
#[serial]
async fn get_waypoint() {
    log();
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders
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
#[serial]
async fn get_market() {
    log();
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders
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
#[serial]
async fn get_shipyard() {
    log();
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders
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
#[serial]
async fn get_jump_gate() {
    log();
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders
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
#[serial]
async fn list_contracts() {
    log();
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders.list_contracts(true).await.unwrap();
    }
}
#[tokio::test]
#[serial]
async fn get_contract() {
    log();
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders.get_contract(STRING).await.unwrap();
    }
}
#[tokio::test]
#[serial]
async fn accept_contracts() {
    log();
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders.accept_contract(STRING).await.unwrap();
    }
}
#[tokio::test]
#[serial]
async fn deliver_contract() {
    log();
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders
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
#[serial]
async fn fulfill_contract() {
    log();
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders.fulfill_contract(STRING).await.unwrap();
    }
}

#[tokio::test]
#[serial]
async fn list_factions() {
    log();
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders.list_factions().await.unwrap();
    }
}
#[tokio::test]
#[serial]
async fn get_faction() {
    log();
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders.get_faction(STRING).await.unwrap();
    }
}

#[tokio::test]
#[serial]
async fn list_ships() {
    log();
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders.list_ships().await.unwrap();
    }
}
#[tokio::test]
#[serial]
async fn purchase_ship() {
    log();
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders
            .purchase_ship(PurchaseShip {
                waypoint_symbol: STRING.to_string(),
                ship_type: ShipType::ShipMiningDrone,
            })
            .await
            .unwrap();
    }
}
#[tokio::test]
#[serial]
async fn get_ship() {
    log();
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders.get_ship(STRING).await.unwrap();
    }
}
#[tokio::test]
#[serial]
async fn get_ship_cargo() {
    log();
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders.get_ship_cargo(STRING).await.unwrap();
    }
}
#[tokio::test]
#[serial]
async fn orbit_ship() {
    log();
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders.orbit_ship(STRING).await.unwrap();
    }
}
#[tokio::test]
#[serial]
async fn ship_refine() {
    log();
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders
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
#[serial]
async fn create_chart() {
    log();
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders.create_chart(STRING).await.unwrap();
    }
}
#[tokio::test]
#[serial]
async fn get_ship_cooldown() {
    log();
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders.get_ship_cooldown(STRING).await.unwrap();
    }
}
#[tokio::test]
#[serial]
async fn dock_ship() {
    log();
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders.dock_ship(STRING).await.unwrap();
    }
}
#[tokio::test]
#[serial]
async fn create_survey() {
    log();
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders.create_survey(STRING).await.unwrap();
    }
}
#[tokio::test]
#[serial]
async fn extract_resources() {
    log();
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders
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
                    expiration: chrono::offset::Utc::now(),
                    size: enums::DepositSize::Small,
                }),
            )
            .await
            .unwrap();
    }
}
#[tokio::test]
#[serial]
async fn jettison_cargo() {
    log();
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders
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
#[serial]
async fn jump_ship() {
    log();
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders
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
#[serial]
async fn navigate_ship() {
    log();
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders
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
#[serial]
async fn patch_ship_nav() {
    log();
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders
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
#[serial]
async fn get_ship_nav() {
    log();
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders.get_ship_nav(STRING).await.unwrap();
    }
}
#[tokio::test]
#[serial]
async fn warp_ship() {
    log();
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders
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
#[serial]
async fn sell_cargo() {
    log();
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders
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
#[serial]
async fn scan_systems() {
    log();
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders.scan_systems(STRING).await.unwrap();
    }
}
#[tokio::test]
#[serial]
async fn scan_waypoints() {
    log();
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders.scan_waypoints(STRING).await.unwrap();
    }
}
#[tokio::test]
#[serial]
async fn scan_ships() {
    log();
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders.scan_ships(STRING).await.unwrap();
    }
}
#[tokio::test]
#[serial]
async fn purchase_cargo() {
    log();
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders
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
#[serial]
async fn transfer_cargo() {
    log();
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders
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
#[serial]
async fn negotiate_contract() {
    log();
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders.negotiate_contract(STRING).await.unwrap();
    }
}
#[tokio::test]
#[serial]
async fn get_mounts() {
    log();
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders.get_mounts(STRING).await.unwrap();
    }
}
#[tokio::test]
#[serial]
async fn install_mount() {
    log();
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders
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
#[serial]
async fn remove_mount() {
    log();
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders
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
