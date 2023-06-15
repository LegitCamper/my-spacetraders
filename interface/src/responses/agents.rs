pub use super::schemas;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Agent {
    pub data: schemas::Agent,
}
