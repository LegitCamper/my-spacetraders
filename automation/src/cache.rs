use spacetraders::{
    responses::schemas, // systems
    SpaceTraders,
};

use async_recursion::async_recursion;
use chrono::{DateTime, Utc};
use log::{info, trace};
use serde::{Deserialize, Serialize};
use std::{
    fs::{remove_file, File},
    path::Path,
    sync::Arc,
};
use tokio::{
    sync::Mutex,
    task::{self, JoinHandle},
};

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

const DISTANCESDB_FILE: &str = "distances.cbor";

#[async_recursion]
pub async fn build_euclidean_distance(space_traders: &SpaceTraders) -> Vec<AllEuclideanDistances> {
    trace!("Building Euclidean Distances");

    if Path::new(DISTANCESDB_FILE).is_file() {
        let distance_file: Result<SerdeEuclideanDistances, ciborium::de::Error<std::io::Error>> =
            ciborium::from_reader(&File::open(DISTANCESDB_FILE).unwrap());
        let distances: SerdeEuclideanDistances = match distance_file {
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
        let mut systems_handles: Vec<JoinHandle<()>> = Vec::new();

        for task_numers in 1..(((num_systems / 20) + 1) / 100) {
            let new_space_traders = SpaceTraders::default().await;
            let systems = systems.clone();
            systems_handles.push(task::spawn(async move {
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
            }));
        }

        for handle in systems_handles.into_iter() {
            handle.await.unwrap();
        }

        info!("Calclulating System Distances");

        let all_euclidean_distance: Arc<Mutex<Vec<AllEuclideanDistances>>> =
            Arc::new(Mutex::new(Vec::new()));
        let mut euclidean_handles: Vec<JoinHandle<()>> = Vec::new();

        let systems_len = systems.lock().await.len();
        let loop_systems = systems.lock().await.clone();

        for system in loop_systems {
            let all_euclidean_distance = all_euclidean_distance.clone();
            let systems = systems.clone();
            euclidean_handles.push(task::spawn(async move {
                let distances = AllEuclideanDistances {
                    name: system.symbol.system.clone(),
                    x: system.x,
                    y: system.y,
                    euclidean_distance: euclidean_distance(
                        &system,
                        systems,
                        Some(systems_len.try_into().unwrap()),
                    )
                    .await,
                };
                all_euclidean_distance.lock().await.push(distances)
            }));
        }

        for handle in euclidean_handles.into_iter() {
            handle.await.unwrap();
        }
        let all_euclidean_distance = all_euclidean_distance.lock().await;

        info!("Writing new distances to {}", DISTANCESDB_FILE);

        ciborium::into_writer(
            &SerdeEuclideanDistances {
                date: chrono::offset::Utc::now(),
                distances: all_euclidean_distance.clone(),
            },
            &File::create(DISTANCESDB_FILE).unwrap(),
        )
        .unwrap();

        all_euclidean_distance.to_vec()
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EuclideanDistances {
    pub distance: u64,
    pub name: String,
    pub x: i32,
    pub y: i32,
}

async fn euclidean_distance(
    current_system: &schemas::System,
    systems: Arc<Mutex<Vec<schemas::System>>>,
    num_returns: Option<u32>,
) -> Vec<EuclideanDistances> {
    trace!("Euclidean Distance Caluclations");
    let num_systems_to_return = num_returns.unwrap_or(5);

    let mut closest_systems: Vec<EuclideanDistances> = Vec::new();
    let (my_x, my_y) = (current_system.x, current_system.y);

    let loop_systems = systems.lock().await.clone();

    for system in loop_systems.iter() {
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
