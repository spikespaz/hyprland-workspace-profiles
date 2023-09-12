pub mod monitors;

use std::collections::HashMap;

use monitors::MonitorSettings;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct ConfigProfile {
    pub profile_name: String,
    pub monitors: HashMap<String, MonitorIdentifier>,
    pub monitor_settings: HashMap<String, MonitorSettings>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct MonitorIdentifier {
    pub make: String,
    pub model: String,
    pub serial: String,
}
