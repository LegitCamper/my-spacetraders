use spacetraders::{
    //contracts
    // SpaceTraders,
    enums,
    requests,
    responses::schemas,
};

use super::func::ShipDataAbstractor;

use log::{error, info, trace, warn};
use tokio::time::{sleep, Duration};

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

pub async fn mine_astroid(ship_id: &str, ship_data: ShipDataAbstractor) {
    trace!("{} Mining Astroid", ship_id);

    let ship = ship_data.clone_ship(ship_id).await.unwrap();
    let waypoints = ship_data.get_waypoints(&ship.nav.system_symbol).await;
    let ship_waypoint = ship_data.get_waypoint(&ship.nav.waypoint_symbol).await;

    let mut mine_distances: Vec<(&schemas::Waypoint, u64)> = Vec::new();
    for waypoint in waypoints.iter() {
        if waypoint.r#type == enums::WaypointType::AsteroidField
            || waypoint.r#type == enums::WaypointType::DebrisField
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
        warn!("{} Failed to find mineable location", ship_id);
        return;
    }
    mine_distances = sort_distance(mine_distances);

    for (waypoint, _distance) in mine_distances.iter() {
        let ship = ship_data
            .travel_waypoint(ship_id, waypoint.symbol.waypoint.as_str())
            .await
            .unwrap();

        if ship.nav.status == enums::ShipNavStatus::Docked {
            ship_data.orbit_ship(ship_id).await;
        } else if ship.nav.status == enums::ShipNavStatus::InTransit {
            ship_data.wait_flight_duration(ship_id).await;
            ship_data.orbit_ship(ship_id).await;
        }

        info!("{} Starting mining astroid", ship_id);

        'inner: for mount in ship.mounts.iter() {
            if mount.symbol == enums::ShipMount::MountSurveyorI
                || mount.symbol == enums::ShipMount::MountSurveyorIi
                || mount.symbol == enums::ShipMount::MountSurveyorIii
            {
                let _surveys = ship_data.create_survey(ship_id).await;
                break 'inner;
            }
        }
        loop {
            let temp_ship_data = match ship_data
                .0
                .lock()
                .await
                .spacetraders
                .extract_resources(ship_id, None)
                .await
            {
                Some(data) => data.data,
                None => {
                    error!("{} Failed to extract resources", ship_id);
                    break;
                }
            };

            ship_data
                .0
                .lock()
                .await
                .ships
                .get_mut(ship_id)
                .unwrap()
                .cargo = temp_ship_data.cargo.clone();
            let (cooldown, _extraction) = (temp_ship_data.cooldown, temp_ship_data.extraction);

            if &temp_ship_data.cargo.capacity - &temp_ship_data.cargo.units > 1 {
                info!(
                    "{} is on cooldown from mining for {} seconds",
                    ship_id, cooldown.remaining_seconds
                );
                sleep(Duration::from_secs(cooldown.remaining_seconds.into())).await;
                continue;
            } else {
                break;
            }
        }
    }
}

pub async fn sell_mining_cargo(ship_id: &str, ship_data: ShipDataAbstractor) {
    trace!("Sell Mining Cargo");

    let ship = ship_data.clone_ship(ship_id).await.unwrap();
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
        warn!("{} Failed to find a market location", ship_id);
        return;
    }
    let mine_distances = sort_distance(mine_distances);

    // TODO: make sure not to sell goods used for contracts
    // TODO: consider demand and gas to get to where prices are better
    for item in ship.cargo.inventory.clone().iter() {
        'inner: for (waypoint, _distance) in mine_distances.iter() {
            let market = ship_data
                .0
                .lock()
                .await
                .spacetraders
                .get_market(&waypoint.system_symbol, &waypoint.symbol)
                .await
                .unwrap()
                .data;
            for tradegood in market.imports.iter() {
                if tradegood.symbol == item.symbol {
                    sell_mining_item(ship_id, ship_data.clone(), &item, waypoint).await;
                    continue 'inner;
                }
            }
            for tradegood in market.exchange.iter() {
                if tradegood.symbol == item.symbol {
                    sell_mining_item(ship_id, ship_data.clone(), &item, waypoint).await;
                    break 'inner;
                }
            }
            for tradegood in market.trade_goods.iter() {
                if tradegood.symbol == item.symbol {
                    sell_mining_item(ship_id, ship_data.clone(), &item, waypoint).await;
                    break 'inner;
                }
            }
        }
        warn!(
            "{} Unable to find location to sell {:?}",
            ship_id, item.symbol
        )
    }
}

pub async fn sell_mining_item(
    ship_id: &str,
    ship_data: ShipDataAbstractor,
    item: &schemas::ShipCargoItem,
    waypoint: &schemas::Waypoint,
) {
    trace!("Sell Mining Item");

    let ship = ship_data
        .travel_waypoint(ship_id, waypoint.symbol.waypoint.as_str())
        .await
        .unwrap();

    if ship.nav.status == enums::ShipNavStatus::InOrbit {
        ship_data.dock_ship(ship_id).await;
    } else if ship.nav.status == enums::ShipNavStatus::InTransit {
        ship_data.wait_flight_duration(ship_id).await;
        ship_data.dock_ship(ship_id).await;
    }

    info!("{} is selling {} {:?}", ship_id, item.units, item.symbol);

    let mut unlocked = ship_data.0.lock().await;

    let temp_ship_data = unlocked
        .spacetraders
        .sell_cargo(
            ship_id,
            requests::SellCargo {
                symbol: item.symbol.clone(),
                units: item.units,
            },
        )
        .await
        .unwrap()
        .data;

    unlocked.ships.get_mut(ship_id).unwrap().cargo = temp_ship_data.cargo;
    let (_agent, transaction) = (temp_ship_data.agent, temp_ship_data.transaction);

    ship_data.add_credits(transaction.units).await;
}
