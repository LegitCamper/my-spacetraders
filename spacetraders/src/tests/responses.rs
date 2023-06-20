use crate::{
    enums::{FlightMode, ShipType, TradeSymbol},
    requests::{
        BuyShip, InstallMount, JumpShip, NavigateShip, PatchShipNav, RemoveMount, ShipRefine,
        WarpShip,
    },
    Method, SpaceTraders,
};

use async_once::AsyncOnce;
use async_once_cell::OnceCell;
use lazy_static::lazy_static;
// use once_cell::sync::OnceCell;

const TIMES_TO_RUN: i32 = 10;
const SYSTEM: &str = "X1-OE";

lazy_static! {
    pub static ref INTERFACE: AsyncOnce<&SpaceTraders> = AsyncOnce::new(async {
        OnceCell::new()
            .get_or_init(async { SpaceTraders::new_testing().await })
            .await
    });
}

#[tokio::test]
async fn recieve_error() {
    for _ in 0..TIMES_TO_RUN {
        INTERFACE
            .get
            .interface
            .custom_endpoint("/doesnotexist", Method::Post)
            .await;
    }
}

#[tokio::test]
async fn get_new_registration() {
    let space_traders = SpaceTraders::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        space_traders
            .get()
            .unwrap()
            .interface
            .custom_endpoint("/register", Method::Post)
            .await;
    }
}

#[tokio::test]
async fn agent() {
    let space_traders = SpaceTraders::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        space_traders.get().unwrap().agent().await;
    }
}

#[tokio::test]
async fn list_systems() {
    let space_traders = SpaceTraders::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        space_traders.get().unwrap().list_systems().await;
    }
}
#[tokio::test]
async fn get_systems() {
    let space_traders = SpaceTraders::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        space_traders.get().unwrap().get_system(SYSTEM).await;
    }
}
#[tokio::test]
async fn list_waypoints() {
    let space_traders = SpaceTraders::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        space_traders.get().unwrap().list_waypoints(SYSTEM).await;
    }
}
#[tokio::test]
async fn get_waypoint() {
    let space_traders = SpaceTraders::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        space_traders
            .get()
            .unwrap()
            .get_waypoint(SYSTEM, SYSTEM)
            .await;
    }
}
#[tokio::test]
async fn get_market() {
    let space_traders = SpaceTraders::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        space_traders
            .get()
            .unwrap()
            .get_market(SYSTEM, SYSTEM)
            .await;
    }
}
#[tokio::test]
async fn get_shipyard() {
    let space_traders = SpaceTraders::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        space_traders
            .get()
            .unwrap()
            .get_shipyard(SYSTEM, SYSTEM)
            .await;
    }
}
#[tokio::test]
async fn get_jump_gate() {
    let space_traders = SpaceTraders::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        space_traders.get().unwrap().jump_gate(SYSTEM, SYSTEM).await;
    }
}

#[tokio::test]
async fn list_contracts() {
    let space_traders = SpaceTraders::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        space_traders.get().unwrap().list_contracts().await;
    }
}
#[tokio::test]
async fn get_contract() {
    let space_traders = SpaceTraders::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        space_traders.get().unwrap().get_contract(SYSTEM).await;
    }
}
#[tokio::test]
async fn accept_contracts() {
    let space_traders = SpaceTraders::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        space_traders.get().unwrap().accept_contract(SYSTEM).await;
    }
}
#[tokio::test]
async fn deliver_contract() {
    let space_traders = SpaceTraders::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        space_traders.get().unwrap().deliver_contract(SYSTEM).await;
    }
}
#[tokio::test]
async fn fulfill_contract() {
    let space_traders = SpaceTraders::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        space_traders.get().unwrap().fulfill_contract(SYSTEM).await;
    }
}

#[tokio::test]
async fn list_factions() {
    let space_traders = SpaceTraders::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        space_traders.get().unwrap().list_factions().await;
    }
}
#[tokio::test]
async fn get_faction() {
    let space_traders = SpaceTraders::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        space_traders.get().unwrap().get_faction(SYSTEM).await;
    }
}

#[tokio::test]
async fn list_ships() {
    let space_traders = SpaceTraders::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        space_traders.get().unwrap().list_ships().await;
    }
}
#[tokio::test]
async fn purchase_ship() {
    let space_traders = SpaceTraders::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        space_traders
            .get()
            .unwrap()
            .purchase_ship(BuyShip {
                waypoint_symbol: SYSTEM.to_string(),
                ship_type: ShipType::ShipMiningDrone,
            })
            .await;
    }
}
#[tokio::test]
async fn get_ship() {
    let space_traders = SpaceTraders::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        space_traders.get().unwrap().get_ship(SYSTEM).await;
    }
}
#[tokio::test]
async fn get_ship_cargo() {
    let space_traders = SpaceTraders::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        space_traders.get().unwrap().get_ship_cargo(SYSTEM).await;
    }
}
#[tokio::test]
async fn orbit_ship() {
    let space_traders = SpaceTraders::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        space_traders.get().unwrap().orbit_ship(SYSTEM).await;
    }
}
#[tokio::test]
async fn ship_refine() {
    let space_traders = SpaceTraders::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        space_traders
            .get()
            .unwrap()
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
    let space_traders = SpaceTraders::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        space_traders.get().unwrap().create_chart(SYSTEM).await;
    }
}
#[tokio::test]
async fn get_ship_cooldown() {
    let space_traders = SpaceTraders::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        space_traders.get().unwrap().get_ship_cooldown(SYSTEM).await;
    }
}
#[tokio::test]
async fn dock_ship() {
    let space_traders = SpaceTraders::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        space_traders.get().unwrap().dock_ship(SYSTEM).await;
    }
}
#[tokio::test]
async fn create_survey() {
    let space_traders = SpaceTraders::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        space_traders.get().unwrap().create_survey(SYSTEM).await;
    }
}
#[tokio::test]
async fn extract_resources() {
    let space_traders = SpaceTraders::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        space_traders.get().unwrap().extract_resources(SYSTEM).await;
    }
}
#[tokio::test]
async fn jettison_cargo() {
    let space_traders = SpaceTraders::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        space_traders.get().unwrap().jettison_cargo(SYSTEM).await;
    }
}
#[tokio::test]
async fn jump_ship() {
    let space_traders = SpaceTraders::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        space_traders
            .get()
            .unwrap()
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
    let space_traders = SpaceTraders::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        space_traders
            .get()
            .unwrap()
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
    let space_traders = SpaceTraders::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        space_traders
            .get()
            .unwrap()
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
    let space_traders = SpaceTraders::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        space_traders.get().unwrap().get_ship_nav(SYSTEM).await;
    }
}
#[tokio::test]
async fn warp_ship() {
    let space_traders = SpaceTraders::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        space_traders.get().unwrap().warp_ship(SYSTEM).await;
    }
}
#[tokio::test]
async fn sell_cargo() {
    let space_traders = SpaceTraders::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        space_traders
            .get()
            .unwrap()
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
    let space_traders = SpaceTraders::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        space_traders.get().unwrap().scan_systems(SYSTEM).await;
    }
}
#[tokio::test]
async fn scan_waypoints() {
    let space_traders = SpaceTraders::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        space_traders.get().unwrap().scan_waypoints(SYSTEM).await;
    }
}
#[tokio::test]
async fn scan_ships() {
    let space_traders = SpaceTraders::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        space_traders.get().unwrap().scan_ships(SYSTEM).await;
    }
}
#[tokio::test]
async fn purchase_cargo() {
    let space_traders = SpaceTraders::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        space_traders.get().unwrap().purchase_cargo(SYSTEM).await;
    }
}
#[tokio::test]
async fn transfer_cargo() {
    let space_traders = SpaceTraders::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        space_traders.get().unwrap().transfer_cargo(SYSTEM).await;
    }
}
#[tokio::test]
async fn negotiate_contract() {
    let space_traders = SpaceTraders::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        space_traders
            .get()
            .unwrap()
            .negotiate_contract(SYSTEM)
            .await;
    }
}
#[tokio::test]
async fn get_mounts() {
    let space_traders = SpaceTraders::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        space_traders.get().unwrap().get_mounts(SYSTEM).await;
    }
}
#[tokio::test]
async fn install_mount() {
    let space_traders = SpaceTraders::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        space_traders
            .get()
            .unwrap()
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
    let space_traders = SpaceTraders::new_testing().await;
    for _ in 0..TIMES_TO_RUN {
        space_traders
            .get()
            .unwrap()
            .remove_mount(
                SYSTEM,
                RemoveMount {
                    symbol: SYSTEM.to_string(),
                },
            )
            .await;
    }
}
