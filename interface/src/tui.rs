use super::{ShipHandlerData, SpaceTraders};

use crossterm::{
    event::{self, Event as CEvent, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::{
    collections::HashMap,
    sync::{mpsc, Arc},
};
use tokio::{
    sync::Mutex,
    task::{self, JoinHandle},
};
#[allow(unused_imports)]
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{
        Block, BorderType, Borders, Cell, List, ListItem, ListState, Paragraph, Row, Table, Tabs,
    },
    Terminal,
};

// handles internal tui events
enum Event<I> {
    Input(I),
    Tick,
}

// stores the state of the tui
#[derive(Copy, Clone, Debug)]
enum MenuItem {
    Home,
    Ships,
}
impl From<MenuItem> for usize {
    fn from(input: MenuItem) -> usize {
        match input {
            MenuItem::Home => 0,
            MenuItem::Ships => 1,
        }
    }
}

pub fn start(
    space_traders: Arc<Mutex<SpaceTraders>>,
    ship_handler_data: Arc<Mutex<ShipHandlerData>>,
) -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode().expect("can run in raw mode");
    Ok(())
}
