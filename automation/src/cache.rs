use spacetraders::{
    enums,
    responses::schemas, // systems
    SpaceTraders,
    WaypointString,
};

use async_recursion::async_recursion;
use chrono::{DateTime, Utc};
use indextree::{Arena, NodeId};
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

const DISTANCESDB_FILE: &str = "distances.cbor";
const GATESDB_FILE: &str = "gates.cbor";

#[derive(Debug, Deserialize, Serialize)]
struct CachedData<T> {
    date: DateTime<Utc>,
    data: T,
}
pub fn cache_data<T: Serialize>(data: T, file_name: &str) {
    ciborium::into_writer(
        &CachedData {
            date: chrono::offset::Utc::now(),
            data,
        },
        &File::create(file_name).unwrap(),
    )
    .unwrap();
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AllEuclideanDistances {
    pub name: String,
    pub x: i32,
    pub y: i32,
    pub euclidean_distance: Vec<EuclideanDistances>,
}

#[async_recursion]
pub async fn build_euclidean_distance(space_traders: &SpaceTraders) -> Vec<AllEuclideanDistances> {
    trace!("Building Euclidean Distances");

    if Path::new(DISTANCESDB_FILE).is_file() {
        let distance_file: Result<
            CachedData<Vec<AllEuclideanDistances>>,
            ciborium::de::Error<std::io::Error>,
        > = ciborium::from_reader(&File::open(DISTANCESDB_FILE).unwrap());
        let distances: CachedData<Vec<AllEuclideanDistances>> = match distance_file {
            Err(_) => {
                info!("removing currupted {}", DISTANCESDB_FILE);

                remove_file(DISTANCESDB_FILE).unwrap();
                return build_euclidean_distance(space_traders).await;
            }
            Ok(data) => {
                info!("{} integrity check good", DISTANCESDB_FILE);

                // if data.date < space_traders.get_status().await.unwrap().reset_date {
                //     info!("{} is outdated", DISTANCESDB_FILE);
                //     remove_file(DISTANCESDB_FILE).unwrap();
                //     return build_euclidean_distance(space_traders).await;
                // }

                data
            }
        };

        distances.data
    } else {
        //TODO: should multithread this to download quicker
        let num_systems = space_traders.get_status().await.unwrap().stats.systems;

        info!(
            "{} does not exist - Downloading {} systems",
            DISTANCESDB_FILE, num_systems,
        );

        let systems: Arc<Mutex<Vec<schemas::System>>> = Arc::new(Mutex::new(Vec::new()));
        let mut systems_handles: Vec<JoinHandle<()>> = Vec::new();

        for task_numers in 1..(((num_systems / 20) + 1) / 100) {
            let new_space_traders = SpaceTraders::default().await;
            let systems = systems.clone();
            systems_handles.push(task::spawn(async move {
                for _ in 1..task_numers {
                    for system in new_space_traders
                        .list_systems(false)
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

        cache_data(all_euclidean_distance.clone(), DISTANCESDB_FILE);

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

pub async fn get_gate_network(
    space_traders: &SpaceTraders,
    symbol: WaypointString,
) -> Option<Arena<schemas::JumpGate>> {
    trace!("Get Gate Network");
    let mut arena = Arena::new();

    let root_gate = space_traders.jump_gate(&symbol).await.ok()?;
    for connected_system in root_gate.data.connected_systems.iter() {
        for waypoint in space_traders
            .list_waypoints(&connected_system.symbol, false)
            .await
            .ok()?
            .data
            .iter()
        {
            recurse_gate_network(space_traders, &mut arena, &waypoint, None).await;
        }
    }
    info!("Finished getting all gates - writing to {}", GATESDB_FILE);
    // cache_data(arena, GATESDB_FILE); // TODO: get this to write to a file

    Some(arena)
}

#[async_recursion]
async fn recurse_gate_network(
    space_traders: &SpaceTraders,
    arena: &mut Arena<schemas::JumpGate>,
    waypoint: &schemas::Waypoint,
    parent: Option<NodeId>,
) {
    // let space_traders = &SpaceTraders::default().await;
    if waypoint.r#type == enums::WaypointType::JumpGate {
        let gate = space_traders.jump_gate(&waypoint.symbol).await.unwrap();

        let new_parent = match parent {
            None => arena.new_node(gate.data.clone()),
            Some(parent) => {
                let new_node = arena.new_node(gate.data.clone());
                parent.append(new_node, arena);

                let node = arena.get(new_node).unwrap();
                arena.get_node_id(node).unwrap()
            }
        };

        for gate_children in gate.data.connected_systems.iter() {
            let waypoints = space_traders
                .list_waypoints(&gate_children.symbol, false)
                .await
                .unwrap();
            for waypoint in waypoints.data.iter() {
                recurse_gate_network(space_traders, arena, &waypoint, Some(new_parent)).await;
            }
        }
        info!("{:?}", arena.count()); // TODO: remove this
        return;
    }
}
