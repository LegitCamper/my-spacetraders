use super::ShipHandlerData;
use spacetraders::{
    //contracts
    // SpaceTraders,
    enums::{self, WaypointTrait},
    requests,
    responses::{contracts::Contract, fleet, schemas},
    SpaceTraders,
    System,
    Waypoint,
};

// use async_recursion::async_recursion;
use chrono::{offset, DateTime, Local};
use log::{info, trace};
use std::{collections::HashMap, sync::Arc};
use tokio::{
    sync::Mutex,
    time::{sleep, Duration},
};

#[derive(Debug, Clone)]
pub struct ShipDataAbstractor(pub Arc<Mutex<ShipHandlerData>>);
impl ShipDataAbstractor {
    pub fn new(ship_handler_data: Arc<Mutex<ShipHandlerData>>) -> Self {
        ShipDataAbstractor(ship_handler_data)
    }

    pub async fn get_credits(&self) -> f64 {
        self.0.lock().await.credits
    }
    pub async fn add_credits(&self, credits: f64) {
        self.0.lock().await.credits += credits;
    }
    pub async fn sub_credits(self, credits: f64) {
        self.0.lock().await.credits -= credits;
    }

    pub async fn clone_ship(&self, ship_id: &str) -> Option<schemas::Ship> {
        self.0.lock().await.ships.get(ship_id).cloned()
    }

    pub async fn clone_ships(&self) -> HashMap<String, schemas::Ship> {
        self.0.lock().await.ships.clone()
    }

    pub async fn system_distance(&self, system1: System, system2: System) -> Option<u64> {
        for system in self.0.lock().await.euclidean_distances.iter() {
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
        self.0.lock().await.contracts.get(contract_id).cloned()
    }
    pub async fn remove_contract(&self, contract_id: &str) -> Option<schemas::Contract> {
        self.0.lock().await.contracts.remove(contract_id)
    }
    pub async fn add_contract(
        &self,
        contract_id: &str,
        contract: schemas::Contract,
    ) -> Option<schemas::Contract> {
        self.0
            .lock()
            .await
            .contracts
            .insert(contract_id.to_string(), contract)
    }

    pub async fn get_survey(&self, waypoint: Waypoint) -> Option<fleet::CreateSurveyData> {
        self.0.lock().await.surveys.get(&waypoint).cloned()
    }
    pub async fn remove_survey(&self, waypoint: Waypoint) -> Option<fleet::CreateSurveyData> {
        self.0.lock().await.surveys.remove(&waypoint)
    }
    pub async fn create_survey(&self, ship_id: &str) -> Option<Vec<schemas::Survey>> {
        let unlocked = self.0.lock().await;
        let ship_posistion = unlocked
            .ships
            .get(ship_id)
            .unwrap()
            .nav
            .waypoint_symbol
            .clone();
        let survey = unlocked.spacetraders.create_survey(ship_id).await;
        self.0
            .lock()
            .await
            .surveys
            .insert(ship_posistion, survey.data.clone());
        //TODO: when spacetraders lib has better error handling consider returning none here
        Some(survey.data.surveys)
    }

    pub async fn orbit_ship(&self, ship_id: &str) {
        let mut unlocked = self.0.lock().await;
        let ship = unlocked.spacetraders.orbit_ship(ship_id).await;
        unlocked.ships.get_mut(ship_id).unwrap().nav = ship.data.nav;
    }

    pub async fn dock_ship(&self, ship_id: &str) {
        let mut unlocked = self.0.lock().await;
        let ship = unlocked.spacetraders.dock_ship(ship_id).await;
        unlocked.ships.get_mut(ship_id).unwrap().nav = ship.data.nav;
    }

    pub async fn get_waypoint(&self, waypoint: Waypoint) -> schemas::Waypoint {
        let mut unlocked = self.0.lock().await;
        match unlocked.waypoints.get(&waypoint) {
            Some(data) => data.clone(),
            None => {
                let new_waypoint = unlocked
                    .spacetraders
                    .get_waypoint(waypoint.to_system(), waypoint)
                    .await
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
    pub async fn get_waypoints(&self, system: System) -> Vec<schemas::Waypoint> {
        let mut unlocked = self.0.lock().await;

        let mut waypoints = unlocked
            .spacetraders
            .list_waypoints(system.clone(), None)
            .await;
        if waypoints.meta.total > 1 {
            for num in 2..waypoints.meta.total {
                let paged_waypoints = unlocked
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

    pub async fn chart_waypoint(&self, ship_id: &str) {
        let unlocked = self.0.lock().await;

        let ship_location = unlocked
            .ships
            .get(ship_id)
            .unwrap()
            .nav
            .waypoint_symbol
            .clone();

        let ship = unlocked.waypoints.clone();
        if ship.get(&ship_location).is_none() {
            let waypoint = unlocked
                .spacetraders
                .get_waypoint(ship_location.to_system(), ship_location)
                .await;

            if waypoint.data.chart.submitted_by.is_empty() {
                unlocked.spacetraders.create_chart(ship_id).await;
            }
        }
    }

    async fn wait_duration(&self, ship_id: &str) {
        let local_time_to_stop: DateTime<Local> = self
            .0
            .lock()
            .await
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
            self.0.lock().await.ships.get(ship_id).unwrap().symbol,
            duration.num_seconds()
        );

        sleep(Duration::from_secs(
            duration.num_seconds().try_into().unwrap(),
        ))
        .await;
    }

    pub async fn travel_waypoint(&self, ship_id: &str, waypoint: &str) -> Option<schemas::Ship> {
        self.chart_waypoint(ship_id).await;

        let ship = self.0.lock().await.ships.get(ship_id).unwrap().clone();

        self.get_fuel(ship_id).await;

        if ship.nav.waypoint_symbol.waypoint != waypoint {
            // there is also a case where the ship is in transit and neither docked or there

            if ship.nav.status == enums::ShipNavStatus::Docked {
                self.orbit_ship(ship_id).await;
            }
            //TODO: consider fuel types here - eg stealth, drift
            let temp_ship_data = self
                .0
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

            (
                self.0.lock().await.ships.get_mut(ship_id).unwrap().nav,
                self.0.lock().await.ships.get_mut(ship_id).unwrap().fuel,
            ) = (temp_ship_data.nav, temp_ship_data.fuel);

            self.wait_duration(ship_id).await;

            self.chart_waypoint(ship_id).await;

            self.get_fuel(ship_id).await;
        }
        self.clone_ship(ship_id).await
    }

    // TODO: needs work
    // should consider fuel prices and other locations
    pub async fn get_fuel(&self, ship_id: &str) {
        let ship = self.clone_ship(ship_id).await.unwrap();
        let waypoint = self.get_waypoint(ship.nav.waypoint_symbol).await;

        for r#trait in waypoint.traits.iter() {
            if r#trait.symbol == enums::WaypointTrait::Marketplace {
                if ship.fuel.current != ship.fuel.capacity {
                    if ship.nav.status == enums::ShipNavStatus::Docked {
                        self.0.lock().await.spacetraders.refuel_ship(ship_id).await;
                    } else if ship.nav.status == enums::ShipNavStatus::InOrbit {
                        self.dock_ship(ship_id).await;
                        self.0.lock().await.spacetraders.refuel_ship(ship_id).await;
                        self.orbit_ship(ship_id).await;
                    }
                }
            }
        }
    }

    pub async fn travel_system(&self, ship_id: &str, waypoint: &str) {
        trace!("travel");

        let ship = self.0.lock().await.ships.get(ship_id).unwrap().clone();

        // TODO: refuel sometime
        if ship.nav.waypoint_symbol.waypoint != waypoint {
            // there is also a case where the ship is in transit and neither docked or there

            if ship.nav.status == enums::ShipNavStatus::Docked {
                self.orbit_ship(ship_id).await;
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
}
