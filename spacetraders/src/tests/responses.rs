use crate::{
    enums::{self, FlightMode, ShipType, TradeSymbol},
    requests::{
        ExtractResources, InstallMount, JettisonCargo, JumpShip, NavigateShip, PatchShipNav,
        PurchaseCargo, PurchaseShip, RemoveMount, SellCargo, ShipRefine, TransferCargo, WarpShip,
    },
    responses::schemas::SurveyDeposit,
    Method, SpaceTraders, SystemString, WaypointString,
};

use serial_test::serial;

const TIMES_TO_RUN: i32 = 10;
const STRING: &str = "X1-OE";

// This test panics - duh
// #[tokio::test]
// #[serial]
// async fn recieve_error() {
//     for _ in 0..TIMES_TO_RUN {
//             .await
//             .spacetraders
//             .custom_endpoint(Method::Post, "/doesnotexist", None)
//             .await;
//     }
// }

#[tokio::test]
#[serial]
async fn get_new_registration() {
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
            .await;
    }
}

#[tokio::test]
#[serial]
async fn get_status() {
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        print!("{:?}", spacetraders.get_status().await);
    }
}

#[tokio::test]
#[serial]
async fn agent() {
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        print!("{:?}", spacetraders.agent().await);
    }
}

#[tokio::test]
#[serial]
async fn list_systems() {
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders.list_systems(None).await;
    }
}
#[tokio::test]
#[serial]
async fn get_systems() {
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders
            .get_system(&SystemString {
                system: STRING.to_string(),
                sector: STRING.to_string(),
            })
            .await;
    }
}
#[tokio::test]
#[serial]
async fn list_waypoints() {
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders
            .list_waypoints(
                &SystemString {
                    system: STRING.to_string(),
                    sector: STRING.to_string(),
                },
                None,
            )
            .await;
    }
}
#[tokio::test]
#[serial]
async fn get_waypoint() {
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
            .await;
    }
}
#[tokio::test]
#[serial]
async fn get_market() {
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
            .await;
    }
}
#[tokio::test]
#[serial]
async fn get_shipyard() {
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
            .await;
    }
}
#[tokio::test]
#[serial]
async fn get_jump_gate() {
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders
            .jump_gate(
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
            .await;
    }
}

#[tokio::test]
#[serial]
async fn list_contracts() {
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders.list_contracts(None).await;
    }
}
#[tokio::test]
#[serial]
async fn get_contract() {
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders.get_contract(STRING).await;
    }
}
#[tokio::test]
#[serial]
async fn accept_contracts() {
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders.accept_contract(STRING).await;
    }
}
#[tokio::test]
#[serial]
async fn deliver_contract() {
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
            .await;
    }
}
#[tokio::test]
#[serial]
async fn fulfill_contract() {
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders.fulfill_contract(STRING).await;
    }
}

#[tokio::test]
#[serial]
async fn list_factions() {
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders.list_factions().await;
    }
}
#[tokio::test]
#[serial]
async fn get_faction() {
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders.get_faction(STRING).await;
    }
}

#[tokio::test]
#[serial]
async fn list_ships() {
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders.list_ships().await;
    }
}
#[tokio::test]
#[serial]
async fn purchase_ship() {
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders
            .purchase_ship(PurchaseShip {
                waypoint_symbol: STRING.to_string(),
                ship_type: ShipType::ShipMiningDrone,
            })
            .await;
    }
}
#[tokio::test]
#[serial]
async fn get_ship() {
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders.get_ship(STRING).await;
    }
}
#[tokio::test]
#[serial]
async fn get_ship_cargo() {
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders.get_ship_cargo(STRING).await;
    }
}
#[tokio::test]
#[serial]
async fn orbit_ship() {
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders.orbit_ship(STRING).await;
    }
}
#[tokio::test]
#[serial]
async fn ship_refine() {
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders
            .ship_refine(
                "fsd",
                ShipRefine {
                    produce: TradeSymbol::Iron,
                },
            )
            .await;
    }
}
#[tokio::test]
#[serial]
async fn create_chart() {
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders.create_chart(STRING).await;
    }
}
#[tokio::test]
#[serial]
async fn get_ship_cooldown() {
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders.get_ship_cooldown(STRING).await;
    }
}
#[tokio::test]
#[serial]
async fn dock_ship() {
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders.dock_ship(STRING).await;
    }
}
#[tokio::test]
#[serial]
async fn create_survey() {
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders.create_survey(STRING).await;
    }
}
#[tokio::test]
#[serial]
async fn extract_resources() {
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
                            symbol: STRING.to_string(),
                        }),
                    ],
                    // TODO: make into datatime
                    expiration: chrono::offset::Utc::now(),
                    size: enums::DepositSize::Small,
                }),
            )
            .await;
    }
}
#[tokio::test]
#[serial]
async fn jettison_cargo() {
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
            .await;
    }
}
#[tokio::test]
#[serial]
async fn jump_ship() {
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders
            .jump_ship(
                STRING,
                JumpShip {
                    system_symbol: STRING.to_string(),
                },
            )
            .await;
    }
}
#[tokio::test]
#[serial]
async fn navigate_ship() {
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders
            .navigate_ship(
                STRING,
                NavigateShip {
                    waypoint_symbol: STRING.to_string(),
                },
            )
            .await;
    }
}
#[tokio::test]
#[serial]
async fn patch_ship_nav() {
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders
            .patch_ship_nav(
                STRING,
                PatchShipNav {
                    ship_symbol: FlightMode::Cruise,
                },
            )
            .await;
    }
}
#[tokio::test]
#[serial]
async fn get_ship_nav() {
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders.get_ship_nav(STRING).await;
    }
}
#[tokio::test]
#[serial]
async fn warp_ship() {
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders
            .warp_ship(
                STRING,
                WarpShip {
                    ship_symbol: STRING.to_string(),
                },
            )
            .await;
    }
}
#[tokio::test]
#[serial]
async fn sell_cargo() {
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
            .await;
    }
}
#[tokio::test]
#[serial]
async fn scan_systems() {
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders.scan_systems(STRING).await;
    }
}
#[tokio::test]
#[serial]
async fn scan_waypoints() {
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders.scan_waypoints(STRING).await;
    }
}
#[tokio::test]
#[serial]
async fn scan_ships() {
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders.scan_ships(STRING).await;
    }
}
#[tokio::test]
#[serial]
async fn purchase_cargo() {
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
            .await;
    }
}
#[tokio::test]
#[serial]
async fn transfer_cargo() {
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
            .await;
    }
}
#[tokio::test]
#[serial]
async fn negotiate_contract() {
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders.negotiate_contract(STRING).await;
    }
}
#[tokio::test]
#[serial]
async fn get_mounts() {
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders.get_mounts(STRING).await;
    }
}
#[tokio::test]
#[serial]
async fn install_mount() {
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders
            .install_mount(
                STRING,
                InstallMount {
                    symbol: STRING.to_string(),
                },
            )
            .await;
    }
}
#[tokio::test]
#[serial]
async fn remove_mount() {
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders
            .remove_mount(
                STRING,
                RemoveMount {
                    symbol: STRING.to_string(),
                },
            )
            .await;
    }
}
