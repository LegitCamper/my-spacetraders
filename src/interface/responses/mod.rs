use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::enums::*;

#[derive(Deserialize, Debug)]
pub struct Meta {
    // metadata for responses
    total: u32,
    page: u32,
    limit: u32,
}

pub mod agents;
pub mod contracts;
pub mod fleet;
pub mod systems;
