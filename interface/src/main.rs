use automation::{self, start_ship_handler, ShipHandlerData};
use spacetraders::{self, responses::schemas, SpaceTraders};
mod tui;

use clap::Parser;
use log::{info, trace};
use simple_logger::SimpleLogger;
use std::{collections::HashMap, sync::Arc};
use tokio::{
    sync::Mutex,
    task::{self, JoinHandle},
};

async fn start_automation(
    token: Option<String>,
) -> (
    Arc<Mutex<SpaceTraders>>,
    Arc<Mutex<ShipHandlerData>>,
    JoinHandle<()>,
) {
    trace!("Starting automation");
    let space_traders: Arc<Mutex<SpaceTraders>> = match token {
        Some(token) => Arc::new(Mutex::new(
            spacetraders::SpaceTraders::new(&token, spacetraders::SpaceTradersEnv::Live).await,
        )),
        None => Arc::new(Mutex::new(spacetraders::SpaceTraders::default().await)),
    };

    let credits = space_traders.lock().await.agent().await.data.credits;
    let systems_db = automation::build_system_db(space_traders.clone()).await;
    let ship_handler_data = Arc::new(Mutex::new(ShipHandlerData {
        ships: vec![],
        contracts: HashMap::new(),
        handles: vec![],
        credits,
        systems_db,
    }));

    let ship_handler: task::JoinHandle<()> =
        start_ship_handler(space_traders.clone(), ship_handler_data.clone());

    (
        space_traders.clone(),
        ship_handler_data.clone(),
        ship_handler,
    )
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
            let (space_traders, ship_hander_data, ship_handler_handle) = match args.token {
                None => start_automation(None).await,
                Some(token) => start_automation(Some(token)).await,
            };
            match args.interactive {
                true => tui::start(space_traders, ship_hander_data).unwrap(),
                false => (), // runs in cli/headless mode
            }
            ship_handler_handle.await.unwrap();
        }
        false => match args.interactive {
            true => (),
            false => (),
        },
    }
}
