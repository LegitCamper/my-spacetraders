use spacetraders::{
    //contracts
    // SpaceTraders,
    enums::{self, ShipMount},
    requests,
    responses::schemas,
};

use super::func::{sort_distances, ShipAutomation};

use log::{error, info, trace, warn};
use tokio::time::{sleep, Duration};

#[derive(Debug, PartialEq)]
enum MinerTask {
    GasMiner,
    AstroidMiner,
    Contractor,
}

pub async fn mine(ship_automation: &mut ShipAutomation, contractor: bool) {
    trace!("{} Mine", ship_automation.ship_id);

    let mut miner_task: MinerTask = MinerTask::AstroidMiner;

    if contractor {
        miner_task = MinerTask::Contractor;
    } else {
        for mount in ship_automation
            .clone_ship()
            .await
            .unwrap()
            .mounts
            .into_iter()
        {
            if mount.symbol == ShipMount::MountGasSiphonI
                || mount.symbol == ShipMount::MountGasSiphonIi
                || mount.symbol == ShipMount::MountGasSiphonIii
            {
                miner_task = MinerTask::GasMiner
            }
            if mount.symbol == ShipMount::MountMiningLaserI
                || mount.symbol == ShipMount::MountMiningLaserIi
                || mount.symbol == ShipMount::MountMiningLaserIii
            {
                miner_task = MinerTask::AstroidMiner
            }
        }
    }

    mine_specific(ship_automation, miner_task).await;
    sell_mining_cargo(ship_automation).await;
}

async fn mine_specific(ship_automation: &ShipAutomation, miner_task: MinerTask) {
    trace!("{} Mining Specific", ship_automation.ship_id);

    let ship = ship_automation.clone_ship().await.unwrap();
    let waypoints = ship_automation.get_waypoints(&ship.nav.system_symbol).await;
    let ship_waypoint = ship_automation
        .get_waypoint(&ship.nav.waypoint_symbol)
        .await;

    let mut mine_distances: Vec<(&schemas::Waypoint, u64)> = Vec::new();
    for waypoint in waypoints.iter() {
        if miner_task == MinerTask::AstroidMiner {
            if waypoint.r#type == enums::WaypointType::AsteroidField
                || waypoint.r#type == enums::WaypointType::DebrisField
            {
                let distance = ship_automation.euclidean_distance(
                    waypoint.x,
                    waypoint.y,
                    ship_waypoint.x,
                    ship_waypoint.y,
                );
                mine_distances.push((waypoint, distance));
            }
        } else if miner_task == MinerTask::GasMiner {
            if waypoint.r#type == enums::WaypointType::GasGiant
                || waypoint.r#type == enums::WaypointType::Nebula
            {
                let distance = ship_automation.euclidean_distance(
                    waypoint.x,
                    waypoint.y,
                    ship_waypoint.x,
                    ship_waypoint.y,
                );
                mine_distances.push((waypoint, distance));
            }
        }
    }
    if mine_distances.is_empty() {
        warn!(
            "{} Failed to find mineable location",
            ship_automation.ship_id
        );
        return;
    }
    mine_distances = sort_distances(mine_distances);

    for (waypoint, _distance) in mine_distances.iter() {
        let ship = ship_automation
            .travel_waypoint(waypoint.symbol.waypoint.as_str())
            .await
            .unwrap();

        if ship.nav.status == enums::ShipNavStatus::Docked {
            ship_automation.orbit_ship().await;
        } else if ship.nav.status == enums::ShipNavStatus::InTransit {
            ship_automation.wait_flight_duration().await;
            ship_automation.orbit_ship().await;
        }

        info!("{} Starting mining", ship_automation.ship_id);

        loop {
            if let Some((cargo, cooldown, _extraction)) = ship_automation.extract_resources().await
            {
                if cargo.capacity - cargo.units > 1 {
                    info!(
                        "{} is on cooldown from mining for {} seconds",
                        ship_automation.ship_id, cooldown.remaining_seconds
                    );
                    sleep(Duration::from_secs(cooldown.remaining_seconds.into())).await;
                    continue;
                } else {
                    break;
                }
            } else {
                error!("{} Extracting failed", ship_automation.ship_id);
                break;
            }
        }
    }
}

pub async fn sell_mining_cargo(ship_automation: &mut ShipAutomation) {
    trace!("Sell Mining Cargo");

    let ship = ship_automation.clone_ship().await.unwrap();
    let waypoints = ship_automation.get_waypoints(&ship.nav.system_symbol).await;
    let ship_waypoint = ship_automation
        .get_waypoint(&ship.nav.waypoint_symbol)
        .await;

    let mut mine_distances: Vec<(&schemas::Waypoint, u64)> = Vec::new();
    for waypoint in waypoints.iter() {
        for r#trait in waypoint.traits.iter() {
            if r#trait.symbol == enums::WaypointTrait::Marketplace {
                {
                    let distance = ship_automation.euclidean_distance(
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
        warn!(
            "{} Failed to find a market location",
            ship_automation.ship_id
        );
        return;
    }
    let mine_distances = sort_distances(mine_distances);

    // TODO: make sure not to sell goods used for contracts
    // TODO: consider demand and fuel to get to where prices are better
    for item in ship.cargo.inventory.clone().iter() {
        'inner: for (waypoint, _distance) in mine_distances.iter() {
            let market = ship_automation
                .read()
                .await
                .st_interface
                .get_market(&waypoint.system_symbol, &waypoint.symbol)
                .await
                .unwrap()
                .data;
            for tradegood in market.imports.iter() {
                if tradegood.symbol == item.symbol {
                    sell_mining_item(
                        ship_automation.ship_id.clone().as_str(),
                        ship_automation,
                        item,
                        waypoint,
                    )
                    .await;
                    continue 'inner;
                }
            }
            for tradegood in market.exchange.iter() {
                if tradegood.symbol == item.symbol {
                    sell_mining_item(
                        ship_automation.ship_id.clone().as_str(),
                        ship_automation,
                        item,
                        waypoint,
                    )
                    .await;
                    break 'inner;
                }
            }
            for tradegood in market.trade_goods.iter() {
                if tradegood.symbol == item.symbol {
                    sell_mining_item(
                        ship_automation.ship_id.clone().as_str(),
                        ship_automation,
                        item,
                        waypoint,
                    )
                    .await;
                    break 'inner;
                }
            }
        }
        warn!(
            "{} Unable to find location to sell {:?}",
            ship_automation.ship_id, item.symbol
        );
        // Assuming this is correct unless I travel elsewhere I should just jettison
        info!(
            "{} Jettison {} {:?}",
            ship_automation.ship_id, item.units, item.symbol
        );
        let _ = ship_automation
            .read()
            .await
            .st_interface
            .jettison_cargo(
                &ship_automation.ship_id,
                requests::JettisonCargo {
                    symbol: item.symbol.clone(),
                    units: item.units,
                },
            )
            .await;
    }
}

async fn sell_mining_item(
    ship_id: &str,
    ship_automation: &mut ShipAutomation,
    item: &schemas::ShipCargoItem,
    waypoint: &schemas::Waypoint,
) {
    trace!("Sell Mining Item");

    let ship = ship_automation
        .travel_waypoint(waypoint.symbol.waypoint.as_str())
        .await
        .unwrap();

    if ship.nav.status == enums::ShipNavStatus::InOrbit {
        ship_automation.dock_ship().await;
    } else if ship.nav.status == enums::ShipNavStatus::InTransit {
        ship_automation.wait_flight_duration().await;
        ship_automation.dock_ship().await;
    }

    info!("{} is selling {} {:?}", ship_id, item.units, item.symbol);

    {
        let mut unlocked = ship_automation.shared_data.write().await;

        let temp_ship_automation = unlocked
            .st_interface
            .sell_cargo(
                ship_id,
                requests::SellCargo {
                    symbol: item.symbol.clone(),
                    units: item.units,
                },
            )
            .await;
        if temp_ship_automation.is_ok() {
            let temp_ship_automation = temp_ship_automation.unwrap().data;

            unlocked
                .automation_data
                .ships
                .get_mut(ship_id)
                .unwrap()
                .cargo = temp_ship_automation.cargo;
            let (_agent, transaction) =
                (temp_ship_automation.agent, temp_ship_automation.transaction);

            unlocked.automation_data.credits += transaction.units;
            ship_automation.credits_generated += transaction.units
        }
    }
}
