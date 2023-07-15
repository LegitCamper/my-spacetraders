use spacetraders::{
    responses::{schemas, systems},
    SpaceTraders,
};

use async_recursion::async_recursion;
use chrono::{DateTime, Utc};
use log::{info, trace};
// use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
    fs::{read_to_string, remove_file, File},
    path::Path,
    sync::Arc,
};
use tokio::sync::Mutex;

#[derive(Debug, Deserialize, Serialize)]
pub struct SerdeEuclideanDistances {
    date: DateTime<Utc>,
    distances: Vec<AllEuclideanDistances>,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AllEuclideanDistances {
    pub name: String,
    pub x: i32,
    pub y: i32,
    pub euclidean_distance: Vec<EuclideanDistances>,
}

const DISTANCESDB_FILE: &str = "distancesDB.json";

#[async_recursion]
pub async fn build_euclidean_distance(space_traders: &SpaceTraders) -> Vec<AllEuclideanDistances> {
    trace!("Building Euclidean Distances");

    if Path::new(DISTANCESDB_FILE).is_file() {
        let distances: SerdeEuclideanDistances = match serde_json::from_str::<SerdeEuclideanDistances>(
            &read_to_string(DISTANCESDB_FILE).unwrap(),
        ) {
            Err(_) => {
                info!("removing currupted {}", DISTANCESDB_FILE);

                remove_file(DISTANCESDB_FILE).unwrap();
                return build_euclidean_distance(space_traders).await;
            }
            Ok(data) => {
                info!("{} integrity check good", DISTANCESDB_FILE);

                if data.date < space_traders.get_status().await.unwrap().reset_date {
                    info!("{} is outdated", DISTANCESDB_FILE);
                    remove_file(DISTANCESDB_FILE).unwrap();
                    return build_euclidean_distance(space_traders).await;
                }

                data
            }
        };

        distances.distances
    } else {
        //TODO: should multithread this to download quicker
        let num_systems = space_traders.get_status().await.unwrap().stats.systems;

        info!(
            "{} does not exist - Downloading will take ~{} minute(s) to to fetch {} systems",
            DISTANCESDB_FILE,
            num_systems / 2400, // = 20 per page every 500 milliseconds / 60 min
            num_systems,
        );

        let systems: Arc<Mutex<Vec<schemas::System>>> = Arc::new(Mutex::new(Vec::new()));

        for task_numers in 1..(((num_systems / 20) + 1) / 100) {
            let new_space_traders = SpaceTraders::default().await;
            let systems = systems.clone();
            tokio::task::spawn(async move {
                for page in 1..task_numers {
                    for system in new_space_traders
                        .list_systems(Some(page))
                        .await
                        .unwrap()
                        .data
                        .iter()
                    {
                        systems.lock().await.push(system.clone());
                    }
                }
            });
        }

        info!("Calclulating System Distances");

        let mut all_euclidean_distance: Vec<AllEuclideanDistances> = Vec::new();

        let systems = systems.lock().await;

        for system in systems.iter() {
            // consider using rayon here
            all_euclidean_distance.push(AllEuclideanDistances {
                name: system.symbol.system.clone(),
                x: system.x,
                y: system.y,
                euclidean_distance: euclidean_distance(
                    system,
                    &systems,
                    Some(systems.len().try_into().unwrap()),
                ),
            });
        }

        info!("Writing new distances to {}", DISTANCESDB_FILE);

        serde_json::to_writer(
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
    pub distance: u64,
    pub name: String,
    pub x: i32,
    pub y: i32,
}

fn euclidean_distance(
    current_system: &schemas::System,
    systems: &[schemas::System],
    num_returns: Option<u32>,
) -> Vec<EuclideanDistances> {
    trace!("Euclidean Distance Caluclations");
    let num_systems_to_return = num_returns.unwrap_or(5);

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
                name: system.symbol.system.clone(),
            });
        } else {
            'inner: for (index, system_distance) in closest_systems.iter().enumerate() {
                if distance <= system_distance.distance {
                    closest_systems.insert(
                        index,
                        EuclideanDistances {
                            distance,
                            x,
                            y,
                            name: system.symbol.system.clone(),
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
