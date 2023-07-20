use automation::{start_ship_handler, ShipHandler};
use spacetraders::{self, SpaceTraders}; // responses::schemas

use clap::Parser;
use log::trace;
use simple_logger::SimpleLogger;
use std::{collections::HashMap, sync::Arc};
use tokio::{
    signal,
    sync::Mutex,
    task::{self, JoinHandle},
};

async fn start_automation(token: Option<String>) -> (Arc<Mutex<ShipHandler>>, JoinHandle<()>) {
    trace!("Starting automation");
    let space_traders: SpaceTraders = match token {
        Some(token) => {
            spacetraders::SpaceTraders::new(&token, spacetraders::SpaceTradersEnv::Live).await
        }
        None => spacetraders::SpaceTraders::default().await,
    };

    let credits = space_traders.agent().await.unwrap().data.credits;
    let euclidean_distances = automation::cache::build_euclidean_distance(&space_traders).await;
    let ship_handler_data = Arc::new(Mutex::new(ShipHandler {
        handles: vec![],
        spacetraders: space_traders,
        ships: HashMap::new(),
        contracts: HashMap::new(),
        surveys: HashMap::new(),
        waypoints: HashMap::new(),
        credits,
        euclidean_distances,
    }));

    let ship_handler: JoinHandle<()> = task::spawn(start_ship_handler(ship_handler_data.clone()));

    (ship_handler_data, ship_handler)
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Token of the agent to signin as
    /// (otherwise generate new agent)
    #[arg(short, long)]
    token: Option<String>,
}

#[tokio::main]
async fn main() {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .with_colors(true)
        .init()
        .unwrap();

    trace!("Starting SpaceTraders cli");

    let args = Args::parse();

    let (ship_hander_data, ship_handler_handle) = match args.token {
        None => start_automation(None).await,
        Some(token) => start_automation(Some(token)).await,
    };

    tokio::select! {
        _ = signal::ctrl_c() => {}
    }

    ship_handler_handle.abort();
    for handle in ship_hander_data.lock().await.handles.iter() {
        handle.abort();
    }
    ship_hander_data.lock().await.spacetraders.task.abort();
    println!("{}", ship_hander_data.lock().await.spacetraders.diagnose());
    println!("Exiting - Bye!");
    std::process::exit(0);
}
