use automation::{start_st_interface, Automation};
use spacetraders::{self, SpaceTraders}; // responses::schemas

use clap::Parser;
use log::trace;
use simple_logger::SimpleLogger;
use std::{collections::HashMap, sync::Arc};
use tokio::{
    signal,
    sync::RwLock,
    task::{self, JoinHandle},
};

async fn start_automation(
    token: Option<String>,
    email: Option<String>,
    _username: Option<String>,
) -> Arc<RwLock<Automation>> {
    trace!("Starting automation");
    let st_interface: Arc<RwLock<SpaceTraders>> = Arc::new(RwLock::new(match token {
        Some(token) => {
            spacetraders::SpaceTraders::new(
                &token,
                email.as_deref(),
                spacetraders::SpaceTradersEnv::Live,
            )
            .await
        }
        None => spacetraders::SpaceTraders::default().await,
    }));

    let headquarters = st_interface
        .read()
        .await
        .agent()
        .await
        .unwrap()
        .data
        .headquarters;
    let credits = st_interface
        .read()
        .await
        .agent()
        .await
        .unwrap()
        .data
        .credits;
    let euclidean_distances = automation::cache::build_euclidean_distance(&st_interface).await;
    // let gate_nodes = automation::cache::get_gate_network(&space_traders, headquarters).await;
    // println!("{gate_nodes:?}");
    let ship_automation = Arc::new(RwLock::new(ShipAutomation {
        handles: vec![],
        ships: HashMap::new(),
        contracts: HashMap::new(),
        surveys: HashMap::new(),
        waypoints: HashMap::new(),
        credits,
        euclidean_distances,
    }));

    start_st_interface(ship_automation.clone(), st_interface.clone());
    ship_automation
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Token of the agent to signin as
    /// (otherwise generate new agent)
    #[arg(short, long)]
    token: Option<String>,
    /// Email to register new agent to
    #[arg(short, long)]
    email: Option<String>,
    /// Email to register new agent to
    #[arg(short, long)]
    username: Option<String>,
}

#[tokio::main]
async fn main() {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .with_colors(true)
        .init()
        .unwrap();

    trace!("Starting SpaceTraders Automation");

    let args = Args::parse();
    let ship_automation = start_automation(args.token, args.email, args.username).await;

    tokio::select! {
        _ = signal::ctrl_c() => {}
    }

    ship_automation.abort();
    for handle in ship_automation.read().await.handles.iter() {
        handle.abort();
    }
    ship_automation.read().await.spacetraders.task.abort();
    println!("{}", ship_automation.read().await.spacetraders.diagnose());
    println!("Exiting - Bye!");
    std::process::exit(0);
}
