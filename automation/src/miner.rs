use spacetraders::{
    //contracts
    // SpaceTraders,
    enums,
    requests,
};

use super::func::ShipDataAbstractor;

use log::{info, trace, warn};

pub async fn mine_astroid(ship_id: &str, ship_data: ShipDataAbstractor) {
    trace!("Mining Astroid");

    let ship = ship_data.clone_ship(ship_id).await.unwrap();
    let waypoints = ship_data.get_waypoints(ship.nav.system_symbol).await;

    for waypoint in waypoints.iter() {
        if waypoint.r#type == enums::WaypointType::AsteroidField {
            let ship = ship_data
                .travel_waypoint(ship_id, waypoint.symbol.waypoint.as_str())
                .await
                .unwrap();

            if ship.nav.status == enums::ShipNavStatus::InOrbit {
                ship_data.orbit_ship(ship_id).await;
            }

            info!("Starting mining astroid");

            'inner: for mount in ship.mounts.iter() {
                if mount.symbol == enums::ShipMount::MountSurveyorI
                    || mount.symbol == enums::ShipMount::MountSurveyorIi
                    || mount.symbol == enums::ShipMount::MountSurveyorIii
                {
                    let surveys = ship_data.create_survey(ship_id).await;
                    break 'inner;
                }
            }
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
                .cargo = temp_ship_data.cargo;
            let (_cooldown, _extraction) = (temp_ship_data.cooldown, temp_ship_data.extraction);

            for waypoint in waypoints.iter() {
                for r#trait in waypoint.traits.iter() {
                    if r#trait.symbol == enums::WaypointTrait::Marketplace {
                        let ship = ship_data
                            .travel_waypoint(ship_id, waypoint.symbol.waypoint.as_str())
                            .await
                            .unwrap();

                        if ship.nav.status == enums::ShipNavStatus::InOrbit {
                            ship_data.dock_ship(ship_id).await;
                        }

                        // TODO: make sure not to sell goods used for contracts
                        // TODO: also make sure I can sell that good here
                        for item in ship.cargo.inventory.clone().iter() {
                            info!("{} is selling {} {:?}", ship_id, item.units, item.symbol);

                            let temp_ship_data = ship_data
                                .0
                                .lock()
                                .await
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

                            ship_data
                                .0
                                .lock()
                                .await
                                .ships
                                .get_mut(ship_id)
                                .unwrap()
                                .cargo = temp_ship_data.cargo;
                            let (_agent, transaction) =
                                (temp_ship_data.agent, temp_ship_data.transaction);

                            ship_data.add_credits(transaction.units).await;
                        }
                        return;
                    }
                }
            }
        }
    }

    // else maybe fly to the closest system with a shipyard - TODO: Pathfinding
    warn!("Failed to find asteroid");
}
