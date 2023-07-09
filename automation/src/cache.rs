use spacetraders::{responses::schemas, SpaceTraders};

use async_recursion::async_recursion;
use chrono::{serde::ts_milliseconds, DateTime, Utc};
// use itertools::Itertools;
use log::{info, trace};
// use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
    fs::{read_to_string, remove_file, File},
    path::Path,
};

#[derive(Debug, Serialize, Deserialize)]
struct SystemDB {
    #[serde(with = "ts_milliseconds")]
    date: DateTime<Utc>,
    data: Vec<schemas::System>,
}

const SYSTEMDB_FILE: &str = "systemDB.json";

#[async_recursion]
pub async fn build_system_db(space_traders: &SpaceTraders) -> Vec<schemas::System> {
    trace!("Building system DB");

    if Path::new(SYSTEMDB_FILE).is_file() {
        let systems: SystemDB =
            match serde_json::from_str::<SystemDB>(&read_to_string(SYSTEMDB_FILE).unwrap()) {
                Err(_) => {
                    info!("removing currupted {}", SYSTEMDB_FILE);

                    remove_file(SYSTEMDB_FILE).unwrap();
                    return build_system_db(space_traders).await;
                }
                Ok(data) => {
                    info!("{} integrity check good", SYSTEMDB_FILE);

                    if data.date < space_traders.get_status().await.reset_date {
                        info!("{} is outdated", SYSTEMDB_FILE);
                        remove_file(SYSTEMDB_FILE).unwrap();
                        return build_system_db(space_traders).await;
                    }

                    data
                }
            };

        systems.data
    } else {
        info!("{} does not exist - building ", SYSTEMDB_FILE);
        // let num_systems = space_traders_unlocked.get_status().await.stats.systems; // TODO: this currently does not work, but should replace below
        let num_systems = space_traders.list_systems(None).await.meta.total;
        info!(
            "There are ~{} systems - Building will take ~{} minute(s)",
            num_systems,
            num_systems / 2400 // = 20 per page every 500 milliseconds / 60 min
        );

        let mut systems: Vec<schemas::System> = Vec::new();

        for page in 1..((num_systems / 20) + 1) {
            for system in space_traders.list_systems(Some(page)).await.data.iter() {
                systems.push(system.clone());
            }
        }
        info!("Writing new systems to {}", SYSTEMDB_FILE);

        serde_json::to_writer_pretty(
            &File::create(SYSTEMDB_FILE).unwrap(),
            &SystemDB {
                date: chrono::offset::Utc::now(),
                data: systems.clone(),
            },
        )
        .unwrap();

        info!("{} systems in db", systems.len());
        systems
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SerdeEuclideanDistances {
    date: DateTime<Utc>,
    distances: Vec<AllEuclideanDistances>,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AllEuclideanDistances {
    name: String,
    x: i32,
    y: i32,
    euclidean_distance: Vec<EuclideanDistances>,
}

const DISTANCESDB_FILE: &str = "distancesDB.json";

#[async_recursion]
pub async fn build_euclidean_distance(
    systems_db: Vec<schemas::System>,
    space_traders: &SpaceTraders,
) -> Vec<AllEuclideanDistances> {
    trace!("Building Euclidean Distances");

    if Path::new(DISTANCESDB_FILE).is_file() {
        let distances: SerdeEuclideanDistances = match serde_json::from_str::<SerdeEuclideanDistances>(
            &read_to_string(DISTANCESDB_FILE).unwrap(),
        ) {
            Err(_) => {
                info!("removing currupted {}", DISTANCESDB_FILE);

                remove_file(DISTANCESDB_FILE).unwrap();
                return build_euclidean_distance(systems_db, space_traders).await;
            }
            Ok(data) => {
                info!("{} integrity check good", DISTANCESDB_FILE);

                if data.date < space_traders.get_status().await.reset_date {
                    info!("{} is outdated", DISTANCESDB_FILE);
                    remove_file(DISTANCESDB_FILE).unwrap();
                    return build_euclidean_distance(systems_db, space_traders).await;
                }

                data
            }
        };

        distances.distances
    } else {
        info!("{} does not exist - building ", DISTANCESDB_FILE);

        let mut all_euclidean_distance: Vec<AllEuclideanDistances> = Vec::new();

        for system in systems_db.iter() {
            // consider using rayon here
            all_euclidean_distance.push(AllEuclideanDistances {
                name: system.symbol.clone(),
                x: system.x,
                y: system.y,
                euclidean_distance: euclidean_distance(
                    system,
                    &systems_db,
                    Some(systems_db.len().try_into().unwrap()),
                ),
            });
        }

        info!("Writing new distances to {}", DISTANCESDB_FILE);

        serde_json::to_writer_pretty(
            &File::create(DISTANCESDB_FILE).unwrap(),
            &SerdeEuclideanDistances {
                date: chrono::offset::Utc::now(),
                distances: all_euclidean_distance.clone(),
            },
        )
        .unwrap();

        all_euclidean_distance
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EuclideanDistances {
    distance: u64,
    name: String,
    x: i32,
    y: i32,
}

// TODO: this has to be super unoptimized. takes way to long
fn euclidean_distance(
    current_system: &schemas::System,
    systems: &Vec<schemas::System>,
    num_returns: Option<u32>,
) -> Vec<EuclideanDistances> {
    trace!("Euclidean Distance Caluclations");
    let num_systems_to_return = match num_returns {
        Some(num) => num,
        None => 5,
    };

    let mut closest_systems: Vec<EuclideanDistances> = Vec::new();
    let (my_x, my_y) = (current_system.x, current_system.y);

    for system in systems.iter() {
        let (x, y) = (system.x, system.y);

        let distance: f64 =
            ((my_x as f64 - my_y as f64).powi(2) + (x as f64 - y as f64).powi(2)).sqrt();
        // giving up trying to do this with floats
        // I am going to round and hope it works out
        let distance: u64 = distance.round() as u64;

        if closest_systems.is_empty() {
            closest_systems.push(EuclideanDistances {
                distance,
                x,
                y,
                name: system.symbol.clone(),
            });
        } else {
            'inner: for (index, system_distance) in closest_systems.iter().enumerate() {
                if distance < system_distance.distance || distance == system_distance.distance {
                    closest_systems.insert(
                        index,
                        EuclideanDistances {
                            distance,
                            x,
                            y,
                            name: system.symbol.clone(),
                        },
                    );
                    if closest_systems.len() >= num_systems_to_return.try_into().unwrap() {
                        closest_systems.pop();
                    }
                    break 'inner;
                }
            }
        }
    }
    closest_systems
}
