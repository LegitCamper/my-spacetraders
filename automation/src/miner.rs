use spacetraders::{
    //contracts
    // SpaceTraders,
    enums::{self, WaypointType},
    requests,
    responses::schemas,
    Waypoint,
};

use super::func::ShipDataAbstractor;

use log::{info, trace, warn};

pub async fn mine_astroid(ship_id: &str, ship_data: ShipDataAbstractor) {
    trace!("Mining Astroid");

    let ship = ship_data.clone_ship(ship_id).await.unwrap();
    let waypoints = ship_data.get_waypoints(ship.nav.system_symbol).await;
    let ship_waypoint = ship_data.get_waypoint(ship.nav.waypoint_symbol).await;

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

    // sort the distances
    {
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
    }

    for (waypoint, _distance) in mine_distances.iter() {
        let ship = ship_data
            .travel_waypoint(ship_id, waypoint.symbol.waypoint.as_str())
            .await
            .unwrap();

        if ship.nav.status == enums::ShipNavStatus::Docked {
            ship_data.orbit_ship(ship_id).await;
        } else if ship.nav.status == enums::ShipNavStatus::InTransit {
            ship_data.wait_duration(ship_id).await;
            ship_data.orbit_ship(ship_id).await;
        }

        info!("Starting mining astroid");

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
            let temp_ship_data = ship_data
                .0
                .lock()
                .await
                .spacetraders
                .extract_resources(ship_id, None)
                .await
                .data;

            ship_data
                .0
                .lock()
                .await
                .ships
                .get_mut(ship_id)
                .unwrap()
                .cargo = temp_ship_data.cargo.clone();
            let (_cooldown, _extraction) = (temp_ship_data.cooldown, temp_ship_data.extraction);

            if &temp_ship_data.cargo.capacity - &temp_ship_data.cargo.units > 1 {
                continue;
            } else {
                break;
            }
        }

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

        // sort the distances
        {
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
        }

        for (waypoint, _distance) in mine_distances.iter() {
            let ship = ship_data
                .travel_waypoint(ship_id, waypoint.symbol.waypoint.as_str())
                .await
                .unwrap();

            if ship.nav.status == enums::ShipNavStatus::InOrbit {
                ship_data.dock_ship(ship_id).await;
            } else if ship.nav.status == enums::ShipNavStatus::InTransit {
                ship_data.wait_duration(ship_id).await;
                ship_data.dock_ship(ship_id).await;
            }

            // TODO: make sure not to sell goods used for contracts
            // TODO: also make sure I can sell that good here
            for item in ship.cargo.inventory.clone().iter() {
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
                    .data;

                unlocked.ships.get_mut(ship_id).unwrap().cargo = temp_ship_data.cargo;
                let (_agent, transaction) = (temp_ship_data.agent, temp_ship_data.transaction);

                ship_data.add_credits(transaction.units).await;
            }
            return;
        }
        //TODO: else maybe fly to the closest system with a shipyard - TODO: Pathfinding
    }
    warn!("Failed to find asteroid");
}
