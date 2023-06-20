use crate::{
    enums::{FlightMode, ShipType, TradeSymbol},
    requests::{
        BuyShip, InstallMount, JumpShip, NavigateShip, PatchShipNav, RemoveMount, ShipRefine,
        WarpShip,
    },
    Method, SpaceTraders,
};

use async_once::AsyncOnce;
use lazy_static::lazy_static;

const TIMES_TO_RUN: i32 = 10;
const SYSTEM: &str = "X1-OE";

lazy_static! {
    static ref INTERFACE: AsyncOnce<SpaceTraders> =
        AsyncOnce::new(async { SpaceTraders::new_testing().await });
}

#[tokio::test]
async fn recieve_error() {
    for _ in 0..TIMES_TO_RUN {
        INTERFACE
            .get()
            .await
            .interface
            .custom_endpoint("/doesnotexist", Method::Post)
            .await;
    }
}

#[tokio::test]
async fn get_new_registration() {
    for _ in 0..TIMES_TO_RUN {
        INTERFACE
            .get()
            .await
            .interface
            .custom_endpoint("/register", Method::Post)
            .await;
    }
}

#[tokio::test]
async fn agent() {
    for _ in 0..TIMES_TO_RUN {
        INTERFACE.get().await.agent().await;
    }
}

#[tokio::test]
async fn list_systems() {
    for _ in 0..TIMES_TO_RUN {
        INTERFACE.get().await.list_systems().await;
    }
}
#[tokio::test]
async fn get_systems() {
    for _ in 0..TIMES_TO_RUN {
        INTERFACE.get().await.get_system(SYSTEM).await;
    }
}
#[tokio::test]
async fn list_waypoints() {
    for _ in 0..TIMES_TO_RUN {
        INTERFACE.get().await.list_waypoints(SYSTEM).await;
    }
}
#[tokio::test]
async fn get_waypoint() {
    for _ in 0..TIMES_TO_RUN {
        INTERFACE.get().await.get_waypoint(SYSTEM, SYSTEM).await;
    }
}
#[tokio::test]
async fn get_market() {
    for _ in 0..TIMES_TO_RUN {
        INTERFACE.get().await.get_market(SYSTEM, SYSTEM).await;
    }
}
#[tokio::test]
async fn get_shipyard() {
    for _ in 0..TIMES_TO_RUN {
        INTERFACE.get().await.get_shipyard(SYSTEM, SYSTEM).await;
    }
}
#[tokio::test]
async fn get_jump_gate() {
    for _ in 0..TIMES_TO_RUN {
        INTERFACE.get().await.jump_gate(SYSTEM, SYSTEM).await;
    }
}

#[tokio::test]
async fn list_contracts() {
    for _ in 0..TIMES_TO_RUN {
        INTERFACE.get().await.list_contracts().await;
    }
}
#[tokio::test]
async fn get_contract() {
    for _ in 0..TIMES_TO_RUN {
        INTERFACE.get().await.get_contract(SYSTEM).await;
    }
}
#[tokio::test]
async fn accept_contracts() {
    for _ in 0..TIMES_TO_RUN {
        INTERFACE.get().await.accept_contract(SYSTEM).await;
    }
}
#[tokio::test]
async fn deliver_contract() {
    for _ in 0..TIMES_TO_RUN {
        INTERFACE.get().await.deliver_contract(SYSTEM).await;
    }
}
#[tokio::test]
async fn fulfill_contract() {
    for _ in 0..TIMES_TO_RUN {
        INTERFACE.get().await.fulfill_contract(SYSTEM).await;
    }
}

#[tokio::test]
async fn list_factions() {
    for _ in 0..TIMES_TO_RUN {
        INTERFACE.get().await.list_factions().await;
    }
}
#[tokio::test]
async fn get_faction() {
    for _ in 0..TIMES_TO_RUN {
        INTERFACE.get().await.get_faction(SYSTEM).await;
    }
}

#[tokio::test]
async fn list_ships() {
    for _ in 0..TIMES_TO_RUN {
        INTERFACE.get().await.list_ships().await;
    }
}
#[tokio::test]
async fn purchase_ship() {
    for _ in 0..TIMES_TO_RUN {
        INTERFACE
            .get()
            .await
            .purchase_ship(BuyShip {
                waypoint_symbol: SYSTEM.to_string(),
                ship_type: ShipType::ShipMiningDrone,
            })
            .await;
    }
}
#[tokio::test]
async fn get_ship() {
    for _ in 0..TIMES_TO_RUN {
        INTERFACE.get().await.get_ship(SYSTEM).await;
    }
}
#[tokio::test]
async fn get_ship_cargo() {
    for _ in 0..TIMES_TO_RUN {
        INTERFACE.get().await.get_ship_cargo(SYSTEM).await;
    }
}
#[tokio::test]
async fn orbit_ship() {
    for _ in 0..TIMES_TO_RUN {
        INTERFACE.get().await.orbit_ship(SYSTEM).await;
    }
}
#[tokio::test]
async fn ship_refine() {
    for _ in 0..TIMES_TO_RUN {
        INTERFACE
            .get()
            .await
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
    for _ in 0..TIMES_TO_RUN {
        INTERFACE.get().await.create_chart(SYSTEM).await;
    }
}
#[tokio::test]
async fn get_ship_cooldown() {
    for _ in 0..TIMES_TO_RUN {
        INTERFACE.get().await.get_ship_cooldown(SYSTEM).await;
    }
}
#[tokio::test]
async fn dock_ship() {
    for _ in 0..TIMES_TO_RUN {
        INTERFACE.get().await.dock_ship(SYSTEM).await;
    }
}
#[tokio::test]
async fn create_survey() {
    for _ in 0..TIMES_TO_RUN {
        INTERFACE.get().await.create_survey(SYSTEM).await;
    }
}
#[tokio::test]
async fn extract_resources() {
    for _ in 0..TIMES_TO_RUN {
        INTERFACE.get().await.extract_resources(SYSTEM).await;
    }
}
#[tokio::test]
async fn jettison_cargo() {
    for _ in 0..TIMES_TO_RUN {
        INTERFACE.get().await.jettison_cargo(SYSTEM).await;
    }
}
#[tokio::test]
async fn jump_ship() {
    for _ in 0..TIMES_TO_RUN {
        INTERFACE
            .get()
            .await
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
    for _ in 0..TIMES_TO_RUN {
        INTERFACE
            .get()
            .await
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
    for _ in 0..TIMES_TO_RUN {
        INTERFACE
            .get()
            .await
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
    for _ in 0..TIMES_TO_RUN {
        INTERFACE.get().await.get_ship_nav(SYSTEM).await;
    }
}
#[tokio::test]
async fn warp_ship() {
    for _ in 0..TIMES_TO_RUN {
        INTERFACE.get().await.warp_ship(SYSTEM).await;
    }
}
#[tokio::test]
async fn sell_cargo() {
    for _ in 0..TIMES_TO_RUN {
        INTERFACE
            .get()
            .await
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
    for _ in 0..TIMES_TO_RUN {
        INTERFACE.get().await.scan_systems(SYSTEM).await;
    }
}
#[tokio::test]
async fn scan_waypoints() {
    for _ in 0..TIMES_TO_RUN {
        INTERFACE.get().await.scan_waypoints(SYSTEM).await;
    }
}
#[tokio::test]
async fn scan_ships() {
    for _ in 0..TIMES_TO_RUN {
        INTERFACE.get().await.scan_ships(SYSTEM).await;
    }
}
#[tokio::test]
async fn purchase_cargo() {
    for _ in 0..TIMES_TO_RUN {
        INTERFACE.get().await.purchase_cargo(SYSTEM).await;
    }
}
#[tokio::test]
async fn transfer_cargo() {
    for _ in 0..TIMES_TO_RUN {
        INTERFACE.get().await.transfer_cargo(SYSTEM).await;
    }
}
#[tokio::test]
async fn negotiate_contract() {
    for _ in 0..TIMES_TO_RUN {
        INTERFACE.get().await.negotiate_contract(SYSTEM).await;
    }
}
#[tokio::test]
async fn get_mounts() {
    for _ in 0..TIMES_TO_RUN {
        INTERFACE.get().await.get_mounts(SYSTEM).await;
    }
}
#[tokio::test]
async fn install_mount() {
    for _ in 0..TIMES_TO_RUN {
        INTERFACE
            .get()
            .await
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
    for _ in 0..TIMES_TO_RUN {
        INTERFACE
            .get()
            .await
            .remove_mount(
                SYSTEM,
                RemoveMount {
                    symbol: SYSTEM.to_string(),
                },
            )
            .await;
    }
}
