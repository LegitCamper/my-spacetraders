use super::ShipHandler;
use spacetraders::{
    //contracts
    // SpaceTraders,
    enums,
    requests,
    responses::schemas,
    SystemString,
    WaypointString,
};

// use async_recursion::async_recursion;
use chrono::{offset, DateTime, Local};
use log::{error, info, trace};
use std::{collections::HashMap, sync::Arc};
use tokio::{
    sync::Mutex,
    time::{sleep, Duration},
};

#[derive(Debug, Clone)]
pub struct ShipWrapper {
    pub ship_handler: Arc<Mutex<ShipHandler>>,
    pub ship_id: String,
}
impl ShipWrapper {
    pub fn new(ship_handler: Arc<Mutex<ShipHandler>>, ship_id: &str) -> Self {
        ShipWrapper {
            ship_handler,
            ship_id: ship_id.to_string(),
        }
    }

    pub async fn get_credits(&self) -> f64 {
        trace!("Get Credits");
        self.ship_handler.lock().await.credits
    }
    pub async fn add_credits(&self, credits: f64) {
        trace!("Add Credits");
        self.ship_handler.lock().await.credits += credits;
    }
    pub async fn sub_credits(self, credits: f64) {
        trace!("Sub Credits");
        self.ship_handler.lock().await.credits -= credits;
    }

    pub async fn clone_ship(&self) -> Option<schemas::Ship> {
        trace!("Clone ship");
        self.ship_handler
            .lock()
            .await
            .ships
            .get(&self.ship_id)
            .cloned()
    }

    pub async fn clone_ships(&self) -> HashMap<String, schemas::Ship> {
        trace!("Clone ships");
        self.ship_handler.lock().await.ships.clone()
    }

    pub async fn system_distance(
        &self,
        system1: &SystemString,
        system2: &SystemString,
    ) -> Option<u64> {
        trace!("System Distance");
        for system in self.ship_handler.lock().await.euclidean_distances.iter() {
            if system1.system == system.name {
                for distances in system.euclidean_distance.iter() {
                    if system2.system == distances.name {
                        return Some(distances.distance);
                    }
                }
            }
        }
        None
    }

    pub async fn get_contract(&self, contract_id: &str) -> Option<schemas::Contract> {
        trace!("Get Contract");
        self.ship_handler
            .lock()
            .await
            .contracts
            .get(contract_id)
            .cloned()
    }
    pub async fn remove_contract(&self, contract_id: &str) -> Option<schemas::Contract> {
        trace!("Remove Contract");
        self.ship_handler.lock().await.contracts.remove(contract_id)
    }
    pub async fn add_contract(
        &self,
        contract_id: &str,
        contract: schemas::Contract,
    ) -> Option<schemas::Contract> {
        trace!("Add Contract");
        self.ship_handler
            .lock()
            .await
            .contracts
            .insert(contract_id.to_string(), contract)
    }

    pub async fn remove_survey(&self, waypoint: &WaypointString) -> Option<schemas::Survey> {
        trace!("Remove Survey");
        self.ship_handler
            .lock()
            .await
            .surveys
            .remove(waypoint)
            .map(|surveys| surveys[0].to_owned())
    }
    pub async fn create_survey(&self) -> Option<schemas::Survey> {
        trace!("Create Survey");

        let ship = self.clone_ship().await.unwrap();
        let mut unlocked = self.ship_handler.lock().await;
        let survey = unlocked.surveys.get(&ship.nav.waypoint_symbol);

        if survey.is_some() {
            let survey = survey.unwrap();
            if !survey.is_empty() {
                Some(survey[0].clone())
            // TODO:       ^^^^ maybe do something fancier here - should check if this is expired
            } else {
                drop(unlocked);
                self.remove_survey(&ship.nav.waypoint_symbol).await;
                None
            }
        } else {
            for mount in ship.mounts.iter() {
                if mount.symbol == enums::ShipMount::MountSurveyorI
                    || mount.symbol == enums::ShipMount::MountSurveyorIi
                    || mount.symbol == enums::ShipMount::MountSurveyorIii
                {
                    let ship_posistion = unlocked
                        .ships
                        .get(&self.ship_id)
                        .unwrap()
                        .nav
                        .waypoint_symbol
                        .clone();
                    let survey = unlocked
                        .spacetraders
                        .create_survey(&self.ship_id)
                        .await
                        .unwrap();

                    match unlocked.surveys.get(&ship.nav.waypoint_symbol) {
                        Some(_) => {
                            for survey in survey.data.surveys.iter() {
                                unlocked
                                    .surveys
                                    .get_mut(&ship.nav.waypoint_symbol)
                                    .unwrap()
                                    .push(survey.clone());
                            }
                        }
                        None => {
                            unlocked
                                .surveys
                                .insert(ship_posistion, survey.data.surveys.clone());
                        }
                    }
                    return Some(survey.data.surveys[0].clone());
                } else {
                    return None;
                }
            }
            None
        }
    }

    pub async fn orbit_ship(&self) {
        trace!("Orbit Ship");
        let ship = self
            .ship_handler
            .lock()
            .await
            .spacetraders
            .orbit_ship(&self.ship_id)
            .await
            .unwrap();
        self.ship_handler
            .lock()
            .await
            .ships
            .get_mut(&self.ship_id)
            .unwrap()
            .nav = ship.data.nav;
        self.wait_flight_duration().await;
    }

    pub async fn dock_ship(&self) {
        trace!("Dock Ship");
        let ship = self
            .ship_handler
            .lock()
            .await
            .spacetraders
            .dock_ship(&self.ship_id)
            .await
            .unwrap();
        self.ship_handler
            .lock()
            .await
            .ships
            .get_mut(&self.ship_id)
            .unwrap()
            .nav = ship.data.nav;
        self.wait_flight_duration().await;
    }

    pub async fn get_waypoint(&self, waypoint: &WaypointString) -> schemas::Waypoint {
        trace!("Get Waypoint");
        let mut unlocked = self.ship_handler.lock().await;
        match unlocked.waypoints.get(waypoint) {
            Some(data) => data.clone(),
            None => {
                let new_waypoint = unlocked
                    .spacetraders
                    .get_waypoint(&waypoint.to_system(), waypoint)
                    .await
                    .unwrap()
                    .data;
                if new_waypoint.chart.submitted_by.is_empty() {
                    new_waypoint
                } else {
                    unlocked
                        .waypoints
                        .insert(new_waypoint.symbol.clone(), new_waypoint.clone());
                    new_waypoint
                }
            }
        }
    }
    pub async fn get_waypoints(&self, system: &SystemString) -> Vec<schemas::Waypoint> {
        trace!("Get Waypoints");
        let mut unlocked = self.ship_handler.lock().await;

        let waypoints = unlocked
            .spacetraders
            .list_waypoints(system, false)
            .await
            .unwrap();
        let mut return_vec = Vec::new();
        for new_waypoint in waypoints.data.iter() {
            let waypoints = unlocked.waypoints.clone();

            match waypoints.get(&new_waypoint.symbol) {
                Some(data) => return_vec.push(data.clone()),
                None => {
                    if new_waypoint.chart.submitted_by.is_empty() {
                        return_vec.push(new_waypoint.clone())
                    } else {
                        unlocked
                            .waypoints
                            .insert(new_waypoint.symbol.clone(), new_waypoint.clone());
                        return_vec.push(new_waypoint.clone());
                    }
                }
            }
        }
        return_vec
    }

    pub async fn chart_waypoint(&self) {
        trace!("Chart Waypoint");
        let unlocked = self.ship_handler.lock().await;

        let ship_location = unlocked
            .ships
            .get(&self.ship_id)
            .unwrap()
            .nav
            .waypoint_symbol
            .clone();

        let ship = unlocked.waypoints.clone();
        if ship.get(&ship_location).is_none() {
            let waypoint = unlocked
                .spacetraders
                .get_waypoint(&ship_location.to_system(), &ship_location)
                .await
                .unwrap();

            if waypoint.data.chart.submitted_by.is_empty() {
                let _ = unlocked.spacetraders.create_chart(&self.ship_id).await;
            }
        }
    }

    pub async fn wait_flight_duration(&self) {
        trace!("Wait Durtation");
        let local_time_to_stop: DateTime<Local> = self
            .ship_handler
            .lock()
            .await
            .ships
            .get(&self.ship_id)
            .unwrap()
            .nav
            .route
            .arrival
            .into();
        let local_time_now: DateTime<Local> = offset::Utc::now().into();
        let duration: chrono::Duration = local_time_to_stop - local_time_now;

        if duration.num_milliseconds() > 0 {
            info!(
                "{} is going to sleep for {} seconds",
                self.ship_handler
                    .lock()
                    .await
                    .ships
                    .get(&self.ship_id)
                    .unwrap()
                    .symbol,
                duration.num_seconds()
            );

            sleep(Duration::from_secs(
                duration.num_seconds().try_into().unwrap(),
            ))
            .await;
        }
    }

    pub async fn travel_waypoint(&self, waypoint: &str) -> Option<schemas::Ship> {
        trace!("Travel Waypoint");
        self.chart_waypoint().await;

        let ship = self
            .ship_handler
            .lock()
            .await
            .ships
            .get(&self.ship_id)
            .unwrap()
            .clone();

        self.get_fuel(ship.fuel.consumed.amount.try_into().unwrap())
            .await;

        if ship.nav.waypoint_symbol.waypoint != waypoint {
            if ship.nav.status == enums::ShipNavStatus::Docked {
                self.orbit_ship().await;
            } else if ship.nav.status == enums::ShipNavStatus::InTransit {
                self.wait_flight_duration().await;
                self.orbit_ship().await;
            }
            //TODO: consider fuel types here - eg stealth, drift
            let temp_ship_data = self
                .ship_handler
                .lock()
                .await
                .spacetraders
                .navigate_ship(
                    &self.ship_id,
                    requests::NavigateShip {
                        waypoint_symbol: waypoint.to_string(),
                    },
                )
                .await;

            if temp_ship_data.is_ok() {
                let temp_ship_data = temp_ship_data.unwrap().data;

                (
                    self.ship_handler
                        .lock()
                        .await
                        .ships
                        .get_mut(&self.ship_id)
                        .unwrap()
                        .nav,
                    self.ship_handler
                        .lock()
                        .await
                        .ships
                        .get_mut(&self.ship_id)
                        .unwrap()
                        .fuel,
                ) = (temp_ship_data.nav, temp_ship_data.fuel);

                self.wait_flight_duration().await;

                self.chart_waypoint().await;

                self.get_fuel(ship.fuel.consumed.amount.try_into().unwrap())
                    .await;
            }
        }
        self.clone_ship().await
    }

    // TODO: needs work
    // should consider fuel prices and other locations
    pub async fn get_fuel(&self, fuel_amount: i32) {
        let ship = self.clone_ship().await.unwrap();
        let waypoint = self.get_waypoint(&ship.nav.waypoint_symbol).await;

        for r#trait in waypoint.traits.iter() {
            if r#trait.symbol == enums::WaypointTrait::Marketplace {
                let market = self
                    .ship_handler
                    .lock()
                    .await
                    .spacetraders
                    .get_market(&waypoint.system_symbol, &waypoint.symbol)
                    .await
                    .unwrap()
                    .data;
                for tradegood in market.exports.iter() {
                    if tradegood.symbol == enums::TradeSymbol::Fuel
                        && ship.fuel.current != ship.fuel.capacity
                    {
                        if ship.nav.status == enums::ShipNavStatus::Docked {
                            for _ in 0..((ship.fuel.capacity as f32 / 100_f32).ceil() as u32) {
                                let _ = self
                                    .ship_handler
                                    .lock()
                                    .await
                                    .spacetraders
                                    .refuel_ship(
                                        &self.ship_id,
                                        Ok(requests::RefuelShip { units: fuel_amount }),
                                    )
                                    .await;
                            }
                        } else if ship.nav.status == enums::ShipNavStatus::InOrbit {
                            self.dock_ship().await;
                            let _ = self
                                .ship_handler
                                .lock()
                                .await
                                .spacetraders
                                .refuel_ship(
                                    &self.ship_id,
                                    Ok(requests::RefuelShip { units: fuel_amount }),
                                )
                                .await;
                            self.orbit_ship().await;
                        }
                        return;
                    }
                }
            }
        }
    }

    pub fn euclidean_distance(&self, x1: i32, y1: i32, x2: i32, y2: i32) -> u64 {
        trace!("ship_data Euclidean Distance");
        let distance: f64 =
            ((x1 as f64 - y1 as f64).powi(2) + (x2 as f64 - y2 as f64).powi(2)).sqrt();
        let distance: u64 = distance.round() as u64;
        distance
    }

    pub async fn travel_system(&self, waypoint: &str) {
        trace!("travel");

        let ship = self
            .ship_handler
            .lock()
            .await
            .ships
            .get(&self.ship_id)
            .unwrap()
            .clone();

        // TODO: refuel sometime
        if ship.nav.waypoint_symbol.waypoint != waypoint {
            // there is also a case where the ship is in transit and neither docked or there

            if ship.nav.status == enums::ShipNavStatus::Docked {
                self.orbit_ship().await;
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

    pub async fn extract_resources(
        &self,
    ) -> Option<(schemas::ShipCargo, schemas::Cooldown, schemas::Extraction)> {
        let survey = self.create_survey().await;
        let ship = match self
            .ship_handler
            .lock()
            .await
            .spacetraders
            .extract_resources(&self.ship_id, survey)
            .await
        {
            Ok(data) => Some(data.data),
            Err(_) => {
                error!("{} Failed to extract resources", self.ship_id);
                None
            }
        };

        if let Some(ship) = ship {
            self.ship_handler
                .lock()
                .await
                .ships
                .get_mut(&self.ship_id)
                .unwrap()
                .cargo = ship.cargo.clone();
            let (cooldown, extraction) = (ship.cooldown, ship.extraction);
            Some((ship.cargo, cooldown, extraction))
        } else {
            None
        }
    }
    // pub async fn pathfind(self)
    // pub async fn buy_mount(self, module: enums::ShipModule) {
    //     let ship = self.clone_ship().await.unwrap();
    //     for r#trait in self
    //         .get_waypoint(&ship.nav.waypoint_symbol)
    //         .await
    //         .traits
    //         .iter()
    //     {
    //         if r#trait == enums::WaypointTrait::Marketplace {
    //             self.buy_from_marketplace(module).await
    //         }
    //     }
    // }
    // pub async fn buy_from_marketplace(self, trade_good: enums::ShipModule) { // enums::TradeSymbol
    //                                                                          // TODO: need to write an adapter to convert enums::ShipModule to enums::TradeSymbol
    //                                                                          // This asumes you are already at the marketplace
    //     self.ship_handler.lock().await.spacetraders.
    // }
    // pub async fn get_market()
    // TODO: cache market data
}
