use super::ShipHandlerData; //SpaceTraders

use crossterm::terminal::enable_raw_mode;
use std::sync::Arc;
use tokio::sync::Mutex;
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
#[allow(dead_code)]
enum Event<I> {
    Input(I),
    Tick,
}

// stores the state of the tui
#[allow(dead_code)]
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
    _ship_handler_data: Arc<Mutex<ShipHandlerData>>,
) -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode().expect("can run in raw mode");
    Ok(())
}
