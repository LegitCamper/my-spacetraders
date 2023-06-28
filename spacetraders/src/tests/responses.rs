use crate::{
    enums::{self, FlightMode, ShipType, TradeSymbol},
    requests::{
        ExtractResources, ExtractResourcesSurvey, ExtractResourcesSurveyDeposits, InstallMount,
        JettisonCargo, JumpShip, NavigateShip, PatchShipNav, PurchaseCargo, PurchaseShip,
        RemoveMount, SellCargo, ShipRefine, TransferCargo, WarpShip,
    },
    Method, SpaceTraders,
};

use async_once::AsyncOnce;
use lazy_static::lazy_static;
use serial_test::serial;

const TIMES_TO_RUN: i32 = 10;
const SYSTEM: &str = "X1-OE";

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
                        email: "".to_string(),
                    },
                )),
            )
            .await;
    }
}

#[tokio::test]
#[serial]
async fn agent() {
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders.agent().await;
    }
}

#[tokio::test]
#[serial]
async fn list_systems() {
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders.list_systems().await;
    }
}
#[tokio::test]
#[serial]
async fn get_systems() {
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders.get_system(SYSTEM).await;
    }
}
#[tokio::test]
#[serial]
async fn list_waypoints() {
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders.list_waypoints(SYSTEM).await;
    }
}
#[tokio::test]
#[serial]
async fn get_waypoint() {
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders.get_waypoint(SYSTEM, SYSTEM).await;
    }
}
#[tokio::test]
#[serial]
async fn get_market() {
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders.get_market(SYSTEM, SYSTEM).await;
    }
}
#[tokio::test]
#[serial]
async fn get_shipyard() {
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders.get_shipyard(SYSTEM, SYSTEM).await;
    }
}
#[tokio::test]
#[serial]
async fn get_jump_gate() {
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders.jump_gate(SYSTEM, SYSTEM).await;
    }
}

#[tokio::test]
#[serial]
async fn list_contracts() {
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders.list_contracts().await;
    }
}
#[tokio::test]
#[serial]
async fn get_contract() {
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders.get_contract(SYSTEM).await;
    }
}
#[tokio::test]
#[serial]
async fn accept_contracts() {
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders.accept_contract(SYSTEM).await;
    }
}
#[tokio::test]
#[serial]
async fn deliver_contract() {
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders
            .deliver_contract(
                SYSTEM,
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
        spacetraders.fulfill_contract(SYSTEM).await;
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
        spacetraders.get_faction(SYSTEM).await;
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
                waypoint_symbol: SYSTEM.to_string(),
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
        spacetraders.get_ship(SYSTEM).await;
    }
}
#[tokio::test]
#[serial]
async fn get_ship_cargo() {
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders.get_ship_cargo(SYSTEM).await;
    }
}
#[tokio::test]
#[serial]
async fn orbit_ship() {
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders.orbit_ship(SYSTEM).await;
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
        spacetraders.create_chart(SYSTEM).await;
    }
}
#[tokio::test]
#[serial]
async fn get_ship_cooldown() {
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders.get_ship_cooldown(SYSTEM).await;
    }
}
#[tokio::test]
#[serial]
async fn dock_ship() {
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders.dock_ship(SYSTEM).await;
    }
}
#[tokio::test]
#[serial]
async fn create_survey() {
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders.create_survey(SYSTEM).await;
    }
}
#[tokio::test]
#[serial]
async fn extract_resources() {
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders
            .extract_resources(
                SYSTEM,
                ExtractResources {
                    survey: ExtractResourcesSurvey {
                        signature: SYSTEM.into(),
                        symbol: SYSTEM.into(),
                        deposits: vec![ExtractResourcesSurveyDeposits {
                            symbol: SYSTEM.into(),
                        }],
                        // TODO: make into datatime
                        expiration: "2019-08-24T14:15:22Z".into(),
                        size: enums::DepositSize::Small,
                    },
                },
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
                SYSTEM,
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
                SYSTEM,
                JumpShip {
                    system_symbol: SYSTEM.to_string(),
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
                SYSTEM,
                NavigateShip {
                    ship_symbol: ShipType::ShipProbe,
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
                SYSTEM,
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
        spacetraders.get_ship_nav(SYSTEM).await;
    }
}
#[tokio::test]
#[serial]
async fn warp_ship() {
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders
            .warp_ship(
                SYSTEM,
                WarpShip {
                    ship_symbol: SYSTEM.to_string(),
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
                SYSTEM,
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
        spacetraders.scan_systems(SYSTEM).await;
    }
}
#[tokio::test]
#[serial]
async fn scan_waypoints() {
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders.scan_waypoints(SYSTEM).await;
    }
}
#[tokio::test]
#[serial]
async fn scan_ships() {
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders.scan_ships(SYSTEM).await;
    }
}
#[tokio::test]
#[serial]
async fn purchase_cargo() {
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders
            .purchase_cargo(
                SYSTEM,
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
                SYSTEM,
                TransferCargo {
                    trade_symbol: TradeSymbol::PreciousStones,
                    units: 1000,
                    ship_symbol: SYSTEM.into(),
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
        spacetraders.negotiate_contract(SYSTEM).await;
    }
}
#[tokio::test]
#[serial]
async fn get_mounts() {
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders.get_mounts(SYSTEM).await;
    }
}
#[tokio::test]
#[serial]
async fn install_mount() {
    let spacetraders = SpaceTraders::testing().await;
    for _ in 0..TIMES_TO_RUN {
        spacetraders
            .install_mount(
                SYSTEM,
                InstallMount {
                    symbol: SYSTEM.to_string(),
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
                SYSTEM,
                RemoveMount {
                    symbol: SYSTEM.to_string(),
                },
            )
            .await;
    }
}
