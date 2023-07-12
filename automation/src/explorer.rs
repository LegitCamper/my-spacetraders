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

// pub async fn explore(ship_handler_data: Arc<Mutex<ShipHandlerData>>) {

// }
