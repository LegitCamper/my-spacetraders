use automation::{ship_handler, Automation};
use spacetraders::{self, SpaceTraders}; // responses::schemas

use clap::Parser;
use log::trace;
use simple_logger::SimpleLogger;
use std::collections::HashMap;

async fn start_automation(token: Option<String>, email: Option<String>, _username: Option<String>) {
    trace!("Starting automation");
    let st_interface: SpaceTraders = match token {
        Some(token) => {
            spacetraders::SpaceTraders::new(token, email, spacetraders::SpaceTradersEnv::Live)
        }
        None => spacetraders::SpaceTraders::new_random().await,
    };

    let _headquarters = st_interface.agent().await.unwrap().data.headquarters;
    let credits = st_interface.agent().await.unwrap().data.credits;
    // TODO: this should be ran in the background during startup
    // let euclidean_distances = automation::cache::build_euclidean_distance(&st_interface).await;
    // let gate_nodes = automation::cache::get_gate_network(&space_traders, headquarters).await;
    // println!("{gate_nodes:?}");
    let automation_data = Automation {
        handles: HashMap::new(),
        ships: HashMap::new(),
        contracts: HashMap::new(),
        surveys: HashMap::new(),
        waypoints: HashMap::new(),
        credits,
        euclidean_distances: Vec::new(),
    };

    ship_handler(st_interface, automation_data).await
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
        .without_timestamps()
        .with_colors(true)
        .init()
        .unwrap();

    trace!("Starting SpaceTraders Automation");

    let args = Args::parse();
    start_automation(args.token, args.email, args.username).await;
}
