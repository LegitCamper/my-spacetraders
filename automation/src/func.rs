use super::ShipHandlerData;
use spacetraders::{
    //contracts
    // SpaceTraders,
    enums,
    requests,
    responses::{self, schemas},
    System,
    Waypoint,
};

// use async_recursion::async_recursion;
use chrono::{offset, DateTime, Local};
use log::{info, trace, warn};
use std::sync::Arc;
use tokio::{
    sync::{mpsc, Mutex},
    time::{sleep, Duration},
};

// TODO: implement copy instead for all custom structs

async fn wait_duration(ship_id: &str, ship_handler_data: Arc<Mutex<ShipHandlerData>>) {
    trace!("Waiting duration");
    let ship_handler_data_u = ship_handler_data.lock().await;

    let local_time_to_stop: DateTime<Local> = ship_handler_data_u
        .ships
        .get(ship_id)
        .unwrap()
        .nav
        .route
        .arrival
        .into();
    let local_time_now: DateTime<Local> = offset::Utc::now().into();
    let duration: chrono::Duration = local_time_to_stop - local_time_now;

    info!(
        "{} is moving - going to sleep for {} seconds",
        ship_handler_data_u.ships.get(ship_id).unwrap().symbol,
        duration.num_seconds()
    );

    sleep(Duration::from_secs(
        duration.num_seconds().try_into().unwrap(),
    ))
    .await;
}

pub async fn chart_waypoint(ship_id: &str, ship_handler_data: Arc<Mutex<ShipHandlerData>>) {
    let ship_location = ship_handler_data
        .lock()
        .await
        .ships
        .get(ship_id)
        .unwrap()
        .nav
        .waypoint_symbol
        .clone();
    if ship_handler_data
        .lock()
        .await
        .waypoints
        .get(&ship_location)
        .is_none()
    {
        let waypoint = ship_handler_data
            .lock()
            .await
            .spacetraders
            .get_waypoint(ship_location.to_system(), ship_location)
            .await;

        if waypoint.data.chart.submitted_by.is_empty() {
            ship_handler_data
                .lock()
                .await
                .spacetraders
                .create_chart(ship_id)
                .await;
        }
    }
}

pub async fn travel_waypoint(
    ship_id: &str,
    ship_handler_data: Arc<Mutex<ShipHandlerData>>,
    waypoint: &str,
) {
    trace!("Travel Waypoint");

    chart_waypoint(ship_id, ship_handler_data.clone()).await;

    // TODO: refuel sometime
    if ship_handler_data
        .lock()
        .await
        .ships
        .get(ship_id)
        .unwrap()
        .nav
        .waypoint_symbol
        .waypoint
        != waypoint
    {
        // there is also a case where the ship is in transit and neither docked or there
        if ship_handler_data
            .lock()
            .await
            .ships
            .get(ship_id)
            .unwrap()
            .nav
            .status
            == enums::ShipNavStatus::Docked
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
        //TODO: consider fuel types here - eg stealth, drift
        let temp_ship_data = ship_handler_data
            .lock()
            .await
            .spacetraders
            .navigate_ship(
                ship_id,
                requests::NavigateShip {
                    waypoint_symbol: waypoint.to_string(),
                },
            )
            .await
            .data;

        {
            let mut ship_handler_data_u = ship_handler_data.lock().await;
            (
                ship_handler_data_u.ships.get_mut(ship_id).unwrap().nav,
                ship_handler_data_u.ships.get_mut(ship_id).unwrap().fuel,
            ) = (temp_ship_data.nav, temp_ship_data.fuel);
        }

        wait_duration(ship_id, ship_handler_data.clone()).await;

        chart_waypoint(ship_id, ship_handler_data.clone()).await;
    }
}

#[allow(dead_code)]
pub async fn travel_system(
    ship_id: &str,
    ship_handler_data: Arc<Mutex<ShipHandlerData>>,
    waypoint: &str,
) {
    trace!("travel");
    let mut ship_handler_data_u = ship_handler_data.lock().await;

    // TODO: refuel before traveling
    if ship_handler_data_u
        .ships
        .get(ship_id)
        .unwrap()
        .nav
        .waypoint_symbol
        .system
        != waypoint
    {
        // there is also a case where the ship is in transit and neither docked or there
        let ship_status = ship_handler_data_u.ships.get(ship_id).unwrap().nav.status;
        if ship_status == enums::ShipNavStatus::Docked {
            ship_handler_data_u.ships.get_mut(ship_id).unwrap().nav = ship_handler_data_u
                .spacetraders
                .orbit_ship(ship_id)
                .await
                .data
                .nav;
        }

        // depending on whether there is a warp drive or jump drive determines the endpoint to use
        // also ensure to check if there is a jump gate

        // let time_to_stop = ship_handler_data
        //     .spacetraders
        //     .navigate_ship(
        //         &ship_details.data.symbol,
        //         requests::NavigateShip {
        //             waypoint_symbol: waypoint.waypoint.clone(),
        //         },
        //     )
        //     .await;

        // wait_duration(time_to_stop.data.nav.route.arrival).await;
    }
}

// TODO: make sure for the following if there are more pages of waypoints you ensure to donwload them aswell
pub async fn get_waypoint(
    waypoint: Waypoint,
    ship_handler_data: Arc<Mutex<ShipHandlerData>>,
) -> schemas::Waypoint {
    trace!("Get Waypoint");

    match ship_handler_data.lock().await.waypoints.get(&waypoint) {
        Some(data) => data.clone(),
        None => {
            let new_waypoint = ship_handler_data
                .lock()
                .await
                .spacetraders
                .get_waypoint(waypoint.to_system(), waypoint)
                .await
                .data;
            if new_waypoint.chart.submitted_by.is_empty() {
                new_waypoint
            } else {
                ship_handler_data
                    .lock()
                    .await
                    .waypoints
                    .insert(new_waypoint.symbol.clone(), new_waypoint.clone());
                new_waypoint
            }
        }
    }
}

pub async fn get_waypoints(
    system: System,
    ship_handler_data: Arc<Mutex<ShipHandlerData>>,
) -> Vec<schemas::Waypoint> {
    let mut waypoints = ship_handler_data
        .lock()
        .await
        .spacetraders
        .list_waypoints(system.clone(), None)
        .await;

    if waypoints.meta.total > 1 {
        for num in 2..waypoints.meta.total {
            let paged_waypoints = ship_handler_data
                .lock()
                .await
                .spacetraders
                .list_waypoints(system.clone(), Some(num))
                .await
                .data;
            for paged_waypoint in paged_waypoints.iter() {
                waypoints.data.push(paged_waypoint.clone())
            }
        }
    }
    let mut return_vec = Vec::new();
    for new_waypoint in waypoints.data.iter() {
        match ship_handler_data
            .lock()
            .await
            .waypoints
            .get(&new_waypoint.symbol)
        {
            Some(data) => return_vec.push(data.clone()),
            None => {
                if new_waypoint.chart.submitted_by.is_empty() {
                    return_vec.push(new_waypoint.clone())
                } else {
                    ship_handler_data
                        .lock()
                        .await
                        .waypoints
                        .insert(new_waypoint.symbol.clone(), new_waypoint.clone());
                    return_vec.push(new_waypoint.clone());
                }
            }
        }
    }
    return_vec
}
