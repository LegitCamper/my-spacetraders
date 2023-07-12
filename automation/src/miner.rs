use spacetraders::{
    //contracts
    // SpaceTraders,
    enums,
    requests,
    responses::{self, schemas},
    System,
    Waypoint,
};

use super::func::{get_waypoint, get_waypoints, travel_system, travel_waypoint};
use super::ShipHandlerData;

use log::{info, trace, warn};
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn mine_astroid(ship_id: &str, ship_handler_data: Arc<Mutex<ShipHandlerData>>) {
    trace!("Mining Astroid");

    let system = ship_handler_data
        .lock()
        .await
        .ships
        .get(ship_id)
        .unwrap()
        .nav
        .system_symbol
        .clone();
    let waypoints = get_waypoints(system, ship_handler_data.clone()).await;

    for waypoint in waypoints.iter() {
        if waypoint.r#type == enums::WaypointType::AsteroidField {
            travel_waypoint(
                ship_id,
                ship_handler_data.clone(),
                waypoint.symbol.waypoint.as_str(),
            )
            .await;

            if ship_handler_data
                .lock()
                .await
                .ships
                .get(ship_id)
                .unwrap()
                .nav
                .status
                == enums::ShipNavStatus::InOrbit
            {
                ship_handler_data
                    .lock()
                    .await
                    .ships
                    .get_mut(ship_id)
                    .unwrap()
                    .nav = ship_handler_data
                    .lock()
                    .await
                    .spacetraders
                    .orbit_ship(ship_id)
                    .await
                    .data
                    .nav;
            }

            info!("Starting mining astroid");

            'inner: for mount in ship_handler_data
                .lock()
                .await
                .ships
                .get(ship_id)
                .unwrap()
                .mounts
                .iter()
            {
                if mount.symbol == enums::ShipMount::MountSurveyorI
                    || mount.symbol == enums::ShipMount::MountSurveyorIi
                    || mount.symbol == enums::ShipMount::MountSurveyorIii
                {
                    let surveys = ship_handler_data
                        .lock()
                        .await
                        .spacetraders
                        .create_survey(ship_id)
                        .await
                        .data;
                    ship_handler_data.lock().await.surveys.push(surveys);
                    break 'inner;
                }
            }
            let temp_ship_data = ship_handler_data
                .lock()
                .await
                .spacetraders
                .extract_resources(ship_id, None)
                .await
                .data;

            ship_handler_data
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
                        travel_waypoint(
                            ship_id,
                            ship_handler_data.clone(),
                            waypoint.symbol.waypoint.as_str(),
                        )
                        .await;

                        let mut ship_handler_data_u = ship_handler_data.lock().await;

                        if ship_handler_data_u.ships.get(ship_id).unwrap().nav.status
                            == enums::ShipNavStatus::InOrbit
                        {
                            ship_handler_data_u.ships.get_mut(ship_id).unwrap().nav =
                                ship_handler_data_u
                                    .spacetraders
                                    .dock_ship(ship_id)
                                    .await
                                    .data
                                    .nav;
                        }

                        // TODO: make sure not to sell goods used for contracts
                        // TODO: also make sure I can sell that good here
                        for item in ship_handler_data_u
                            .ships
                            .get(ship_id)
                            .unwrap()
                            .cargo
                            .inventory
                            .clone()
                            .iter()
                        {
                            info!("{} is selling {} {:?}", ship_id, item.units, item.symbol);

                            let temp_ship_data = ship_handler_data_u
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

                            ship_handler_data_u.ships.get_mut(ship_id).unwrap().cargo =
                                temp_ship_data.cargo;
                            let (_agent, transaction) =
                                (temp_ship_data.agent, temp_ship_data.transaction);

                            ship_handler_data_u.credits += transaction.units;
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
