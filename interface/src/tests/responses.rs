use crate::{
    enum_to_string,
    enums::{ShipType, TradeSymbol},
    requests::{BuyShip, ShipRefine},
    Method, SpaceTradersHandler,
};

// use serial_test::serial;

const TIMES_TO_RUN: i32 = 10;
const SYSTEM: &str = "X1-OE";

#[tokio::test]
async fn recieve_error() {
    let interface = SpaceTradersHandler::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        interface
            .info
            .custom_endpoint("/doesnotexist", Method::Post)
            .await;
    }
}

#[tokio::test]
async fn get_new_registration() {
    let interface = SpaceTradersHandler::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        interface
            .info
            .custom_endpoint("/register", Method::Post)
            .await;
    }
}

#[tokio::test]
// #[serial]
async fn agent() {
    let interface = SpaceTradersHandler::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        interface.agent().await;
    }
}

#[tokio::test]
// #[serial]
async fn list_systems() {
    let interface = SpaceTradersHandler::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        interface.list_systems().await;
    }
}

#[tokio::test]
// #[serial]
async fn get_systems() {
    let interface = SpaceTradersHandler::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        interface.get_system(SYSTEM).await;
    }
}

#[tokio::test]
// #[serial]
async fn list_waypoints() {
    let interface = SpaceTradersHandler::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        interface.list_waypoints(SYSTEM).await;
    }
}

#[tokio::test]
// #[serial]
async fn get_waypoint() {
    let interface = SpaceTradersHandler::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        interface.get_waypoint(SYSTEM, SYSTEM).await;
    }
}

#[tokio::test]
// #[serial]
async fn get_market() {
    let interface = SpaceTradersHandler::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        interface.get_market(SYSTEM, SYSTEM).await;
    }
}

#[tokio::test]
// #[serial]
async fn get_shipyard() {
    let interface = SpaceTradersHandler::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        interface.get_shipyard(SYSTEM, SYSTEM).await;
    }
}

#[tokio::test]
// #[serial]
async fn get_jump_gate() {
    let interface = SpaceTradersHandler::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        interface.jump_gate(SYSTEM, SYSTEM).await;
    }
}

#[tokio::test]
// #[serial]
async fn list_contracts() {
    let interface = SpaceTradersHandler::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        interface.list_contracts().await;
    }
}

#[tokio::test]
// #[serial]
async fn get_contract() {
    let interface = SpaceTradersHandler::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        interface.get_contract(SYSTEM).await;
    }
}

#[tokio::test]
// #[serial]
async fn accept_contracts() {
    let interface = SpaceTradersHandler::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        interface.accept_contract(SYSTEM).await;
    }
}

#[tokio::test]
// #[serial]
async fn deliver_contract() {
    let interface = SpaceTradersHandler::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        interface.deliver_contract(SYSTEM).await;
    }
}

#[tokio::test]
// #[serial]
async fn fulfill_contract() {
    let interface = SpaceTradersHandler::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        interface.fulfill_contract(SYSTEM).await;
    }
}

#[tokio::test]
// #[serial]
async fn list_factions() {
    let interface = SpaceTradersHandler::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        interface.list_factions().await;
    }
}

#[tokio::test]
// #[serial]
async fn get_faction() {
    let interface = SpaceTradersHandler::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        interface.get_faction(SYSTEM).await;
    }
}

#[tokio::test]
// #[serial]
async fn list_ships() {
    let interface = SpaceTradersHandler::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        interface.list_ships().await;
    }
}

#[tokio::test]
// #[serial]
async fn purchase_ship() {
    let interface = SpaceTradersHandler::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        interface
            .purchase_ship(BuyShip {
                waypointSymbol: SYSTEM.to_string(),
                shipType: enum_to_string(ShipType::ShipMiningDrone),
            })
            .await;
    }
}

#[tokio::test]
// #[serial]
async fn get_ship() {
    let interface = SpaceTradersHandler::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        interface.get_ship(SYSTEM).await;
    }
}

#[tokio::test]
// #[serial]
async fn get_ship_cargo() {
    let interface = SpaceTradersHandler::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        interface.get_ship_cargo(SYSTEM).await;
    }
}

#[tokio::test]
// #[serial]
async fn orbit_ship() {
    let interface = SpaceTradersHandler::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        interface.orbit_ship(SYSTEM).await;
    }
}

#[tokio::test]
// #[serial]
async fn ship_refine() {
    let interface = SpaceTradersHandler::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        interface
            .ship_refine(
                SYSTEM,
                ShipRefine {
                    produce: TradeSymbol::PreciousStones,
                },
            )
            .await;
    }
}

// #[tokio::test]
// // #[serial]
// fn create_chart() {
//     let interface = SpaceTradersHandler::new_testing().await;
//     for _ in 0..TIMES_TO_RUN {
//         interface.create_chart("").await;
//     }
// }

// #[test]
// #[serial]
// async fn get_systems() {
//     let interface = SpaceTradersHandler::new_testing().await;
//     for _ in 0..TIMES_TO_RUN {
//         interface.get_system("").await;
//     }
// }
