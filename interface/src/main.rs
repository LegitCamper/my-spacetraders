use automation::{self, start_ship_handler, ShipHandlerData};
use spacetraders::{self, SpaceTraders}; // responses::schemas
mod tui;

use clap::Parser;
use log::trace;
use simple_logger::SimpleLogger;
use std::{collections::HashMap, sync::Arc};
use tokio::{
    signal,
    sync::Mutex,
    task::{self, JoinHandle},
};

async fn start_automation(token: Option<String>) -> (Arc<Mutex<ShipHandlerData>>, JoinHandle<()>) {
    trace!("Starting automation");
    let space_traders: SpaceTraders = match token {
        Some(token) => {
            spacetraders::SpaceTraders::new(&token, spacetraders::SpaceTradersEnv::Live).await
        }
        None => spacetraders::SpaceTraders::default().await,
    };

    let credits = space_traders.agent().await.data.credits;
    let systems_db = automation::cache::build_system_db(&space_traders).await;
    let euclidean_distances =
        automation::cache::build_euclidean_distance(systems_db, &space_traders).await;
    let ship_handler_data = Arc::new(Mutex::new(ShipHandlerData {
        handles: vec![],
        spacetraders: space_traders,
        ships: HashMap::new(),
        contracts: HashMap::new(),
        credits,
        surveys: Vec::new(),
        euclidean_distances,
    }));

    let ship_handler: task::JoinHandle<()> =
        tokio::task::spawn(start_ship_handler(ship_handler_data.clone()));

    (ship_handler_data.clone(), ship_handler)
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Token of the agent to signin as
    /// (otherwise generate new agent)
    #[arg(short, long)]
    token: Option<String>,

    /// Run in interactive if true
    /// and headless if false
    #[arg(short, long, default_value_t = false)]
    interactive: bool,

    /// Starts automation for the agent
    #[arg(short, long, default_value_t = true)]
    automation: bool,
}

#[tokio::main]
async fn main() {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .with_colors(true)
        .without_timestamps()
        .init()
        .unwrap();
    trace!("Starting SpaceTraders cli");

    let args = Args::parse();

    match args.automation {
        true => {
            let (ship_hander_data, ship_handler_handle) = match args.token {
                None => start_automation(None).await,
                Some(token) => start_automation(Some(token)).await,
            };
            match args.interactive {
                true => tui::start(ship_hander_data.clone()).unwrap(),
                false => (), // runs in cli/headless mode
            }

            tokio::select! {
                _ = signal::ctrl_c() => {}
            }

            ship_handler_handle.abort();
            for handle in ship_hander_data.lock().await.handles.iter() {
                handle.abort();
            }
            ship_hander_data.lock().await.spacetraders.task.abort();
            println!("Exiting - Bye!");
            std::process::exit(0);
        }
        false => match args.interactive {
            true => (),
            false => (),
        },
    }
}
