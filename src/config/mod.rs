pub mod monitors;

use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct MonitorIdentifier {
    pub make: String,
    pub model: String,
    pub serial: String,
}
