use crate::{
    enums::{FlightMode, ShipType, TradeSymbol},
    requests::{
        BuyShip, InstallMount, JumpShip, NavigateShip, PatchShipNav, RemoveMount, ShipRefine,
        WarpShip,
    },
    Method, SpaceTradersHandler,
};

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
async fn agent() {
    let interface = SpaceTradersHandler::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        interface.agent().await;
    }
}

#[tokio::test]
async fn list_systems() {
    let interface = SpaceTradersHandler::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        interface.list_systems().await;
    }
}
#[tokio::test]
async fn get_systems() {
    let interface = SpaceTradersHandler::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        interface.get_system(SYSTEM).await;
    }
}
#[tokio::test]
async fn list_waypoints() {
    let interface = SpaceTradersHandler::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        interface.list_waypoints(SYSTEM).await;
    }
}
#[tokio::test]
async fn get_waypoint() {
    let interface = SpaceTradersHandler::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        interface.get_waypoint(SYSTEM, SYSTEM).await;
    }
}
#[tokio::test]
async fn get_market() {
    let interface = SpaceTradersHandler::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        interface.get_market(SYSTEM, SYSTEM).await;
    }
}
#[tokio::test]
async fn get_shipyard() {
    let interface = SpaceTradersHandler::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        interface.get_shipyard(SYSTEM, SYSTEM).await;
    }
}
#[tokio::test]
async fn get_jump_gate() {
    let interface = SpaceTradersHandler::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        interface.jump_gate(SYSTEM, SYSTEM).await;
    }
}

#[tokio::test]
async fn list_contracts() {
    let interface = SpaceTradersHandler::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        interface.list_contracts().await;
    }
}
#[tokio::test]
async fn get_contract() {
    let interface = SpaceTradersHandler::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        interface.get_contract(SYSTEM).await;
    }
}
#[tokio::test]
async fn accept_contracts() {
    let interface = SpaceTradersHandler::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        interface.accept_contract(SYSTEM).await;
    }
}
#[tokio::test]
async fn deliver_contract() {
    let interface = SpaceTradersHandler::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        interface.deliver_contract(SYSTEM).await;
    }
}
#[tokio::test]
async fn fulfill_contract() {
    let interface = SpaceTradersHandler::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        interface.fulfill_contract(SYSTEM).await;
    }
}

#[tokio::test]
async fn list_factions() {
    let interface = SpaceTradersHandler::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        interface.list_factions().await;
    }
}
#[tokio::test]
async fn get_faction() {
    let interface = SpaceTradersHandler::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        interface.get_faction(SYSTEM).await;
    }
}

#[tokio::test]
async fn list_ships() {
    let interface = SpaceTradersHandler::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        interface.list_ships().await;
    }
}
#[tokio::test]
async fn purchase_ship() {
    let interface = SpaceTradersHandler::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        interface
            .purchase_ship(BuyShip {
                waypoint_symbol: SYSTEM.to_string(),
                ship_type: ShipType::ShipMiningDrone,
            })
            .await;
    }
}
#[tokio::test]
async fn get_ship() {
    let interface = SpaceTradersHandler::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        interface.get_ship(SYSTEM).await;
    }
}
#[tokio::test]
async fn get_ship_cargo() {
    let interface = SpaceTradersHandler::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        interface.get_ship_cargo(SYSTEM).await;
    }
}
#[tokio::test]
async fn orbit_ship() {
    let interface = SpaceTradersHandler::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        interface.orbit_ship(SYSTEM).await;
    }
}
#[tokio::test]
async fn ship_refine() {
    let interface = SpaceTradersHandler::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        interface
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
async fn create_chart() {
    let interface = SpaceTradersHandler::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        interface.create_chart(SYSTEM).await;
    }
}
#[tokio::test]
async fn get_ship_cooldown() {
    let interface = SpaceTradersHandler::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        interface.get_ship_cooldown(SYSTEM).await;
    }
}
#[tokio::test]
async fn dock_ship() {
    let interface = SpaceTradersHandler::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        interface.dock_ship(SYSTEM).await;
    }
}
#[tokio::test]
async fn create_survey() {
    let interface = SpaceTradersHandler::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        interface.create_survey(SYSTEM).await;
    }
}
#[tokio::test]
async fn extract_resources() {
    let interface = SpaceTradersHandler::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        interface.extract_resources(SYSTEM).await;
    }
}
#[tokio::test]
async fn jettison_cargo() {
    let interface = SpaceTradersHandler::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        interface.jettison_cargo(SYSTEM).await;
    }
}
#[tokio::test]
async fn jump_ship() {
    let interface = SpaceTradersHandler::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        interface
            .jump_ship(
                SYSTEM,
                JumpShip {
                    ship_symbol: ShipType::ShipProbe,
                },
            )
            .await;
    }
}
#[tokio::test]
async fn navigate_ship() {
    let interface = SpaceTradersHandler::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        interface
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
async fn patch_ship_nav() {
    let interface = SpaceTradersHandler::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        interface
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
async fn get_ship_nav() {
    let interface = SpaceTradersHandler::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        interface.get_ship_nav(SYSTEM).await;
    }
}
#[tokio::test]
async fn warp_ship() {
    let interface = SpaceTradersHandler::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        interface.warp_ship(SYSTEM).await;
    }
}
#[tokio::test]
async fn sell_cargo() {
    let interface = SpaceTradersHandler::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        interface
            .sell_cargo(
                SYSTEM,
                WarpShip {
                    ship_symbol: SYSTEM.to_string(),
                },
            )
            .await;
    }
}
#[tokio::test]
async fn scan_systems() {
    let interface = SpaceTradersHandler::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        interface.scan_systems(SYSTEM).await;
    }
}
#[tokio::test]
async fn scan_waypoints() {
    let interface = SpaceTradersHandler::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        interface.scan_waypoints(SYSTEM).await;
    }
}
#[tokio::test]
async fn scan_ships() {
    let interface = SpaceTradersHandler::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        interface.scan_ships(SYSTEM).await;
    }
}
#[tokio::test]
async fn purchase_cargo() {
    let interface = SpaceTradersHandler::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        interface.purchase_cargo(SYSTEM).await;
    }
}
#[tokio::test]
async fn transfer_cargo() {
    let interface = SpaceTradersHandler::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        interface.transfer_cargo(SYSTEM).await;
    }
}
#[tokio::test]
async fn negotiate_contract() {
    let interface = SpaceTradersHandler::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        interface.negotiate_contract(SYSTEM).await;
    }
}
#[tokio::test]
async fn get_mounts() {
    let interface = SpaceTradersHandler::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        interface.get_mounts(SYSTEM).await;
    }
}
#[tokio::test]
async fn install_mount() {
    let interface = SpaceTradersHandler::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        interface
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
async fn remove_mount() {
    let interface = SpaceTradersHandler::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        interface
            .remove_mount(
                SYSTEM,
                RemoveMount {
                    symbol: SYSTEM.to_string(),
                },
            )
            .await;
    }
}
