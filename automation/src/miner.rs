use spacetraders::{
    //contracts
    // SpaceTraders,
    enums,
    requests,
    responses::schemas,
};

use super::func::ShipAutomation;

use log::{error, info, trace, warn};
use tokio::time::{sleep, Duration};

// enum MinerTask {
//     GasMiner,
//     AstroidMiner,
//     Contractor,
// }

// pub async fn mine(ship_data: ShipWrapper, contractor: bool) {
//     trace!("{} Mine", ship_data.ship_id);

//     let mut miner_task: MinerTask;

//     if contractor {
//         miner_task = MinerTask::Contractor;
//     } else {
//         for mount in ship_data.clone_ship().await.unwrap().mounts.into_iter() {
//             if mount.symbol == enums::ShipMount::MountGasSiphonI
//                 || mount.symbol == enums::ShipMount::MountGasSiphonIi
//                 || mount.symbol == enums::ShipMount::MountGasSiphonIii
//             {
//                 miner_task = MinerTask::GasMiner
//             }
//             if mount.symbol == enums::ShipMount::MountMiningLaserI
//                 || mount.symbol == enums::ShipMount::MountMiningLaserIi
//                 || mount.symbol == enums::ShipMount::MountMiningLaserIii
//             {
//                 miner_task = MinerTask::AstroidMiner
//             }
//         }
//     }
// }

pub async fn mine_astroid(ship_data: &ShipAutomation) {
    trace!("{} Mining Astroid", ship_data.ship_id);

    let ship = ship_data.clone_ship().await.unwrap();
    let waypoints = ship_data.get_waypoints(&ship.nav.system_symbol).await;
    let ship_waypoint = ship_data.get_waypoint(&ship.nav.waypoint_symbol).await;

    let mut mine_distances: Vec<(&schemas::Waypoint, u64)> = Vec::new();
    for waypoint in waypoints.iter() {
        if waypoint.r#type == enums::WaypointType::AsteroidField
            || waypoint.r#type == enums::WaypointType::DebrisField
        // TODO: definatly look into other minable location
        {
            let distance = ship_data.euclidean_distance(
                waypoint.x,
                waypoint.y,
                ship_waypoint.x,
                ship_waypoint.y,
            );
            mine_distances.push((waypoint, distance));
        }
    }
    if mine_distances.is_empty() {
        warn!("{} Failed to find mineable location", ship_data.ship_id);
        return;
    }
    mine_distances = sort_distance(mine_distances);

    for (waypoint, _distance) in mine_distances.iter() {
        let ship = ship_data
            .travel_waypoint(waypoint.symbol.waypoint.as_str())
            .await
            .unwrap();

        if ship.nav.status == enums::ShipNavStatus::Docked {
            ship_data.orbit_ship().await;
        } else if ship.nav.status == enums::ShipNavStatus::InTransit {
            ship_data.wait_flight_duration().await;
            ship_data.orbit_ship().await;
        }

        info!("{} Starting mining astroid", ship_data.ship_id);

        loop {
            if let Some((cargo, cooldown, _extraction)) = ship_data.extract_resources().await {
                if cargo.capacity - cargo.units > 1 {
                    info!(
                        "{} is on cooldown from mining for {} seconds",
                        ship_data.ship_id, cooldown.remaining_seconds
                    );
                    sleep(Duration::from_secs(cooldown.remaining_seconds.into())).await;
                    continue;
                } else {
                    break;
                }
            } else {
                error!("{} Extracting failed", ship_data.ship_id);
                break;
            }
        }
    }
}

pub fn sort_distance(
    mut mine_distances: Vec<(&schemas::Waypoint, u64)>,
) -> Vec<(&schemas::Waypoint, u64)> {
    let mut swapped = true;
    while swapped {
        // No swap means array is sorted.
        swapped = false;
        for i in 1..mine_distances.len() {
            if mine_distances[i - 1].1 > mine_distances[i].1 {
                mine_distances.swap(i - 1, i);
                swapped = true
            }
        }
    }
    mine_distances
}

pub async fn sell_mining_cargo(ship_data: &ShipAutomation) {
    trace!("Sell Mining Cargo");

    let ship = ship_data.clone_ship().await.unwrap();
    let waypoints = ship_data.get_waypoints(&ship.nav.system_symbol).await;
    let ship_waypoint = ship_data.get_waypoint(&ship.nav.waypoint_symbol).await;

    let mut mine_distances: Vec<(&schemas::Waypoint, u64)> = Vec::new();
    for waypoint in waypoints.iter() {
        for r#trait in waypoint.traits.iter() {
            if r#trait.symbol == enums::WaypointTrait::Marketplace {
                {
                    let distance = ship_data.euclidean_distance(
                        waypoint.x,
                        waypoint.y,
                        ship_waypoint.x,
                        ship_waypoint.y,
                    );
                    mine_distances.push((waypoint, distance));
                }
            }
        }
    }
    if mine_distances.is_empty() {
        warn!("{} Failed to find a market location", ship_data.ship_id);
        return;
    }
    let mine_distances = sort_distance(mine_distances);

    // TODO: make sure not to sell goods used for contracts
    // TODO: consider demand and fuel to get to where prices are better
    for item in ship.cargo.inventory.clone().iter() {
        'inner: for (waypoint, _distance) in mine_distances.iter() {
            let market = ship_data
                .ship_automation
                .read()
                .await
                .spacetraders
                .get_market(&waypoint.system_symbol, &waypoint.symbol)
                .await
                .unwrap()
                .data;
            for tradegood in market.imports.iter() {
                if tradegood.symbol == item.symbol {
                    sell_mining_item(&ship_data.ship_id, ship_data, item, waypoint).await;
                    continue 'inner;
                }
            }
            for tradegood in market.exchange.iter() {
                if tradegood.symbol == item.symbol {
                    sell_mining_item(&ship_data.ship_id, ship_data, item, waypoint).await;
                    break 'inner;
                }
            }
            for tradegood in market.trade_goods.iter() {
                if tradegood.symbol == item.symbol {
                    sell_mining_item(&ship_data.ship_id, ship_data, item, waypoint).await;
                    break 'inner;
                }
            }
        }
        warn!(
            "{} Unable to find location to sell {:?}",
            ship_data.ship_id, item.symbol
        );
        // Assuming this is correct unless I travel elsewhere I should just jettison
        info!(
            "{} Jettison {} {:?}",
            ship_data.ship_id, item.units, item.symbol
        );
        let _ = ship_data
            .ship_automation
            .read()
            .await
            .spacetraders
            .jettison_cargo(
                &ship_data.ship_id,
                requests::JettisonCargo {
                    symbol: item.symbol.clone(),
                    units: item.units,
                },
            )
            .await;
    }
}

pub async fn sell_mining_item(
    ship_id: &str,
    ship_data: &ShipAutomation,
    item: &schemas::ShipCargoItem,
    waypoint: &schemas::Waypoint,
) {
    trace!("Sell Mining Item");

    let ship = ship_data
        .travel_waypoint(waypoint.symbol.waypoint.as_str())
        .await
        .unwrap();

    if ship.nav.status == enums::ShipNavStatus::InOrbit {
        ship_data.dock_ship().await;
    } else if ship.nav.status == enums::ShipNavStatus::InTransit {
        ship_data.wait_flight_duration().await;
        ship_data.dock_ship().await;
    }

    info!("{} is selling {} {:?}", ship_id, item.units, item.symbol);

    {
        let mut unlocked = ship_data.ship_automation.write().await;

        let temp_ship_data = unlocked
            .spacetraders
            .sell_cargo(
                ship_id,
                requests::SellCargo {
                    symbol: item.symbol.clone(),
                    units: item.units,
                },
            )
            .await;
        if temp_ship_data.is_ok() {
            let temp_ship_data = temp_ship_data.unwrap().data;

            unlocked.ships.get_mut(ship_id).unwrap().cargo = temp_ship_data.cargo;
            let (_agent, transaction) = (temp_ship_data.agent, temp_ship_data.transaction);

            unlocked.credits += transaction.units;
            ship_data.credits_generated += transaction.units
        }
    }
}
